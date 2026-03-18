//! Emit layer: ExtensionSetDef → Rust source.

use std::{fs::File, io::Write, path::Path};

use anyhow::Result;

use crate::{model::module::ExtensionSetDef, xml};

pub fn generate_extension_set_file(registry: &xml::Registry, generated_dir: &Path) -> Result<()> {
    let path = generated_dir.join("extensions.rs");
    let mut file = File::create(&path)?;

    writeln!(file, "use crate::{{define_extension_set, vk::*}};\n")?;

    let instance_set =
        crate::model::module::build_extension_set("InstanceExtensionSet", registry, "instance");
    emit_extension_set(&mut file, &instance_set)?;

    let device_set =
        crate::model::module::build_extension_set("DeviceExtensionSet", registry, "device");
    emit_extension_set(&mut file, &device_set)?;

    Ok(())
}

pub fn emit_extension_set(file: &mut impl Write, def: &ExtensionSetDef) -> Result<()> {
    writeln!(file, "define_extension_set!({}, [", def.name)?;
    for entry in &def.entries {
        if entry.provisional {
            writeln!(file, "    #[cfg(feature = \"provisional\")]")?;
        }
        writeln!(
            file,
            "    ({}, {}::EXTENSION_NAME),",
            entry.ident, entry.mod_path
        )?;
    }
    writeln!(file, "]);\n")?;
    Ok(())
}
