use std::{
    borrow::Cow,
    collections::{BTreeMap, HashMap, HashSet},
    io::Write,
};

use heck::{ToShoutySnakeCase, ToSnakeCase};

use crate::xml::Constant;

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

fn generate(xmls: &[&xml::Registry]) {
    let trailing_number = regex::Regex::new(r"(\d+)$").unwrap();

    let sys_output_dir = "crates/kazan-sys/src/generated/vk";
    let output_dir = "crates/kazan/src/generated/vk";

    std::fs::remove_dir_all(sys_output_dir).unwrap();
    std::fs::remove_dir_all(output_dir).unwrap();

    let mut sys_vendor_modules = BTreeMap::new();
    let mut vendor_modules = BTreeMap::new();

    let mut visited_items = HashSet::new();

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
                .chain(required_commands.map(|cmd| cmd.name))
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
                    ("core".to_string(), name.to_ascii_lowercase())
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
                writeln!(sys_file, "use std::ffi::{{c_char, c_int, c_void}};").unwrap();
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
                    let bitmask = ty.bitvalues.map(|b| {
                        xml.bitmasks
                            .iter()
                            .find(|bitmask| bitmask.name == b)
                            .unwrap()
                    });

                    let name = normalize_ty_name(ty.name).replace("FlagBits", "Flags");
                    //let bitwidth = ty.bitwidth.unwrap_or(32);
                    writeln!(sys_file, "bitflags! {{").unwrap();
                    writeln!(sys_file, "    #[repr(transparent)]").unwrap();
                    writeln!(
                        sys_file,
                        "    #[derive(Copy, Clone, PartialEq, Eq, Default)]"
                    )
                    .unwrap();
                    //writeln!(file, "    pub struct {}: u{} {{", name, bitwidth).unwrap();
                    writeln!(
                        sys_file,
                        "    pub struct {}: {} {{",
                        name,
                        ctype_to_rust_type_str(ty.ty)
                    )
                    .unwrap();

                    if let Some(bitmask) = bitmask {
                        let value_prefix = strip_vendor_suffix(bitmask.name)
                            .replace("FlagBits", "")
                            .to_shouty_snake_case();
                        let value_prefix =
                            trailing_number.replace(&value_prefix, "_$1").to_string();

                        for bit in &bitmask.bits {
                            let name = bit
                                .name
                                .strip_prefix(&value_prefix)
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
                            writeln!(sys_file, "        const {} = 1 << {};", name, bit.bitpos)
                                .unwrap();
                        }
                        for value in &bitmask.values {
                            let name = value
                                .name
                                .strip_prefix(&value_prefix)
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

                for ty in commands.clone() {
                    writeln!(
                        sys_file,
                        "pub type PFN_{} = unsafe extern \"system\" fn(",
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
            }

            if commands.clone().next().is_some() {
                vendor_modules
                    .entry(vendor.to_string())
                    .or_insert_with(Vec::new)
                    .push(module_name.clone());

                #[derive(Copy, Clone, Debug)]
                enum CommandType {
                    Instance,
                    Device,
                }

                std::fs::create_dir_all(format!("{}/{}", output_dir, vendor)).unwrap();
                let mut file =
                    std::fs::File::create(format!("{}/{}/{}.rs", output_dir, vendor, module_name))
                        .unwrap();

                // For each handle type, determine if it is a device child
                use std::collections::VecDeque;
                let mut handle_command_types = HashMap::new();
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

                let device_commands = commands.clone().filter(|command| {
                let ty = &command.params.iter().next().unwrap().c_decl.ty;
                matches!(ty, cdecl::CType::Base(base) if matches!(handle_command_types.get(base.name), Some(CommandType::Device)))});

                writeln!(file, "pub struct Device {{").unwrap();
                writeln!(file, "}}").unwrap();

                writeln!(file, "impl Device {{").unwrap();
                for command in device_commands {
                    writeln!(file, "pub fn {}(&self,", normalize_name(command.name)).unwrap();
                    writeln!(file, ");").unwrap();
                }
                writeln!(file, "}}").unwrap();
            }
        }
    }

    std::fs::create_dir_all(sys_output_dir).unwrap();
    let mut sys_mod_file = std::fs::File::create(format!("{}/mod.rs", sys_output_dir)).unwrap();
    let mut mod_file = std::fs::File::create(format!("{}/mod.rs", output_dir)).unwrap();
    //writeln!(mod_file, "pub use core::*;").unwrap();

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
        writeln!(mod_file, "mod {};", vendor).unwrap();
        writeln!(mod_file, "pub use {}::*;", vendor).unwrap();

        std::fs::create_dir_all(format!("{}/{}", output_dir, vendor)).unwrap();
        let mut file = std::fs::File::create(format!("{}/{}/mod.rs", output_dir, vendor)).unwrap();

        for name in names {
            writeln!(file, "mod {};", name).unwrap();
            writeln!(file, "pub use {}::*;", name).unwrap();
        }
    }

    std::process::Command::new("rustfmt")
        .arg(format!("{}/mod.rs", output_dir))
        .output()
        .unwrap();
    std::process::Command::new("rustfmt")
        .arg(format!("{}/mod.rs", sys_output_dir))
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

fn normalize_const_name(name: &str) -> &str {
    name.strip_prefix("VK_").unwrap_or(name)
}

fn normalize_ty_name(name: &str) -> &str {
    //strip_vendor_suffix(name.strip_prefix("Vk").unwrap_or(name))
    name.strip_prefix("Vk").unwrap_or(name)
}

fn ctype_to_rust_type_str(name: &str) -> String {
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
    .replace("FlagBits", "Flags")
}

fn ctype_to_rust_type(ty: &cdecl::CType) -> String {
    match ty {
        cdecl::CType::Base(base) => ctype_to_rust_type_str(base.name),
        cdecl::CType::Ptr {
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
        cdecl::CType::Array { element, len } => {
            let element_ty = ctype_to_rust_type(element.as_ref());
            match len {
                cdecl::CArrayLen::Named(name) => {
                    format!("[{}; {} as usize]", element_ty, normalize_const_name(name))
                }
                cdecl::CArrayLen::Literal(len) => format!("[{}; {}]", element_ty, len),
            }
        }
        cdecl::CType::Func { ret_ty, params, .. } => todo!(),
    }
}
