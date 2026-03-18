//! Emit layer: ExternalTypeDef → Rust source for external.rs.

use std::{
    fs::{self, File},
    io::Write,
    path::Path,
};

use anyhow::Result;

use crate::{analysis::Analysis, model::external::ExternalTypeDef};

pub fn generate_external_type_file(analysis: &Analysis, generated_dir: &Path) -> Result<()> {
    fs::create_dir_all(generated_dir)?;
    let mut file = File::create(generated_dir.join("external.rs"))?;
    let models = crate::model::external::build_external_types(analysis);
    emit_external_type_file(&mut file, &models)
}

pub fn emit_external_type_file(file: &mut impl Write, types: &[ExternalTypeDef]) -> Result<()> {
    writeln!(
        file,
        "#![allow(non_camel_case_types)]
use core::ffi::{{c_int, c_uint, c_ulong, c_void}};
"
    )?;

    for t in types {
        writeln!(file, "pub type {} = {};", t.name, t.rust_type)?;
    }
    Ok(())
}
