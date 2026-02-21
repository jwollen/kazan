use std::{
    borrow::Cow,
    collections::{BTreeMap, HashMap, HashSet},
    io::Write,
};

use heck::{ToShoutySnakeCase, ToSnakeCase};

use crate::{cdecl::CType, xml::Constant};

mod analysis;
mod cdecl;
mod xml;

fn main() {
    let analysis = analysis::Analysis::new("crates/generator/external/Vulkan-Headers");

    generate(&[analysis.vk_xml(), analysis.video_xml()]);
}

#[derive(Copy, Clone)]
enum FeatureOrExtension<'a> {
    Feature(&'a xml::Feature),
    Extension(&'a xml::Extension),
}

impl<'a> FeatureOrExtension<'a> {
    fn requires(&self) -> &[xml::Require] {
        match self {
            FeatureOrExtension::Feature(feature) => &feature.requires,
            FeatureOrExtension::Extension(extension) => &extension.requires,
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum CommandType {
    Entry,
    Instance,
    Device,
}

fn generate(xmls: &[&xml::Registry]) {
    let trailing_number = regex::Regex::new(r"(\d+)$").unwrap();

    let sys_output_dir = "crates/kazan-sys/src/generated/vk";
    let output_dir = "crates/kazan/src/generated/vk";

    let _ = std::fs::remove_dir_all(sys_output_dir);
    let _ = std::fs::remove_dir_all(output_dir);

    let mut sys_vendor_modules = BTreeMap::new();
    let mut vendor_modules = BTreeMap::new();

    let mut visited_items = HashSet::new();

    // For each handle type, determine if it is a device child
    use std::collections::VecDeque;
    let mut handle_command_types = HashMap::new();
    for xml in xmls {
        let mut pending_handles = xml.handles.iter().collect::<VecDeque<_>>();
        while let Some(handle) = pending_handles.pop_front() {
            if handle.name == "VkInstance" {
                handle_command_types.insert(handle.name, CommandType::Instance);
            } else if handle.name == "VkDevice" {
                handle_command_types.insert(handle.name, CommandType::Device);
            } else {
                let parent = handle.parent.unwrap();
                if let Some(parent_command_type) = handle_command_types.get(parent) {
                    handle_command_types.insert(handle.name, *parent_command_type);
                } else {
                    pending_handles.push_back(handle);
                }
            }
        }
    }

    for xml in xmls {
        let features_and_extensions = xml
            .features
            .iter()
            .map(FeatureOrExtension::Feature)
            .chain(xml.extensions.iter().map(FeatureOrExtension::Extension));

        for feature_or_extension in features_and_extensions {
            let required_types = feature_or_extension
                .requires()
                .iter()
                .map(|r| &r.types)
                .flatten();

            let required_commands = feature_or_extension
                .requires()
                .iter()
                .map(|r| &r.commands)
                .flatten();

            let required_api_constants = feature_or_extension
                .requires()
                .iter()
                .map(|r| &r.constants)
                .flatten()
                .filter_map(|constant| {
                    let global_api_constant = xml
                        .constants
                        .iter()
                        .find(|api_constant| api_constant.name == constant.name);

                    if let Some(global_api_constant) = global_api_constant {
                        Some(global_api_constant.clone())
                    } else if let (Some(ty), Some(value)) = (constant.ty, constant.value) {
                        Some(Constant {
                            name: constant.name,
                            ty,
                            value,
                        })
                    } else {
                        None
                    }
                });

            let new_items = required_types
                .map(|ty| ty.name)
                .chain(required_commands.clone().map(|cmd| cmd.name))
                .chain(required_api_constants.clone().map(|constant| constant.name))
                .filter(|name| visited_items.insert(*name))
                .collect::<HashSet<_>>();

            let commands = xml.commands.iter().filter(|ty| new_items.contains(ty.name));

            let (vendor, module_name) = match feature_or_extension {
                FeatureOrExtension::Feature(feature) => {
                    let name = format!(
                        "{}_{}_{}",
                        feature.version.api, feature.version.major, feature.version.minor
                    );
                    ("features".to_string(), name.to_ascii_lowercase())
                }
                FeatureOrExtension::Extension(extension) => {
                    let (vendor, name) = if extension.name.starts_with("VK_") {
                        extension
                            .name
                            .strip_prefix("VK_")
                            .unwrap()
                            .split_once("_")
                            .unwrap()
                    } else {
                        let name = extension.name.strip_prefix("vulkan_video_").unwrap();
                        ("video", name)
                    };

                    let name = if name.chars().next().unwrap().is_ascii_digit() {
                        format!("_{}", name)
                    } else {
                        name.to_string()
                    };

                    (vendor.to_lowercase(), name)
                }
            };

            if !new_items.is_empty() {
                sys_vendor_modules
                    .entry(vendor.to_string())
                    .or_insert_with(Vec::new)
                    .push(module_name.clone());

                std::fs::create_dir_all(format!("{}/{}", sys_output_dir, vendor)).unwrap();
                let mut sys_file = std::fs::File::create(format!(
                    "{}/{}/{}.rs",
                    sys_output_dir, vendor, module_name
                ))
                .unwrap();

                writeln!(sys_file, "#![allow(non_camel_case_types, unused_imports)]").unwrap();
                writeln!(sys_file, "use core::ffi::{{c_char, c_int, c_void}};").unwrap();
                writeln!(sys_file, "use bitflags::bitflags;").unwrap();
                writeln!(sys_file, "use crate::{{*, vk::*}};").unwrap();

                let constants =
                    required_api_constants.filter(|constant| new_items.contains(constant.name));

                for constant in constants {
                    writeln!(
                        sys_file,
                        "pub const {}: {} = {};",
                        normalize_const_name(constant.name),
                        ctype_to_rust_type_str(constant.ty),
                        convert_c_expr(constant.value),
                    )
                    .unwrap();
                }

                let basetypes = xml
                    .basetypes
                    .iter()
                    .filter(|ty| new_items.contains(ty.name));
                for ty in basetypes {
                    writeln!(
                        sys_file,
                        "pub type {} = {};",
                        normalize_ty_name(ty.name),
                        ctype_to_rust_type_str(ty.ty.unwrap_or("*const c_void"))
                    )
                    .unwrap();
                }

                let handles = xml.handles.iter().filter(|ty| new_items.contains(ty.name));

                for ty in handles {
                    let underlying_ty = match ty.ty {
                        "VK_DEFINE_HANDLE" => "usize",
                        "VK_DEFINE_NON_DISPATCHABLE_HANDLE" => "u64",
                        _ => panic!(),
                    };
                    writeln!(sys_file, "#[repr(C)]").unwrap();
                    writeln!(sys_file, "#[derive(Copy, Clone)]").unwrap();
                    writeln!(
                        sys_file,
                        "pub struct {}({});",
                        normalize_ty_name(ty.name),
                        underlying_ty
                    )
                    .unwrap();
                }

                let type_aliases = xml
                    .enum_aliases
                    .iter()
                    .filter(|alias| xml.enums.iter().find(|ty| ty.name == alias.alias).is_some())
                    .chain(xml.handle_aliases.iter())
                    .chain(xml.struct_aliases.iter())
                    .chain(xml.bitmask_aliases.iter())
                    .filter(|alias| new_items.contains(alias.name));

                for ty in type_aliases {
                    writeln!(
                        sys_file,
                        "pub type {} = {};",
                        normalize_ty_name(ty.name),
                        normalize_ty_name(ty.alias)
                    )
                    .unwrap();
                }

                let command_aliases = xml
                    .command_aliases
                    .iter()
                    .filter(|alias| new_items.contains(alias.name));

                for ty in command_aliases {
                    writeln!(
                        sys_file,
                        "pub type PFN_{} = PFN_{};",
                        normalize_ty_name(ty.name),
                        normalize_ty_name(ty.alias)
                    )
                    .unwrap();
                }

                let structs = xml.structs.iter().filter(|ty| new_items.contains(ty.name));
                for ty in structs {
                    writeln!(sys_file, "#[repr(C)]").unwrap();
                    writeln!(sys_file, "#[derive(Copy, Clone)]").unwrap();
                    writeln!(sys_file, "pub struct {} {{", normalize_ty_name(ty.name)).unwrap();
                    for member in &ty.members {
                        let field_ty = ctype_to_rust_type(&member.c_decl.ty);
                        writeln!(
                            sys_file,
                            "    pub {}: {},",
                            normalize_name(member.c_decl.name),
                            field_ty
                        )
                        .unwrap();
                    }
                    writeln!(sys_file, "}}").unwrap();
                }

                let unions = xml.unions.iter().filter(|ty| new_items.contains(ty.name));
                for ty in unions {
                    writeln!(sys_file, "#[repr(C)]").unwrap();
                    writeln!(sys_file, "#[derive(Copy, Clone)]").unwrap();
                    writeln!(sys_file, "pub union {} {{", normalize_ty_name(ty.name)).unwrap();
                    for member in &ty.members {
                        let field_ty = ctype_to_rust_type(&member.c_decl.ty);
                        writeln!(
                            sys_file,
                            "    pub {}: {},",
                            normalize_name(member.c_decl.name),
                            field_ty
                        )
                        .unwrap();
                    }
                    writeln!(sys_file, "}}").unwrap();
                }

                let enums = xml.enums.iter().filter(|ty| new_items.contains(ty.name));
                for ty in enums {
                    let value_prefix = if ty.name == "VkResult" {
                        "VK".to_string()
                    } else {
                        let value_prefix = strip_vendor_suffix(ty.name).to_shouty_snake_case();
                        trailing_number.replace(&value_prefix, "_$1").to_string()
                    };

                    writeln!(sys_file, "#[repr(transparent)]").unwrap();
                    writeln!(
                        sys_file,
                        "#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]"
                    )
                    .unwrap();
                    writeln!(sys_file, "pub struct {}(i32);", normalize_ty_name(ty.name)).unwrap();

                    writeln!(sys_file, "impl {} {{", normalize_ty_name(ty.name)).unwrap();
                    for value in &ty.values {
                        let name = value
                            .name
                            .strip_prefix(&value_prefix)
                            .unwrap()
                            .strip_prefix("_")
                            .unwrap()
                            .to_uppercase(); // Formats contain lowercase 'x'

                        //let name = strip_vendor_suffix(name);
                        let name = if name
                            .chars()
                            .next()
                            .map(|c| c.is_ascii_digit())
                            .unwrap_or_default()
                        {
                            format!("_{}", &name)
                        } else {
                            name.to_string()
                        };
                        writeln!(
                            sys_file,
                            "    pub const {}: Self = Self({});",
                            name, value.value
                        )
                        .unwrap();
                    }
                    writeln!(sys_file, "}}").unwrap();
                }

                let bitmask_types = xml
                    .bitmask_types
                    .iter()
                    .filter(|ty| new_items.contains(ty.name));
                for ty in bitmask_types {
                    let bitmask = ty.bitvalues.or(ty.requires).map(|b| {
                        xml.bitmasks
                            .iter()
                            .find(|bitmask| bitmask.name == b)
                            .unwrap()
                    });

                    let name = normalize_ty_name(ty.name);

                    writeln!(sys_file, "bitflags! {{").unwrap();
                    writeln!(sys_file, "    #[repr(transparent)]").unwrap();
                    writeln!(
                        sys_file,
                        "    #[derive(Copy, Clone, PartialEq, Eq, Default)]"
                    )
                    .unwrap();
                    let base_type = ctype_to_rust_type_str(ty.ty);
                    writeln!(sys_file, "    pub struct {}: {} {{", name, base_type).unwrap();

                    let value_prefix = bitmask.map(|bitmask| {
                        let value_prefix = strip_vendor_suffix(bitmask.name)
                            .replace("FlagBits", "")
                            .to_shouty_snake_case();
                        trailing_number.replace(&value_prefix, "_$1").to_string()
                    });

                    let bits = bitmask
                        .map(|bitmask| {
                            bitmask
                                .bits
                                .iter()
                                .map(|bit| {
                                    let name = bit
                                        .name
                                        .strip_prefix(value_prefix.as_ref().unwrap())
                                        .unwrap()
                                        .strip_prefix("_")
                                        .unwrap();
                                    //let name = strip_vendor_suffix(name);
                                    //let name = name.strip_suffix("_BIT").unwrap_or(name);
                                    let name = name.replace("_BIT", "");
                                    let name = if name
                                        .chars()
                                        .next()
                                        .map(|c| c.is_ascii_digit())
                                        .unwrap_or_default()
                                    {
                                        format!("_{}", &name)
                                    } else {
                                        name.to_string()
                                    };
                                    (name, bit)
                                })
                                .collect::<Vec<_>>()
                        })
                        .unwrap_or_default();

                    if let Some(bitmask) = bitmask {
                        let bitmask_name = normalize_ty_name(bitmask.name);
                        for (name, _bit) in &bits {
                            writeln!(
                                sys_file,
                                "        const {} = {}::{}.0;",
                                name, bitmask_name, name
                            )
                            .unwrap();
                        }
                        for value in &bitmask.values {
                            let name = value
                                .name
                                .strip_prefix(value_prefix.as_ref().unwrap())
                                .unwrap()
                                .strip_prefix("_")
                                .unwrap();
                            let name = strip_vendor_suffix(name);
                            writeln!(sys_file, "        const {} = {};", name, value.value)
                                .unwrap();
                        }
                    }

                    writeln!(sys_file, "    }}").unwrap();
                    writeln!(sys_file, "}}").unwrap();

                    if let Some(bitmask) = bitmask {
                        let bitmask_name = normalize_ty_name(bitmask.name);
                        writeln!(
                            sys_file,
                            "#[repr(transparent)]
                            #[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
                            pub struct {}(u{});
                            impl {} {{",
                            bitmask_name,
                            bitmask.bitwidth.unwrap_or(32),
                            bitmask_name,
                        )
                        .unwrap();

                        for (name, bit) in &bits {
                            writeln!(sys_file, "pub const {}: Self = Self(1 << {});", name, bit.bitpos).unwrap();
                        }

                        writeln!(sys_file, "}}").unwrap();
                    }
                }

                let funcpointers = xml
                    .funcpointers
                    .iter()
                    .filter(|ty| new_items.contains(ty.name));
                for ty in funcpointers {
                    writeln!(
                        sys_file,
                        "pub type {} = unsafe extern \"system\" fn(",
                        ty.name
                    )
                    .unwrap();
                    for param in &ty.params {
                        writeln!(
                            sys_file,
                            "    {}: {},",
                            normalize_name(param.c_decl.name),
                            ctype_to_rust_type(&param.c_decl.ty)
                        )
                        .unwrap();
                    }
                    if let Some(ref return_type) = ty.return_type {
                        writeln!(sys_file, ") -> {};", ctype_to_rust_type(&return_type)).unwrap();
                    } else {
                        writeln!(sys_file, ");").unwrap();
                    }
                }

                for command in commands.clone() {
                    writeln!(
                        sys_file,
                        "pub type PFN_{} = unsafe extern \"system\" fn(",
                        command.name
                    )
                    .unwrap();
                    for param in &command.params {
                        writeln!(
                            sys_file,
                            "    {}: {},",
                            normalize_name(param.c_decl.name),
                            ctype_to_rust_type(&param.c_decl.ty)
                        )
                        .unwrap();
                    }
                    if let Some(ref return_type) = command.return_type {
                        writeln!(sys_file, ") -> {};", ctype_to_rust_type(&return_type)).unwrap();
                    } else {
                        writeln!(sys_file, ");").unwrap();
                    }
                }
            }

            if required_commands.clone().next().is_some() {
                vendor_modules
                    .entry(vendor.to_string())
                    .or_insert_with(Vec::new)
                    .push(module_name.clone());

                std::fs::create_dir_all(format!("{}/{}", output_dir, vendor)).unwrap();
                let mut file =
                    std::fs::File::create(format!("{}/{}/{}.rs", output_dir, vendor, module_name))
                        .unwrap();

                writeln!(file, "#![allow(unused_imports)]").unwrap();
                writeln!(file, "use core::ffi::{{c_char, c_int, c_void, CStr}};").unwrap();
                writeln!(file, "use core::mem::transmute;").unwrap();
                writeln!(file, "use kazan_sys::{{*, vk::*}};").unwrap();
                writeln!(file, "use crate::*;").unwrap();

                // for cmd in required_commands {
                //     writeln!(file, "// {}", cmd.name).unwrap();
                // }

                let mut generate_commands = |cmd_type: CommandType, fn_type_name: &str| {
                    let command_groups: Vec<_> = feature_or_extension
                        .requires()
                        .iter()
                        .flat_map(|require| {
                            let commands: Vec<_> = require
                                .commands
                                .iter()
                                .map(|req_cmd| {
                                    let alias = xml.command_aliases.iter().find_map(|alias| {
                                        if alias.name == req_cmd.name {
                                            Some(alias.alias)
                                        } else {
                                            None
                                        }
                                    });
                                    let name = alias.unwrap_or(req_cmd.name);
                                    let command =
                                        xml.commands.iter().find(|cmd| cmd.name == name).unwrap();
                                    CommandInfo {
                                        alias: req_cmd.name,
                                        command,
                                        optional: !require.depends.is_empty(),
                                    }
                                })
                                .filter(|cmd| {
                                    let ty = &cmd.command.params.iter().next().unwrap().c_decl.ty;
                                    if let CType::Base(base) = ty {
                                        handle_command_types
                                            .get(base.name)
                                            .copied()
                                            .unwrap_or(CommandType::Entry)
                                            == cmd_type
                                    } else {
                                        cmd_type == CommandType::Entry
                                    }
                                })
                                .collect();

                            if commands.is_empty() {
                                None
                            } else {
                                Some(CommandGroup { require, commands })
                            }
                        })
                        .collect::<Vec<_>>();

                    if command_groups.is_empty() {
                        return;
                    }

                    writeln!(file, "pub struct {} {{", fn_type_name).unwrap();
                    for command_group in &command_groups {
                        for command in &command_group.commands {
                            let name = normalize_command_name(command.alias);
                            let ty = format!("PFN_{}", normalize_ty_name(command.command.name));
                            let ty = if command.optional {
                                format!("Option<{}>", ty)
                            } else {
                                ty
                            };
                            writeln!(file, "{}: {},", name, ty).unwrap();
                        }
                    }
                    writeln!(file, "}}").unwrap();

                    writeln!(file, "impl {} {{", fn_type_name).unwrap();
                    writeln!(file, "pub unsafe fn load(load: impl Fn(&CStr) -> Option<PFN_vkVoidFunction>) -> core::result::Result<Self, LoadingError> {{").unwrap();
                    writeln!(file, "unsafe {{ Ok(Self {{").unwrap();
                    for command_group in &command_groups {
                        for command in &command_group.commands {
                            let name = normalize_command_name(command.alias);
                            if command.optional {
                                writeln!(
                                    file,
                                    "{}: transmute(load(c\"{}\")),",
                                    name, command.alias
                                )
                                .unwrap();
                            } else {
                                writeln!(
                                    file,
                                    "{}: transmute(load(c\"{}\").ok_or(LoadingError)?),",
                                    name, command.alias
                                )
                                .unwrap();
                            }
                        }
                    }
                    writeln!(file, "}}) }} }} }}").unwrap();

                    writeln!(file, "impl {} {{", fn_type_name).unwrap();
                    for command_group in &command_groups {
                        for command in &command_group.commands {
                            write_command_wrapper(&mut file, command, &xml.structs);
                        }
                    }
                    writeln!(file, "}}").unwrap();
                };

                generate_commands(CommandType::Entry, "EntryFn");
                generate_commands(CommandType::Instance, "InstanceFn");
                generate_commands(CommandType::Device, "DeviceFn");
            }
        }
    }

    std::fs::create_dir_all(sys_output_dir).unwrap();
    let mut sys_mod_file = std::fs::File::create(format!("{}/mod.rs", sys_output_dir)).unwrap();
    let mut mod_file = std::fs::File::create(format!("{}/mod.rs", output_dir)).unwrap();

    for (vendor, names) in sys_vendor_modules {
        writeln!(sys_mod_file, "mod {};", vendor).unwrap();
        writeln!(sys_mod_file, "pub use {}::*;", vendor).unwrap();

        std::fs::create_dir_all(format!("{}/{}", sys_output_dir, vendor)).unwrap();
        let mut sys_file =
            std::fs::File::create(format!("{}/{}/mod.rs", sys_output_dir, vendor)).unwrap();

        for name in names {
            writeln!(sys_file, "mod {};", name).unwrap();
            writeln!(sys_file, "pub use {}::*;", name).unwrap();
        }
    }

    for (vendor, names) in vendor_modules {
        writeln!(mod_file, "pub mod {};", vendor).unwrap();
        //writeln!(mod_file, "pub use {}::*;", vendor).unwrap();

        std::fs::create_dir_all(format!("{}/{}", output_dir, vendor)).unwrap();
        let mut file = std::fs::File::create(format!("{}/{}/mod.rs", output_dir, vendor)).unwrap();

        for name in names {
            writeln!(file, "pub mod {};", name).unwrap();
            //writeln!(file, "pub use {}::*;", name).unwrap();
        }
    }

    std::process::Command::new("rustfmt")
        .arg(format!("{}/mod.rs", output_dir))
        .arg("--edition=2024")
        .output()
        .unwrap();
    std::process::Command::new("rustfmt")
        .arg(format!("{}/mod.rs", sys_output_dir))
        .arg("--edition=2024")
        .output()
        .unwrap();
}

fn convert_c_expr<'a>(expr: &'a str) -> Cow<'a, str> {
    let expr = expr
        .strip_prefix('(')
        .and_then(|expr| expr.strip_suffix(')'))
        .unwrap_or(expr);

    let expr = expr
        .strip_suffix('f')
        .or_else(|| expr.strip_suffix('F'))
        .or_else(|| expr.strip_suffix("ULL"))
        .or_else(|| expr.strip_suffix("LL"))
        .or_else(|| expr.strip_suffix('U'))
        .unwrap_or(expr);

    if let Some(expr) = expr.strip_prefix("~") {
        Cow::Owned(format!("!{}", expr))
    } else {
        Cow::Borrowed(expr)
    }
}

fn strip_vendor_suffix(name: &str) -> &str {
    let vendors = [
        "AMD",
        "AMDX",
        "ANDROID",
        "ARM",
        "BRCM",
        "CHROMIUM",
        "EXT",
        "FB",
        "FSL",
        "FUCHSIA",
        "GGP",
        "GOOGLE",
        "HUAWEI",
        "IMG",
        "INTEL",
        "JUICE",
        "KDAB",
        "KHR",
        "KHX",
        "LUNARG",
        "MESA",
        "MSFT",
        "MVK",
        "NN",
        "NV",
        "NVX",
        "NXP",
        "NZXT",
        "OHOS",
        "QCOM",
        "QNX",
        "RASTERGRID",
        "RENDERDOC",
        "SAMSUNG",
        "SEC",
        "TIZEN",
        "VALVE",
        "VIV",
        "VSI",
    ];

    let vendor = vendors
        .iter()
        .find(|&vendor| name.ends_with(vendor))
        .cloned();

    if let Some(vendor) = vendor {
        let name = name.strip_suffix(vendor).unwrap();
        if name.ends_with("_") {
            name.strip_suffix("_").unwrap()
        } else {
            name
        }
    } else {
        name
    }
}

fn normalize_name(name: &str) -> String {
    match name {
        "type" => "ty".to_string(),
        _ => name.to_snake_case(),
    }
}

fn normalize_param_name(name: &str) -> String {
    let name = normalize_name(name);

    name.strip_prefix("pp_")
        .or_else(|| name.strip_prefix("p_"))
        .unwrap_or(name.as_str())
        .to_string()
}

fn normalize_command_name(name: &str) -> String {
    name.strip_prefix("vk").unwrap().to_snake_case()
}

fn normalize_const_name(name: &str) -> &str {
    name.strip_prefix("VK_").unwrap_or(name)
}

fn normalize_ty_name(name: &str) -> &str {
    //strip_vendor_suffix(name.strip_prefix("Vk").unwrap_or(name))
    name.strip_prefix("Vk").unwrap_or(name)
}

fn ctype_to_rust_type_str(name: &str) -> &str {
    match name {
        "int8_t" => "i8",
        "uint8_t" => "u8",
        "int16_t" => "i16",
        "uint16_t" => "u16",
        "int32_t" => "i32",
        "uint32_t" => "u32",
        "int64_t" => "i64",
        "uint64_t" => "u64",
        "size_t" => "usize",
        "float" => "f32",
        "double" => "f64",
        "void" => "c_void",
        "char" => "c_char",
        "int" => "c_int",
        _ => normalize_ty_name(name),
    }
}

// struct StructInfo<'a> {
//     ty: &'a xml::Structure,
// }

// struct MemberInfo<'a> {
//     name: String,
//     ty: String,
//     len: Option<LengthKind<'a>>,
// }

// fn analyze_struct<'a>(
//     ty: &'a xml::Structure,
// ) -> StructInfo<'a> {
//     todo!()
// }

struct CommandGroup<'a> {
    require: &'a xml::Require,
    commands: Vec<CommandInfo<'a>>,
}

struct CommandInfo<'a> {
    command: &'a xml::Command,
    alias: &'a str,
    optional: bool,
}

struct WrapperCommandInfo<'a> {
    // The original xml command
    command: &'a xml::Command,

    // The normalized command name
    name: String,

    // Info about functions that can either output a length or enumerate items
    enumeration_info: Option<EnumerationCommandInfo>,

    wrapper_params: Vec<WrapperParamInfo<'a>>,
    params: Vec<ParamInfo<'a>>,
}

struct EnumerationCommandInfo {
    len_param: usize,
    array_params: Vec<usize>,
}

struct WrapperParamInfo<'a> {
    name: String,
    param: &'a xml::CommandParam,
    ty: String,
}

struct ParamInfo<'a> {
    name: String,
    param: &'a xml::CommandParam,
    ty: String,
    len: Option<LengthKind<'a>>,
    optional: (bool, bool),
}

#[derive(Clone)]
enum LengthKind<'a> {
    NullTerminated,
    Literal(u32),
    Param {
        index: usize,
        param: &'a xml::CommandParam,
    },
    ParamField {
        index: usize,
        param: &'a xml::CommandParam,
        field: &'a xml::StructureMember,
    },
    Unknown(&'static str),
}

impl<'a> LengthKind<'a> {
    fn ty(&self) -> Option<&CType> {
        match self {
            LengthKind::Param { param, .. } => Some(&param.c_decl.ty),
            LengthKind::ParamField { field, .. } => Some(&field.c_decl.ty),
            _ => None,
        }
    }
}

fn get_param_index(command: &xml::Command, param_name: &str) -> Option<usize> {
    command
        .params
        .iter()
        .enumerate()
        .find_map(|(index, other_param)| {
            if other_param.c_decl.name == param_name {
                Some(index)
            } else {
                None
            }
        })
}

fn get_len_kind<'a>(
    command: &'a xml::Command,
    structs: &'a [xml::Structure],
    len: &'static str,
) -> LengthKind<'a> {
    if len == "null-terminated" {
        LengthKind::NullTerminated
    } else if let Ok(len) = len.parse() {
        LengthKind::Literal(len)
    } else if let Some((param_name, field_name)) = len.split_once("->")
        && let Some(index) = get_param_index(command, param_name)
    {
        let param = &command.params[index];
        let param_ty = &param.c_decl.ty;
        let CType::Ptr { pointee, .. } = param_ty else {
            panic!("expected pointer type, got {:?}", param_ty);
        };
        let CType::Base(base) = pointee.as_ref() else {
            panic!("expected base type, got {:?}", pointee);
        };

        let struct_ty = structs
            .iter()
            .find(|ty| ty.name == base.name)
            .unwrap_or_else(|| panic!("failed to find struct {}", base.name));

        let field = struct_ty
            .members
            .iter()
            .find(|field| field.c_decl.name == field_name)
            .unwrap_or_else(|| panic!("failed to find field {}", field_name));

        LengthKind::ParamField {
            index,
            param,
            field,
        }
    } else if let Some(index) = get_param_index(command, len) {
        let param = &command.params[index];
        LengthKind::Param { index, param }
    } else {
        LengthKind::Unknown(len)
    }
}

fn analyze_command<'a>(
    info: &CommandInfo<'a>,
    structs: &'a [xml::Structure],
) -> WrapperCommandInfo<'a> {
    let command = info.command;
    let len_kinds: Vec<_> = command
        .params
        .iter()
        .map(|param| param.len.map(|len| get_len_kind(command, structs, len)))
        .collect();

    let enumeration_len_param = len_kinds.iter().find_map(|len_kind| match len_kind {
        Some(LengthKind::Param { index, param }) => {
            let param_ty = &param.c_decl.ty;
            match param_ty {
                CType::Ptr { is_const, .. } if !*is_const => Some(*index),
                _ => None,
            }
        }
        _ => None,
    });

    let enumeration_info = enumeration_len_param.map(|len_param| {
        let array_params = len_kinds
            .iter()
            .enumerate()
            .filter_map(|(param_index, len_kind)| match len_kind {
                Some(LengthKind::Param { index, .. }) if *index == len_param => Some(param_index),
                _ => None,
            })
            .collect();
        EnumerationCommandInfo {
            len_param,
            array_params,
        }
    });

    let mut wrapper_params = Vec::new();
    let mut params = Vec::new();

    for (param_index, param) in command.params.iter().enumerate() {
        let name = normalize_param_name(param.c_decl.name);

        let optional = (
            param
                .optional
                .get(0)
                .and_then(|s| s.parse().ok())
                .unwrap_or_default(),
            param
                .optional
                .get(1)
                .and_then(|s| s.parse().ok())
                .unwrap_or_default(),
        );

        let is_implicit_length = len_kinds.iter().any(
            |len| matches!(len, Some(LengthKind::Param { index, .. }) if *index == param_index),
        );

        if !is_implicit_length {
            let name = normalize_param_name(param.c_decl.name);
            let wrapper_ty =
                convert_param_type(&param.c_decl.ty, len_kinds[param_index].as_ref(), optional);

            wrapper_params.push(WrapperParamInfo {
                name: name.clone(),
                param,
                ty: wrapper_ty,
            });
        }

        params.push(ParamInfo {
            name,
            param,
            ty: ctype_to_rust_type(&param.c_decl.ty),
            len: len_kinds[param_index].clone(),
            optional,
        });
    }

    let name = normalize_command_name(info.alias);

    WrapperCommandInfo {
        command,
        name,
        enumeration_info,
        wrapper_params,
        params,
    }
}

fn convert_param_type(ty: &CType, len: Option<&LengthKind<'_>>, optional: (bool, bool)) -> String {
    if let Some(len) = len
        && !matches!(len, LengthKind::Literal(1))
    {
        let CType::Ptr {
            pointee, is_const, ..
        } = ty
        else {
            panic!();
        };

        let ty = if matches!(pointee.as_ref(), cdecl::CType::Base(base) if base.name == "char") {
            let pointee = "CStr";
            if *is_const {
                format!("&{}", pointee)
            } else {
                format!("&mut {}", pointee)
            }
        } else {
            let element_ty = ctype_to_rust_type(pointee.as_ref());
            let element_ty = if element_ty == "c_void" {
                "u8"
            } else {
                &element_ty
            };

            if *is_const {
                format!("&[{}]", element_ty)
            } else {
                if matches!(len.ty(), Some(cdecl::CType::Base { .. })) {
                    format!("&mut [{}]", element_ty)
                } else {
                    assert!(matches!(len.ty(), Some(cdecl::CType::Ptr { .. })));
                    return format!("impl ExtendUninit<{}>", element_ty);
                }
            }
        };

        if (optional).0 {
            format!("Option<{}>", ty)
        } else {
            ty
        }
    } else if let CType::Ptr {
        pointee, is_const, ..
    } = ty
    {
        let pointee = ctype_to_rust_type(&pointee);

        let ty = if *is_const {
            format!("&{}", pointee)
        } else {
            format!("&mut {}", pointee)
        };

        if (optional).0 {
            format!("Option<{}>", ty)
        } else {
            ty
        }
    } else {
        ctype_to_rust_type(ty)
    }
}

fn ctype_to_rust_type(ty: &CType) -> String {
    match ty {
        CType::Base(base) => ctype_to_rust_type_str(base.name).to_string(),
        CType::Ptr {
            pointee,
            implicit_for_decay,
            is_const,
            ..
        } => {
            let pointee = ctype_to_rust_type(pointee.as_ref());
            if *is_const {
                format!("*const {}", pointee).to_string()
            } else {
                format!("*mut {}", pointee).to_string()
            }
        }
        CType::Array { element, len } => {
            let element_ty = ctype_to_rust_type(element.as_ref());
            match len {
                cdecl::CArrayLen::Named(name) => {
                    format!("[{}; {} as usize]", element_ty, normalize_const_name(name))
                }
                cdecl::CArrayLen::Literal(len) => format!("[{}; {}]", element_ty, len),
            }
        }
        CType::Func { ret_ty, params, .. } => todo!(),
    }
}

fn write_command_wrapper(
    file: &mut impl std::io::Write,
    info: &CommandInfo<'_>,
    structs: &[xml::Structure],
) {
    let command = info.command;
    let wrapper = analyze_command(info, structs);

    writeln!(file, "pub unsafe fn {}(&self,", wrapper.name).unwrap();

    for param in &wrapper.wrapper_params {
        writeln!(file, "{}: {},", param.name, param.ty).unwrap();
    }

    if let Some(ref return_type) = command.return_type {
        writeln!(file, ") -> {} {{", ctype_to_rust_type(&return_type)).unwrap();
    } else {
        writeln!(file, ") {{").unwrap();
    }

    writeln!(file, "unsafe {{").unwrap();

    if let Some(enumeration_info) = &wrapper.enumeration_info {
        let has_result = if let Some(ret_type) = &command.return_type {
            if let CType::Base(base) = ret_type {
                base.name == "VkResult"
            } else {
                false
            }
        } else {
            false
        };

        let extend_fn_name = if has_result {
            match enumeration_info.array_params.len() {
                1 => "try_extend_uninit",
                2 => "try_extend_uninit2",
                _ => todo!(),
            }
        } else {
            match enumeration_info.array_params.len() {
                1 => "extend_uninit",
                _ => todo!(),
            }
        };

        let len_param = &wrapper.params[enumeration_info.len_param];
        let array_params = enumeration_info
            .array_params
            .iter()
            .map(|i| wrapper.params[*i].name.as_str())
            .collect::<Vec<_>>()
            .join(", ");

        writeln!(
            file,
            "{}({}, |{}, {}| {{",
            extend_fn_name, array_params, len_param.name, array_params
        )
        .unwrap();
        write_fn_call(file, &wrapper, info.optional);
        writeln!(file, "}})").unwrap();
    } else {
        write_fn_call(file, &wrapper, info.optional);
    }

    writeln!(file, "}}").unwrap();
    writeln!(file, "}}").unwrap();
}

fn write_fn_call(file: &mut impl std::io::Write, wrapper: &WrapperCommandInfo, optional: bool) {
    if optional {
        writeln!(file, "(self.{}.unwrap())(", wrapper.name).unwrap();
    } else {
        writeln!(file, "(self.{})(", wrapper.name).unwrap();
    }

    for (param_index, param) in wrapper.params.iter().enumerate() {
        let array_param = wrapper.params.iter().find(
            |p| matches!(p.len, Some(LengthKind::Param { index, .. }) if index == param_index),
        );

        let ty = &param.param.c_decl.ty;

        if let Some(array_param) = array_param {
            if matches!(ty, CType::Ptr { .. }) {
                writeln!(file, "{},", param.name).unwrap();
            } else {
                writeln!(file, "{}.len().try_into().unwrap(),", array_param.name).unwrap();
            }
        } else {
            if let Some(enumeration_info) = &wrapper.enumeration_info
                && enumeration_info.array_params.contains(&param_index)
            {
                writeln!(file, "{} as _,", param.name).unwrap();
            } else if let Some(len) = &param.len
                && !matches!(len, LengthKind::Literal(1))
            {
                let CType::Ptr { is_const, .. } = ty else {
                    panic!();
                };

                if param.optional.0 {
                    if *is_const {
                        writeln!(file, "{}.to_raw_ptr(),", param.name).unwrap();
                    } else {
                        writeln!(file, "{}.to_raw_mut_ptr(),", param.name).unwrap();
                    }
                } else {
                    if *is_const {
                        writeln!(file, "{}.as_ptr() as _,", param.name).unwrap();
                    } else {
                        writeln!(file, "{}.as_mut_ptr() as _,", param.name).unwrap();
                    }
                }
            } else {
                if let CType::Ptr { is_const, .. } = ty
                    && param.optional.0
                {
                    if *is_const {
                        writeln!(file, "{}.to_raw_ptr(),", param.name).unwrap();
                    } else {
                        if wrapper.enumeration_info.is_some() {
                            writeln!(
                                file,
                                "todo!(\"output parameters in enumeration commands\"),"
                            )
                            .unwrap();
                        } else {
                            writeln!(file, "{}.to_raw_mut_ptr(),", param.name).unwrap();
                        }
                    }
                } else {
                    writeln!(file, "{},", param.name).unwrap();
                }
            }
        }
    }
    writeln!(file, ")").unwrap();
}
