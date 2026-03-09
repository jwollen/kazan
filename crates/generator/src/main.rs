use std::{
    collections::BTreeMap,
    fs::{self, File},
    io::Write,
};

use anyhow::Result;
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

fn main() -> Result<()> {
    let analysis = analysis::Analysis::new("crates/generator/external/Vulkan-Headers");

    generate(&analysis)
}

struct ModuleEntry {
    name: String,
    provisional: bool,
}

fn generate(analysis: &analysis::Analysis) -> Result<()> {
    let registry = analysis.registry();

    let generated_dir = "crates/kazan/src/generated";
    let output_dir = &format!("{}/vk", generated_dir);

    let _ = fs::remove_dir_all(output_dir);

    external::generate_external_type_file(analysis, generated_dir)?;

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
        )?;
    }

    fs::create_dir_all(output_dir)?;
    let mut mod_file = File::create(format!("{}/mod.rs", output_dir))?;

    for (vendor, entries) in &vendor_modules {
        // Vendor-tagged extensions go under `vk/{vendor}/mod.rs` with a combined `defs` re-export module.
        // Core/unvendored modules are emitted directly into `vk/mod.rs`.
        if let Some(vendor) = vendor {
            writeln!(mod_file, "pub mod {};", vendor)?;
            writeln!(mod_file, "pub use {}::defs::*;", vendor)?;

            fs::create_dir_all(format!("{}/{}", output_dir, vendor))?;
            let mut file = File::create(format!("{}/{}/mod.rs", output_dir, vendor))?;

            for entry in entries {
                write!(file, "pub mod {};\n", entry.name)?;
            }

            writeln!(file, "pub(super) mod defs {{")?;
            writeln!(file, "use super::*;")?;
            for entry in entries {
                let cfg = if entry.provisional {
                    "#[cfg(feature = \"provisional\")]\n"
                } else {
                    ""
                };
                write!(file, "{}pub use {}::defs::*;\n", cfg, entry.name)?;
            }
            writeln!(file, "}}")?;
        } else {
            for entry in entries {
                writeln!(mod_file, "pub mod {};", entry.name)?;
                writeln!(mod_file, "pub use {}::defs::*;", entry.name)?;
            }
        }
    }

    std::process::Command::new("rustfmt")
        .arg(format!("{}/mod.rs", output_dir))
        .arg("--edition=2024")
        .output()?;

    module::generate_extension_set_file(registry, generated_dir)?;
    Ok(())
}

fn generate_module(
    analysis: &Analysis,
    output_dir: &str,
    vendor_modules: &mut BTreeMap<Option<String>, Vec<ModuleEntry>>,
    module_index: usize,
    module: &Module<'_>,
) -> Result<()> {
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

    fs::create_dir_all(&vendor_path)?;
    let mut file = File::create(format!("{}/{}.rs", &vendor_path, module_name))?;

    if provisional {
        writeln!(file, "#![cfg(feature = \"provisional\")]")?;
    }

    if let Module::Extension(extension) = module {
        writeln!(
            file,
            "//! <https://registry.khronos.org/vulkan/specs/latest/man/html/{}.html>",
            extension.name
        )?;
    }

    writeln!(
        file,
        "#![allow(unused_imports)]
            use core::ffi::{{c_char, c_int, c_void, CStr}};
            use core::mem::transmute;
            use core::ptr;
            use crate::{{*, vk::*, vk::Result as VkResult}};
            "
    )?;

    if let Module::Extension(extension) = module {
        writeln!(
            file,
            "pub const EXTENSION_NAME: &CStr = c\"{}\";\n",
            extension.name
        )?;
    }

    writeln!(file, "pub(super) mod defs {{")?;

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
        )?;

        constants::generate_api_constants(&mut file, &owned, required_api_constants)?;

        constants::generate_basetypes(&mut file, analysis, &owned)?;

        handle::generate_handles(&mut file, analysis, &owned)?;

        constants::generate_type_aliases(&mut file, analysis, &owned)?;

        structs::generate_structs(&mut file, analysis, &owned)?;

        structs::generate_unions(&mut file, analysis, &owned)?;

        enums::generate_enum_types(&mut file, analysis, &owned)?;

        enums::generate_bitmask_types(&mut file, analysis, &owned)?;

        command::generate_funcpointers(&mut file, analysis, &owned)?;

        command::generate_functions(&mut file, analysis, new_commands.clone())?;
    }
    writeln!(file, "}}\n")?;

    if requires
        .iter()
        .flat_map(|r| &r.commands)
        .clone()
        .next()
        .is_some()
    {
        command::generate_commands(&mut file, analysis, &requires)?;
    }
    Ok(())
}

pub(crate) fn doc_url(name: &str) -> String {
    format!("https://registry.khronos.org/vulkan/specs/latest/man/html/{name}.html")
}

/// Write a doc comment linking to the Vulkan spec for the given Vk-prefixed name.
pub(crate) fn write_doc_link(file: &mut impl std::io::Write, name: &str) -> Result<()> {
    let url = doc_url(name);
    writeln!(file, "/// <{url}>")?;
    Ok(())
}

/// Convert a C struct member / field name to a Rust-style snake_case identifier.
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

fn ctype_to_rust_type(analysis: &Analysis, ty: &CType, lifetime: Option<&str>) -> String {
    match ty {
        CType::Base(base) => ctype_rust::type_name_with_lifetime(analysis, base.name, lifetime),
        CType::Ptr {
            pointee, is_const, ..
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

/// Classification of a Vulkan `len` attribute (e.g. `"null-terminated"`, `"2"`, `"dataSize"`).
#[derive(Clone, Debug)]
enum LengthKind<'a> {
    NullTerminated,
    Literal(u32),
    /// Length given by another parameter in the same command/struct.
    Param {
        /// Index of the length parameter in the parameter list.
        index: usize,
        c_decl: &'a cdecl::CDecl<'static>,
    },
    /// Length given by a field of a struct pointed to by another parameter (e.g. `pCreateInfo->count`).
    ParamField {
        field: &'a xml::StructureMember,
    },
    /// Unrecognized length expression — treated as no length info.
    Unknown,
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

/// Classify a `len` attribute string (e.g. `"null-terminated"`, `"2"`, `"pCreateInfo->count"`)
/// into a `LengthKind`.
fn get_len_kind<'a>(
    analysis: &'a Analysis,
    params: &'a [impl Param],
    len: &'static str,
) -> LengthKind<'a> {
    if len == "null-terminated" {
        LengthKind::NullTerminated
    } else if let Ok(len) = len.parse() {
        LengthKind::Literal(len)
    // Length via struct field dereference (e.g. "pAllocateInfo->commandBufferCount"):
    // resolve the struct the parameter points to, then find the named field.
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

        LengthKind::ParamField { field }
    } else if let Some(index) = get_param_index(params, len) {
        let param = &params[index];
        LengthKind::Param {
            index,
            c_decl: param.c_decl(),
        }
    } else {
        LengthKind::Unknown
    }
}

/// Convert a C parameter name to Rust style, stripping `p_`/`pp_` pointer prefixes.
pub fn normalize_param_name(name: &str) -> String {
    let name = normalize_name(name);

    name.strip_prefix("pp_")
        .or_else(|| name.strip_prefix("p_"))
        .unwrap_or(name.as_str())
        .to_string()
}

/// Like `normalize_param_name`, but for struct setter parameters:
/// `pp_`-prefixed names get a `_ptrs` suffix (e.g. `ppGeometries` → `geometries_ptrs`).
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
