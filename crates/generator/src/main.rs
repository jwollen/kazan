use std::{
    borrow::Cow,
    collections::{BTreeMap, HashMap, HashSet},
    fs::{self, File},
    io::Write,
};

use heck::{ToShoutySnakeCase, ToSnakeCase};
use itertools::Itertools;

use crate::{
    analysis::Analysis,
    cdecl::CType,
    command::{CommandGroup, CommandInfo, normalize_command_name, write_command_wrapper},
    structs::write_struct,
    xml::Constant,
};

mod analysis;
mod cdecl;
mod command;
mod structs;
mod xml;

fn main() {
    let analysis = analysis::Analysis::new("crates/generator/external/Vulkan-Headers");

    generate(&analysis);
}

#[derive(Clone)]
enum VersionOrExtension<'a> {
    Version(VersionInfo<'a>),
    Extension(&'a xml::Extension),
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum CommandType {
    Entry,
    Instance,
    Device,
}

#[derive(Clone)]
struct VersionInfo<'a> {
    major: u32,
    minor: u32,
    features: Vec<&'a xml::Feature>,
}

fn generate(analysis: &analysis::Analysis) {
    let registry = analysis.registry();
    let trailing_number = regex::Regex::new(r"(\d+)$").unwrap();

    let sys_output_dir = "crates/kazan-sys/src/generated/vk";
    let output_dir = "crates/kazan/src/generated/vk";

    let _ = fs::remove_dir_all(sys_output_dir);
    let _ = fs::remove_dir_all(output_dir);

    let mut sys_vendor_modules = BTreeMap::new();
    let mut vendor_modules = BTreeMap::new();

    let mut visited_items = HashSet::new();

    // For each handle type, determine if it is a device child
    use std::collections::VecDeque;
    let mut handle_command_types = HashMap::new();
    let mut pending_handles = registry.handles.iter().collect::<VecDeque<_>>();
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

    let versions = registry
        .features
        .iter()
        .chunk_by(|feature| (feature.version.major, feature.version.minor))
        .into_iter()
        .map(|(version, features)| VersionInfo {
            major: version.0,
            minor: version.1,
            features: features.into_iter().collect(),
        })
        .collect::<Vec<_>>();

    let version_or_extension = versions.into_iter().map(VersionOrExtension::Version).chain(
        registry
            .extensions
            .iter()
            .map(VersionOrExtension::Extension),
    );

    let mut req_enum_variants = BTreeMap::new();
    let mut req_bitspos = BTreeMap::new();
    let mut req_enum_aliases = BTreeMap::new();
    let mut req_enum_values = BTreeMap::new();

    for version_or_extension in version_or_extension.clone() {
        let requires: Vec<_> = match version_or_extension {
            VersionOrExtension::Version(ref version) => version
                .features
                .iter()
                .flat_map(|feature| feature.requires.iter())
                .collect(),
            VersionOrExtension::Extension(extension) => extension.requires.iter().collect(),
        };

        let ext_number = match version_or_extension {
            VersionOrExtension::Version(_) => None,
            VersionOrExtension::Extension(extension) => extension.number,
        };

        for req in requires {
            for variant in &req.enum_variants {
                let variants = req_enum_variants
                    .entry(variant.extends)
                    .or_insert_with(BTreeMap::new);

                let ext_number = variant.extnumber.or(ext_number).unwrap() as i32;
                let value = 1_000_000_000i32 + (ext_number - 1) * 1000 + variant.offset as i32;
                let value = if variant.negative { -value } else { value };
                variants.insert(variant.name, value);
            }
            for bitpos in &req.bitpositions {
                let variants = req_bitspos
                    .entry(bitpos.extends)
                    .or_insert_with(BTreeMap::new);
                variants.insert(bitpos.name, bitpos.bitpos);
            }
            for alias in &req.enum_aliases {
                if let Some(extends) = alias.extends {
                    let aliases = req_enum_aliases
                        .entry(extends)
                        .or_insert_with(BTreeMap::new);
                    aliases.insert(alias.name, alias.alias);
                }
            }
            for value in &req.enum_values {
                let values = req_enum_values
                    .entry(value.extends)
                    .or_insert_with(BTreeMap::new);
                values.insert(value.name, value.value);
            }
        }
    }

    for version_or_extension in version_or_extension {
        let requires: Vec<_> = match version_or_extension {
            VersionOrExtension::Version(ref version) => version
                .features
                .iter()
                .flat_map(|feature| feature.requires.iter())
                .collect(),
            VersionOrExtension::Extension(extension) => extension.requires.iter().collect(),
        };

        let required_types = requires.iter().map(|r| &r.types).flatten();

        let required_commands = requires.iter().map(|r| &r.commands).flatten();

        let required_api_constants =
            requires
                .iter()
                .map(|r| &r.constants)
                .flatten()
                .filter_map(|constant| {
                    let global_api_constant = registry
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

        let new_commands = registry
            .commands
            .iter()
            .filter(|cmd| new_items.contains(cmd.name));

        let (vendor, module_name) = match version_or_extension {
            VersionOrExtension::Version(version) => {
                let name = format!("vk{}_{}", version.major, version.minor);
                (None, name.to_ascii_lowercase())
            }
            VersionOrExtension::Extension(extension) => {
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

                (Some(vendor.to_lowercase()), name)
            }
        };

        if !new_items.is_empty() {
            sys_vendor_modules
                .entry(vendor.clone())
                .or_insert_with(Vec::new)
                .push(module_name.clone());

            let vendor_path = match vendor {
                Some(ref vendor) => format!("{}/{}", sys_output_dir, vendor),
                None => sys_output_dir.to_string(),
            };
            fs::create_dir_all(&vendor_path).unwrap();
            let mut sys_file =
                File::create(format!("{}/{}.rs", &vendor_path, module_name)).unwrap();

            writeln!(sys_file, "#![allow(non_camel_case_types, unused_imports)]").unwrap();
            writeln!(sys_file, "use core::ffi::{{c_char, c_int, c_void}};").unwrap();
            writeln!(sys_file, "use core::marker::PhantomData;").unwrap();
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

            let basetypes = registry
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

            let handles = registry
                .handles
                .iter()
                .filter(|ty| new_items.contains(ty.name));

            for ty in handles {
                let underlying_ty = match ty.ty {
                    "VK_DEFINE_HANDLE" => "usize",
                    "VK_DEFINE_NON_DISPATCHABLE_HANDLE" => "u64",
                    _ => panic!(),
                };
                writeln!(sys_file, "#[repr(C)]").unwrap();
                writeln!(sys_file, "#[derive(Copy, Clone, Default)]").unwrap();
                writeln!(
                    sys_file,
                    "pub struct {}({});",
                    normalize_ty_name(ty.name),
                    underlying_ty
                )
                .unwrap();
            }

            let type_aliases = registry
                .enum_aliases
                .iter()
                .clone()
                .filter(|alias| {
                    registry
                        .enums
                        .iter()
                        .find(|ty| ty.name == alias.alias)
                        .is_some()
                })
                .chain(registry.handle_aliases.iter())
                .chain(registry.struct_aliases.iter())
                .chain(registry.bitmask_aliases.iter())
                .filter(|alias| new_items.contains(alias.name));

            for ty in type_aliases {
                writeln!(
                    sys_file,
                    "pub type {} = {};",
                    type_name_with_lifetime(analysis, ty.name, Some("a")),
                    type_name_with_lifetime(analysis, ty.alias, Some("a"))
                )
                .unwrap();
            }

            let command_aliases = registry
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

            let new_structs = registry
                .structs
                .iter()
                .filter(|ty| new_items.contains(ty.name));
            for ty in new_structs {
                write_struct(&mut sys_file, analysis, ty);
            }

            let unions = registry
                .unions
                .iter()
                .filter(|ty| new_items.contains(ty.name));
            for ty in unions {
                let name = normalize_ty_name(ty.name);
                let type_info = analysis.get_base_type_info(ty.name).unwrap();
                writeln!(
                    sys_file,
                    "#[repr(C)]
                    #[derive(Copy, Clone)]
                    pub union {}{} {{",
                    name,
                    if type_info.lifetime_param { "<'a>" } else { "" }
                )
                .unwrap();
                for member in &ty.members {
                    let field_ty = ctype_to_rust_type(analysis, &member.c_decl.ty, Some("a"));
                    writeln!(
                        sys_file,
                        "    pub {}: {},",
                        normalize_name(member.c_decl.name),
                        field_ty
                    )
                    .unwrap();
                }
                if type_info.lifetime_param {
                    writeln!(sys_file, "pub _marker: PhantomData<&'a ()>,",).unwrap();
                }
                writeln!(sys_file, "}}").unwrap();
                writeln!(
                    sys_file,
                    "impl Default for {}{} {{
                        fn default() -> Self {{
                            unsafe {{ core::mem::zeroed() }}
                        }}
                    }}",
                    name,
                    if type_info.lifetime_param { "<'_>" } else { "" }
                )
                .unwrap();
            }

            let enums = registry
                .enums
                .iter()
                .filter(|ty| new_items.contains(ty.name));
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
                    "#[derive(Copy, Clone, Default,PartialEq, Eq, PartialOrd, Ord, Hash)]"
                )
                .unwrap();
                writeln!(sys_file, "pub struct {}(i32);", normalize_ty_name(ty.name)).unwrap();

                let normalize_variant_name = |name: &str| {
                    let name = name
                        .strip_prefix(&value_prefix)
                        .unwrap()
                        .strip_prefix("_")
                        .unwrap()
                        .to_uppercase(); // Formats contain lowercase 'x'

                    //let name = strip_vendor_suffix(name);
                    if name
                        .chars()
                        .next()
                        .map(|c| c.is_ascii_digit())
                        .unwrap_or_default()
                    {
                        format!("_{}", &name)
                    } else {
                        name.to_string()
                    }
                };

                let variants = {
                    let bits = ty
                        .values
                        .iter()
                        .map(|bit| (normalize_variant_name(bit.name), bit.value.to_string()));

                    let req_enum = req_enum_variants.get(ty.name);
                    let req_variants = req_enum.iter().flat_map(|bits| {
                        bits.iter().map(|(name, variant)| {
                            (normalize_variant_name(name), variant.to_string())
                        })
                    });

                    let mut bits = bits.chain(req_variants).collect::<Vec<_>>();
                    //bits.sort_by_key(|(_, value)| value);
                    bits
                };

                writeln!(sys_file, "impl {} {{", normalize_ty_name(ty.name)).unwrap();
                for (name, value) in &variants {
                    writeln!(sys_file, "pub const {}: Self = Self({});", name, value).unwrap();
                }

                if let Some(values) = req_enum_values.get(ty.name) {
                    for (name, value) in values {
                        let name = normalize_variant_name(name);
                        writeln!(sys_file, "pub const {}: Self = Self({});", name, value).unwrap();
                    }
                }

                if let Some(aliases) = req_enum_aliases.get(ty.name) {
                    for (name, alias) in aliases {
                        let name = normalize_variant_name(name);
                        let alias = normalize_variant_name(alias);
                        writeln!(sys_file, "pub const {}: Self = Self::{};", name, alias).unwrap();
                    }
                }

                writeln!(sys_file, "}}").unwrap();
            }

            let bitmask_types = registry
                .bitmask_types
                .iter()
                .clone()
                .filter(|ty| new_items.contains(ty.name));
            for ty in bitmask_types {
                let bitmask = ty.bitvalues.or(ty.requires).map(|b| {
                    registry
                        .bitmasks
                        .iter()
                        .find(|bitmask| bitmask.name == b)
                        .unwrap()
                });

                let name = normalize_ty_name(ty.name);

                writeln!(sys_file, "bitflags! {{").unwrap();
                writeln!(sys_file, "    #[repr(transparent)]").unwrap();
                writeln!(
                    sys_file,
                    "    #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]"
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

                let normalize_bit_name = |name: &str| {
                    let name = name
                        .strip_prefix(value_prefix.as_ref().unwrap())
                        .unwrap_or(name.strip_prefix("VK").unwrap()) // Some variants have non-standard names
                        .strip_prefix("_")
                        .unwrap();
                    //let name = strip_vendor_suffix(name);
                    //let name = name.strip_suffix("_BIT").unwrap_or(name);
                    let name = name.replace("_BIT", "");

                    if name
                        .chars()
                        .next()
                        .map(|c| c.is_ascii_digit())
                        .unwrap_or_default()
                    {
                        format!("_{}", &name)
                    } else {
                        name.to_string()
                    }
                };

                let bits = if let Some(bitmask) = bitmask {
                    let bits = bitmask
                        .bits
                        .iter()
                        .map(|bit| (normalize_bit_name(bit.name), bit.bitpos));

                    let req_bitmask = req_bitspos.get(bitmask.name);
                    let req_bits = req_bitmask.iter().flat_map(|bits| {
                        bits.iter()
                            .map(|(name, bitpos)| (normalize_bit_name(name), *bitpos))
                    });

                    let mut bits = bits.chain(req_bits).collect::<Vec<_>>();
                    bits.sort_by_key(|(_, bit)| *bit);
                    bits
                } else {
                    Vec::new()
                };

                if let Some(bitmask) = bitmask {
                    let bitmask_name = normalize_ty_name(bitmask.name);

                    // Filter out duplicates. These can happen because we are removing the "_BIT" suffix,
                    // but some bitmask variants miss this suffix and got later corrected through aliases.
                    let mut visited_bits = HashSet::new();

                    for (name, _bit) in &bits {
                        if !visited_bits.insert(name.clone()) {
                            continue;
                        }
                        writeln!(
                            sys_file,
                            "        const {} = {}::{}.0;",
                            name, bitmask_name, name
                        )
                        .unwrap();
                    }

                    for alias in &bitmask.aliases {
                        let name = normalize_bit_name(alias.name);
                        let alias = normalize_bit_name(alias.alias);

                        if !visited_bits.insert(name.clone()) {
                            continue;
                        }

                        writeln!(sys_file, "        const {} = Self::{}.bits();", name, alias)
                            .unwrap();
                    }

                    if let Some(aliases) = req_enum_aliases.get(bitmask.name) {
                        for (name, alias) in aliases {
                            let name = normalize_bit_name(name);
                            let alias = normalize_bit_name(alias);

                            if !visited_bits.insert(name.clone()) {
                                continue;
                            }

                            writeln!(sys_file, "        const {} = Self::{}.bits();", name, alias)
                                .unwrap();
                        }
                    }

                    for value in &bitmask.values {
                        let name = value
                            .name
                            .strip_prefix(value_prefix.as_ref().unwrap())
                            .unwrap()
                            .strip_prefix("_")
                            .unwrap();
                        let name = strip_vendor_suffix(name);
                        writeln!(sys_file, "        const {} = {};", name, value.value).unwrap();
                    }

                    if let Some(values) = req_enum_values.get(bitmask.name) {
                        for (name, value) in values {
                            let name = normalize_bit_name(name);
                            writeln!(sys_file, "        const {} = {};", name, value).unwrap();
                        }
                    }
                }

                writeln!(sys_file, "    }}").unwrap();
                writeln!(sys_file, "}}").unwrap();

                if let Some(bitmask) = bitmask {
                    let bitmask_name = normalize_ty_name(bitmask.name);
                    writeln!(
                        sys_file,
                        "#[repr(transparent)]
                            #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
                            pub struct {}(u{});
                            impl {} {{",
                        bitmask_name,
                        bitmask.bitwidth.unwrap_or(32),
                        bitmask_name,
                    )
                    .unwrap();

                    // Filter out duplicates. These can happen because we are removing the "_BIT" suffix,
                    // but some bitmask variants miss this suffix and got later corrected through aliases.
                    let mut visited_bits = HashSet::new();

                    for (name, bit) in &bits {
                        if !visited_bits.insert(name.clone()) {
                            continue;
                        }
                        writeln!(sys_file, "pub const {}: Self = Self(1 << {});", name, bit)
                            .unwrap();
                    }

                    if let Some(aliases) = req_enum_aliases.get(bitmask.name) {
                        for (name, alias) in aliases {
                            let name = normalize_bit_name(name);
                            let alias = normalize_bit_name(alias);

                            // Only generate aliases for bits, not values
                            if !bits.iter().any(|(name, _)| name.as_str() == alias) {
                                continue;
                            }

                            if !visited_bits.insert(name.clone()) {
                                continue;
                            }

                            writeln!(sys_file, "pub const {}: Self = Self::{};", name, alias)
                                .unwrap();
                        }
                    }

                    writeln!(sys_file, "}}").unwrap();
                }
            }

            let funcpointers = registry
                .funcpointers
                .iter()
                .clone()
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
                        ctype_to_rust_type(analysis, &param.c_decl.ty, None)
                    )
                    .unwrap();
                }
                if let Some(ref return_type) = ty.return_type {
                    writeln!(
                        sys_file,
                        ") -> {};",
                        ctype_to_rust_type(analysis, &return_type, None)
                    )
                    .unwrap();
                } else {
                    writeln!(sys_file, ");").unwrap();
                }
            }

            for command in new_commands.clone() {
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
                        ctype_to_rust_type(analysis, &param.c_decl.ty, None)
                    )
                    .unwrap();
                }
                if let Some(ref return_type) = command.return_type {
                    writeln!(
                        sys_file,
                        ") -> {};",
                        ctype_to_rust_type(analysis, &return_type, None)
                    )
                    .unwrap();
                } else {
                    writeln!(sys_file, ");").unwrap();
                }
            }
        }

        if required_commands.clone().next().is_some() {
            vendor_modules
                .entry(vendor.clone())
                .or_insert_with(Vec::new)
                .push(module_name.clone());

            let vendor_path = match vendor {
                Some(vendor) => format!("{}/{}", output_dir, vendor),
                None => output_dir.to_string(),
            };
            fs::create_dir_all(&vendor_path).unwrap();
            let mut file = File::create(format!("{}/{}.rs", &vendor_path, module_name)).unwrap();

            writeln!(file, "#![allow(unused_imports)]").unwrap();
            writeln!(file, "use core::ffi::{{c_char, c_int, c_void, CStr}};").unwrap();
            writeln!(file, "use core::mem::transmute;").unwrap();
            writeln!(file, "use kazan_sys::{{*, vk::*, vk::Result as VkResult}};").unwrap();
            writeln!(file, "use crate::*;").unwrap();

            let mut generate_commands = |cmd_type: CommandType, fn_type_name: &str| {
                let command_groups: Vec<_> = requires
                    .iter()
                    .flat_map(|require| {
                        let commands: Vec<_> = require
                            .commands
                            .iter()
                            .map(|req_cmd| {
                                let alias = registry.command_aliases.iter().find_map(|alias| {
                                    if alias.name == req_cmd.name {
                                        Some(alias.alias)
                                    } else {
                                        None
                                    }
                                });
                                let name = alias.unwrap_or(req_cmd.name);
                                let command = registry
                                    .commands
                                    .iter()
                                    .find(|cmd| cmd.name == name)
                                    .unwrap();
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
                            writeln!(file, "{}: transmute(load(c\"{}\")),", name, command.alias)
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
                        write_command_wrapper(&mut file, analysis, command);
                    }
                }
                writeln!(file, "}}").unwrap();
            };

            generate_commands(CommandType::Entry, "EntryFn");
            generate_commands(CommandType::Instance, "InstanceFn");
            generate_commands(CommandType::Device, "DeviceFn");
        }
    }

    fs::create_dir_all(sys_output_dir).unwrap();
    let mut sys_mod_file = File::create(format!("{}/mod.rs", sys_output_dir)).unwrap();
    let mut mod_file = File::create(format!("{}/mod.rs", output_dir)).unwrap();

    for (vendor, names) in &sys_vendor_modules {
        if let Some(vendor) = vendor {
            writeln!(sys_mod_file, "mod {};", vendor).unwrap();
            writeln!(sys_mod_file, "pub use {}::*;", vendor).unwrap();

            fs::create_dir_all(format!("{}/{}", sys_output_dir, vendor)).unwrap();
            let mut sys_file =
                File::create(format!("{}/{}/mod.rs", sys_output_dir, vendor)).unwrap();

            for name in names {
                writeln!(sys_file, "mod {};", name).unwrap();
                writeln!(sys_file, "pub use {}::*;", name).unwrap();
            }
        } else {
            for name in names {
                writeln!(sys_mod_file, "mod {};", name).unwrap();
                writeln!(sys_mod_file, "pub use {}::*;", name).unwrap();
            }
        }
    }

    for (vendor, names) in &vendor_modules {
        if let Some(vendor) = vendor {
            writeln!(mod_file, "pub mod {};", vendor).unwrap();

            fs::create_dir_all(format!("{}/{}", output_dir, vendor)).unwrap();
            let mut file = File::create(format!("{}/{}/mod.rs", output_dir, vendor)).unwrap();

            for name in names {
                writeln!(file, "pub mod {};", name).unwrap();
            }
        } else {
            for name in names {
                writeln!(mod_file, "pub mod {};", name).unwrap();
            }
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

fn normalize_const_name(name: &str) -> &str {
    name.strip_prefix("VK_").unwrap_or(name)
}

fn normalize_ty_name(name: &str) -> &str {
    //strip_vendor_suffix(name.strip_prefix("Vk").unwrap_or(name))
    name.strip_prefix("Vk").unwrap_or(name)
}

fn type_name_with_lifetime(analysis: &Analysis, name: &str, lifetime: Option<&str>) -> String {
    let type_info = analysis.get_base_type_info(name).unwrap();
    let name = ctype_to_rust_type_str(name);
    if type_info.lifetime_param {
        format!("{}<'{}>", name, lifetime.unwrap_or("_"))
    } else {
        name.to_string()
    }
}

// TODO: Replace occurances with type_name_with_lifetime
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

fn ctype_to_rust_type(analysis: &Analysis, ty: &CType, lifetime: Option<&str>) -> String {
    match ty {
        CType::Base(base) => type_name_with_lifetime(analysis, base.name, lifetime),
        CType::Ptr {
            pointee,
            implicit_for_decay,
            is_const,
            ..
        } => {
            let pointee = ctype_to_rust_type(analysis, pointee.as_ref(), lifetime);
            if *is_const {
                format!("*const {}", pointee).to_string()
            } else {
                format!("*mut {}", pointee).to_string()
            }
        }
        CType::Array { element, len } => {
            let element_ty = ctype_to_rust_type(analysis, element.as_ref(), lifetime);
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

#[derive(Clone)]
enum LengthKind<'a> {
    NullTerminated,
    Literal(u32),
    Param {
        index: usize,
        c_decl: &'a cdecl::CDecl<'static>,
    },
    ParamField {
        index: usize,
        field: &'a xml::StructureMember,
    },
    Unknown(&'static str),
}

trait Param {
    fn c_decl(&self) -> &cdecl::CDecl<'static>;
}

impl Param for xml::CommandParam {
    fn c_decl(&self) -> &cdecl::CDecl<'static> {
        &self.c_decl
    }
}

impl Param for xml::StructureMember {
    fn c_decl(&self) -> &cdecl::CDecl<'static> {
        &self.c_decl
    }
}

fn get_param_index(params: &[impl Param], param_name: &str) -> Option<usize> {
    params.iter().enumerate().find_map(|(index, other_param)| {
        if other_param.c_decl().name == param_name {
            Some(index)
        } else {
            None
        }
    })
}

fn get_len_kind<'a>(
    analysis: &'a Analysis,
    params: &'a [impl Param],
    len: &'static str,
) -> LengthKind<'a> {
    if len == "null-terminated" {
        LengthKind::NullTerminated
    } else if let Ok(len) = len.parse() {
        LengthKind::Literal(len)
    } else if let Some((param_name, field_name)) = len.split_once("->")
        && let Some(index) = get_param_index(params, param_name)
    {
        let param = &params[index];
        let param_ty = &param.c_decl().ty;
        let CType::Ptr { pointee, .. } = param_ty else {
            panic!("expected pointer type, got {:?}", param_ty);
        };
        let CType::Base(base) = pointee.as_ref() else {
            panic!("expected base type, got {:?}", pointee);
        };

        let struct_ty = analysis
            .registry()
            .structs
            .iter()
            .find(|ty| ty.name == base.name)
            .unwrap_or_else(|| panic!("failed to find struct {}", base.name));

        let field = struct_ty
            .members
            .iter()
            .find(|field| field.c_decl.name == field_name)
            .unwrap_or_else(|| panic!("failed to find field {}", field_name));

        LengthKind::ParamField { index, field }
    } else if let Some(index) = get_param_index(params, len) {
        let param = &params[index];
        LengthKind::Param {
            index,
            c_decl: param.c_decl(),
        }
    } else {
        LengthKind::Unknown(len)
    }
}
