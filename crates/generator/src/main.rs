use std::{
    collections::BTreeMap,
    fs::{self, File},
    io::Write,
};

use heck::ToSnakeCase;

use crate::{
    analysis::Analysis,
    cdecl::CType,
    module::{Module, ModuleName},
    xml::Constant,
};

mod analysis;
mod cdecl;
mod command;
mod constants;
mod ctype_rust;
mod enums;
mod external;
mod handle;
mod module;
mod overrides;
mod structs;
mod xml;

fn main() {
    let analysis = analysis::Analysis::new("crates/generator/external/Vulkan-Headers");

    generate(&analysis);
}

struct ModuleEntry {
    name: String,
    provisional: bool,
}

fn generate(analysis: &analysis::Analysis) {
    let registry = analysis.registry();

    let generated_dir = "crates/kazan/src/generated";
    let output_dir = &format!("{}/vk", generated_dir);

    let _ = fs::remove_dir_all(output_dir);

    external::generate_external_type_file(analysis, generated_dir);

    let mut vendor_modules: BTreeMap<Option<String>, Vec<ModuleEntry>> = BTreeMap::new();

    // Build ownership map: for each item, determine which module should define it.
    // Prefer the module whose vendor tag matches the item's vendor suffix.
    let modules: Vec<_> = Module::from_registry(registry).collect();

    for (module_index, module) in modules.iter().enumerate() {
        generate_module(
            analysis,
            output_dir,
            &mut vendor_modules,
            module_index,
            module,
        );
    }

    fs::create_dir_all(output_dir).unwrap();
    let mut mod_file = File::create(format!("{}/mod.rs", output_dir)).unwrap();

    for (vendor, entries) in &vendor_modules {
        if let Some(vendor) = vendor {
            writeln!(mod_file, "pub mod {};", vendor).unwrap();
            writeln!(mod_file, "pub use {}::defs::*;", vendor).unwrap();

            fs::create_dir_all(format!("{}/{}", output_dir, vendor)).unwrap();
            let mut file = File::create(format!("{}/{}/mod.rs", output_dir, vendor)).unwrap();

            for entry in entries {
                let cfg = if entry.provisional {
                    "#[cfg(feature = \"provisional\")]\n"
                } else {
                    ""
                };
                write!(file, "pub mod {};\n", entry.name).unwrap();
            }

            writeln!(file, "pub(super) mod defs {{").unwrap();
            writeln!(file, "use super::*;").unwrap();
            for entry in entries {
                let cfg = if entry.provisional {
                    "#[cfg(feature = \"provisional\")]\n"
                } else {
                    ""
                };
                write!(file, "{}pub use {}::defs::*;\n", cfg, entry.name).unwrap();
            }
            writeln!(file, "}}").unwrap();
        } else {
            for entry in entries {
                writeln!(mod_file, "pub mod {};", entry.name).unwrap();
                writeln!(mod_file, "pub use {}::defs::*;", entry.name).unwrap();
            }
        }
    }

    std::process::Command::new("rustfmt")
        .arg(format!("{}/mod.rs", output_dir))
        .arg("--edition=2024")
        .output()
        .unwrap();

    module::generate_extension_set_file(registry, generated_dir);
}

fn generate_module(
    analysis: &Analysis,
    output_dir: &str,
    vendor_modules: &mut BTreeMap<Option<String>, Vec<ModuleEntry>>,
    module_index: usize,
    module: &Module<'_>,
) {
    let registry = analysis.registry();
    let requires: Vec<_> = module.requires();
    let owned = analysis.module_items(module_index);

    let required_api_constants = requires
        .iter()
        .flat_map(|r| &r.constants)
        .filter(|c| owned.contains(c.name))
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

    let new_commands = registry
        .commands
        .iter()
        .filter(|cmd| owned.contains(cmd.name));

    let ModuleName {
        vendor,
        name: module_name,
    } = module.name();

    let provisional = matches!(module, Module::Extension(ext) if ext.provisional);
    vendor_modules
        .entry(vendor.clone())
        .or_insert_with(Vec::new)
        .push(ModuleEntry {
            name: module_name.clone(),
            provisional,
        });

    let vendor_path = match vendor {
        Some(ref vendor) => format!("{}/{}", output_dir, vendor),
        None => output_dir.to_string(),
    };

    fs::create_dir_all(&vendor_path).unwrap();
    let mut file = File::create(format!("{}/{}.rs", &vendor_path, module_name)).unwrap();

    if provisional {
        writeln!(file, "#![cfg(feature = \"provisional\")]").unwrap();
    }

    if let Module::Extension(extension) = module {
        writeln!(
            file,
            "//! <https://registry.khronos.org/vulkan/specs/latest/man/html/{}.html>",
            extension.name
        )
        .unwrap();
    }

    writeln!(
        file,
        "#![allow(unused_imports)]
            use core::ffi::{{c_char, c_int, c_void, CStr}};
            use core::mem::transmute;
            use core::ptr;
            use crate::{{*, vk::*, vk::Result as VkResult}};
            "
    )
    .unwrap();

    if let Module::Extension(extension) = module {
        writeln!(
            file,
            "pub const EXTENSION_NAME: &CStr = c\"{}\";\n",
            extension.name
        )
        .unwrap();
    }

    writeln!(file, "pub(super) mod defs {{").unwrap();

    if !owned.is_empty() {
        writeln!(
            file,
            "#![allow(non_camel_case_types, unused_imports)]
                use core::ffi::{{c_char, c_int, c_void, CStr}};
                use core::fmt;
                use core::marker::PhantomData;
                use core::ptr;
                use crate::{{*, vk::*}};
                "
        )
        .unwrap();

        constants::generate_api_constants(&mut file, &owned, required_api_constants);

        constants::generate_basetypes(&mut file, analysis, &owned);

        handle::generate_handles(&mut file, analysis, &owned);

        constants::generate_type_aliases(&mut file, analysis, &owned);

        structs::generate_structs(&mut file, analysis, &owned);

        structs::generate_unions(&mut file, analysis, &owned);

        enums::generate_enum_types(&mut file, analysis, &owned);

        enums::generate_bitmask_types(&mut file, analysis, &owned);

        command::generate_funcpointers(&mut file, analysis, &owned);

        command::generate_functions(&mut file, analysis, new_commands.clone());
    }
    writeln!(file, "}}\n").unwrap();

    if requires
        .iter()
        .flat_map(|r| &r.commands)
        .clone()
        .next()
        .is_some()
    {
        command::generate_commands(&mut file, analysis, &requires);
    }
}

pub(crate) fn doc_url(name: &str) -> String {
    format!("https://registry.khronos.org/vulkan/specs/latest/man/html/{name}.html")
}

/// Write a doc comment linking to the Vulkan spec for the given Vk-prefixed name.
pub(crate) fn write_doc_link(file: &mut impl std::io::Write, name: &str) {
    let url = doc_url(name);
    writeln!(file, "/// <{url}>").unwrap();
}

fn normalize_name(name: &str) -> String {
    match name {
        "type" => "ty".to_string(),
        _ => name.to_snake_case(),
    }
}

pub(crate) fn normalize_const_name(name: &str) -> &str {
    name.strip_prefix("VK_").unwrap_or(name)
}

pub(crate) fn normalize_ty_name(name: &str) -> &str {
    if name == "VkResult" {
        "vk::Result"
    } else {
        name.strip_prefix("Vk").unwrap_or(name)
    }
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
pub(crate) fn ctype_to_rust_type_str(name: &str) -> &str {
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
        "isize_t" => "isize",
        "float" => "f32",
        "double" => "f64",
        "void" => "c_void",
        "char" => "c_char",
        "int" => "c_int",
        "unsigned int" => "c_uint",
        "unsigned long" => "c_ulong",
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
        CType::Func { .. } => todo!(),
    }
}

#[derive(Clone, Debug)]
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

pub fn normalize_param_name(name: &str) -> String {
    let name = normalize_name(name);

    name.strip_prefix("pp_")
        .or_else(|| name.strip_prefix("p_"))
        .unwrap_or(name.as_str())
        .to_string()
}

pub fn normalize_setter_param_name(name: &str) -> String {
    let name = normalize_name(name);

    let name = if name.starts_with("pp_") {
        format!("{}_ptrs", name)
    } else {
        name
    };

    name.strip_prefix("pp_")
        .or_else(|| name.strip_prefix("p_"))
        .unwrap_or(name.as_str())
        .to_string()
}

pub fn normalize_command_name(name: &str) -> String {
    name.strip_prefix("vk").unwrap().to_snake_case()
}
