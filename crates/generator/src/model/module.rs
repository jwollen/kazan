//! Model and builder for extension set definitions.

use crate::{module::ModuleName, xml};

/// Model for a single entry in an extension set macro invocation.
#[derive(Debug, Clone)]
pub struct ExtensionSetEntry {
    pub ident: String,
    pub mod_path: String,
    pub provisional: bool,
}

/// Model for a complete extension set definition.
#[derive(Debug, Clone)]
pub struct ExtensionSetDef {
    pub name: String,
    pub entries: Vec<ExtensionSetEntry>,
}

// ── Builder ─────────────────────────────────────────────────────────────────

pub fn build_extension_set(name: &str, registry: &xml::Registry, ty: &str) -> ExtensionSetDef {
    let entries = registry
        .extensions
        .iter()
        .filter(|ext| ext.ty == Some(ty))
        .map(|ext| {
            let mod_name = get_extension_name(ext);
            let (mod_path, ident) = match &mod_name.vendor {
                Some(vendor) => (
                    format!("{vendor}::{}", mod_name.name),
                    format!("{vendor}_{}", mod_name.name),
                ),
                None => (mod_name.name.clone(), mod_name.name.clone()),
            };
            ExtensionSetEntry {
                ident,
                mod_path,
                provisional: ext.provisional,
            }
        })
        .collect();

    ExtensionSetDef {
        name: name.to_string(),
        entries,
    }
}

fn get_extension_name(extension: &xml::Extension) -> ModuleName {
    if extension.name.starts_with("VK_") {
        let (vendor, name) = extension
            .name
            .strip_prefix("VK_")
            .unwrap()
            .split_once("_")
            .unwrap();

        let name = if name.chars().next().unwrap().is_ascii_digit() {
            format!("_{name}")
        } else {
            name.to_string()
        };

        ModuleName {
            vendor: Some(vendor.to_lowercase()),
            name,
        }
    } else {
        let name = extension.name.strip_prefix("vulkan_video_").unwrap();
        let name = if name.chars().next().unwrap().is_ascii_digit() {
            format!("_{name}")
        } else {
            name.to_string()
        };
        ModuleName {
            vendor: Some("video".to_string()),
            name,
        }
    }
}
