//! Emit layer: HandleDef → Rust source for handle definitions.

use std::io::Write;

use anyhow::Result;

use crate::{model::handle::HandleDef, xml};

pub fn generate_handles(file: &mut impl Write, handles: &[&xml::Handle]) -> Result<()> {
    for handle in handles {
        let m = crate::model::handle::build_handle(handle);
        emit_handle(file, &m)?;
    }
    writeln!(file)?;
    Ok(())
}

pub fn emit_handle(file: &mut impl Write, h: &HandleDef) -> Result<()> {
    writeln!(
        file,
        "{}!({}, {}, doc = \"<{}>\");",
        h.macro_name, h.name, h.obj_type, h.doc_url
    )?;
    Ok(())
}
