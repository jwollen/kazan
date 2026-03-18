use std::{
    collections::BTreeMap,
    fs::{self, File},
    io::Write,
    path::Path,
};

use anyhow::Result;
use heck::ToSnakeCase;

use crate::{
    analysis::Analysis,
    module::{Module, ModuleName},
};

pub mod analysis;
pub mod build;
pub mod cdecl;
pub mod ctype;
mod emit;
pub mod external;
pub mod model;
pub mod module;
pub mod overrides;
pub mod xml;

struct ModuleEntry {
    name: String,
    provisional: bool,
    has_defs: bool,
    has_ffi: bool,
}

pub fn generate(analysis: &analysis::Analysis, root: &Path) -> Result<()> {
    let registry = analysis.registry();

    let generated_dir = root.join("crates/kazan/src/generated");
    let output_dir = generated_dir.join("vk");

    let _ = fs::remove_dir_all(&output_dir);

    let mut vendor_modules: BTreeMap<Option<String>, Vec<ModuleEntry>> = BTreeMap::new();
    let modules: Vec<_> = Module::from_registry(registry).collect();

    for (module_index, module) in modules.iter().enumerate() {
        generate_module(
            analysis,
            &output_dir,
            &mut vendor_modules,
            module_index,
            module,
        )?;
    }

    fs::create_dir_all(&output_dir)?;
    let mut mod_file = File::create(output_dir.join("mod.rs"))?;

    for (vendor, entries) in &vendor_modules {
        // Vendor-tagged extensions go under `vk/{vendor}/mod.rs` with a combined `defs` re-export module.
        // Core/unvendored modules are emitted directly into `vk/mod.rs`.
        if let Some(vendor) = vendor {
            let has_any_defs = entries.iter().any(|e| e.has_defs);

            writeln!(mod_file, "pub mod {vendor};")?;
            if has_any_defs {
                writeln!(mod_file, "pub use {vendor}::defs::*;")?;
            }

            fs::create_dir_all(output_dir.join(vendor))?;
            let mut file = File::create(output_dir.join(vendor).join("mod.rs"))?;

            for entry in entries {
                writeln!(file, "pub mod {};", entry.name)?;
            }

            if has_any_defs {
                writeln!(file, "pub(super) mod defs {{")?;
                writeln!(file, "use super::*;")?;
                for entry in entries.iter().filter(|e| e.has_defs) {
                    let cfg = if entry.provisional {
                        "#[cfg(feature = \"provisional\")]\n"
                    } else {
                        ""
                    };
                    writeln!(file, "{cfg}pub use {}::defs::*;", entry.name)?;
                }
                writeln!(file, "}}")?;

                let has_any_ffi = entries.iter().any(|e| e.has_ffi);
                if has_any_ffi {
                    writeln!(file, "#[cfg(feature = \"ffi\")]")?;
                    writeln!(file, "pub(super) mod ffi {{")?;
                    for entry in entries.iter().filter(|e| e.has_ffi) {
                        let cfg = if entry.provisional {
                            "#[cfg(feature = \"provisional\")]\n"
                        } else {
                            ""
                        };
                        writeln!(file, "{cfg}pub use super::{}::ffi::*;", entry.name)?;
                    }
                    writeln!(file, "}}")?;
                }
            }
        } else {
            for entry in entries {
                writeln!(mod_file, "pub mod {};", entry.name)?;
                if entry.has_defs {
                    writeln!(mod_file, "pub use {}::defs::*;", entry.name)?;
                }
            }
        }
    }

    // Generate the top-level ffi re-export module inside vk/.
    let has_any_ffi = vendor_modules.values().flatten().any(|e| e.has_ffi);
    if has_any_ffi {
        writeln!(mod_file, "#[cfg(feature = \"ffi\")]")?;
        writeln!(mod_file, "#[allow(non_camel_case_types)]")?;
        writeln!(mod_file, "pub mod ffi {{")?;
        for (vendor, entries) in &vendor_modules {
            if let Some(vendor) = vendor {
                if entries.iter().any(|e| e.has_ffi) {
                    writeln!(mod_file, "pub use super::{vendor}::ffi::*;")?;
                }
            } else {
                for entry in entries.iter().filter(|e| e.has_ffi) {
                    writeln!(mod_file, "pub use super::{}::ffi::*;", entry.name)?;
                }
            }
        }
        writeln!(mod_file, "}}")?;
    }

    emit::external::generate_external_type_file(analysis, &generated_dir)?;

    emit::module::generate_extension_set_file(registry, &generated_dir)?;

    std::process::Command::new("cargo")
        .arg("fmt")
        .arg("--")
        .arg(root.join("crates/kazan/src/generated.rs"))
        .output()?;

    Ok(())
}

fn generate_module(
    analysis: &Analysis,
    output_dir: &Path,
    vendor_modules: &mut BTreeMap<Option<String>, Vec<ModuleEntry>>,
    module_index: usize,
    module: &Module<'_>,
) -> Result<()> {
    let items = analysis.module_items(module_index);

    let ModuleName {
        vendor,
        name: module_name,
    } = module.name();

    let provisional = matches!(module, Module::Extension(ext) if ext.provisional);
    let has_defs = !items.is_empty();
    let has_ffi = has_defs && items.has_ffi_types();
    vendor_modules
        .entry(vendor.clone())
        .or_default()
        .push(ModuleEntry {
            name: module_name.clone(),
            provisional,
            has_defs,
            has_ffi,
        });

    let vendor_path = match vendor {
        Some(ref vendor) => output_dir.join(vendor),
        None => output_dir.to_path_buf(),
    };

    fs::create_dir_all(&vendor_path)?;
    let mut file = File::create(vendor_path.join(format!("{module_name}.rs")))?;

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
            "pub const EXTENSION_NAME: &CStr = c\"{0}\";\n",
            extension.name
        )?;
    }

    if let Module::Version(version) = module {
        writeln!(
            file,
            "pub const API_VERSION: ApiVersion = ApiVersion::new(0, {}, {}, 0);\n",
            version.major, version.minor
        )?;
    }

    if !items.is_empty() {
        writeln!(file, "pub(super) mod defs {{")?;
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

        emit::generate_api_constants(&mut file, &items.api_constants)?;

        emit::generate_basetypes(&mut file, &items.basetypes)?;

        emit::handle::generate_handles(&mut file, &items.handles)?;

        emit::generate_type_aliases(&mut file, analysis, &items.type_aliases)?;

        emit::func_pointer::generate_command_aliases(&mut file, &items.command_aliases)?;

        emit::structure::generate_structs(&mut file, analysis, &items.structs)?;

        emit::r#union::generate_unions(&mut file, analysis, &items.unions)?;

        emit::enumeration::generate_enums(&mut file, analysis, &items.enums)?;

        emit::enumeration::generate_bitmasks(&mut file, analysis, &items.bitmask_types)?;

        emit::func_pointer::generate_func_pointer_typedefs(
            &mut file,
            analysis,
            &items.funcpointers,
        )?;
        emit::func_pointer::generate_command_func_pointers(&mut file, analysis, &items.commands)?;

        writeln!(file, "}}\n")?;

        if items.has_ffi_types() {
            emit::ffi::generate_ffi_module(&mut file, analysis, items)?;
        }
    }

    let requires: Vec<_> = module.requires();
    if requires.iter().flat_map(|r| &r.commands).next().is_some() {
        emit::command::generate_commands(&mut file, analysis, &requires)?;
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
        format!("{name}_ptrs")
    } else {
        name
    };

    name.strip_prefix("pp_")
        .or_else(|| name.strip_prefix("p_"))
        .unwrap_or(name.as_str())
        .to_string()
}

pub fn normalize_command_name(name: &str, registry: &xml::Registry) -> String {
    let without_prefix = name.strip_prefix("vk").unwrap();
    let snake = without_prefix.to_snake_case();
    if let Some(tag) = registry.vendor_suffix(without_prefix) {
        let suffix = format!("_{}", tag.to_lowercase());
        snake.strip_suffix(&suffix).unwrap_or(&snake).to_string()
    } else {
        snake
    }
}
