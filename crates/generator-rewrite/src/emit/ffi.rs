//! Emit layer: FfiModuleDef → Rust source for FFI module.

use std::io::Write;

use anyhow::Result;

use crate::{analysis::Analysis, model::ffi::FfiModuleDef};

pub fn generate_ffi_module(
    file: &mut impl Write,
    analysis: &Analysis,
    items: &crate::analysis::ModuleItems<'_>,
) -> Result<()> {
    let model = crate::model::ffi::build_ffi_module(analysis, items);
    emit_ffi_module(file, &model)
}

pub fn emit_ffi_module(file: &mut impl Write, def: &FfiModuleDef) -> Result<()> {
    writeln!(
        file,
        "#[cfg(feature = \"ffi\")]
        pub(super) mod ffi {{
        #![allow(non_camel_case_types)]
        use super::defs::*;
        "
    )?;

    for alias in &def.aliases {
        writeln!(file, "pub type {} = {};", alias.c_name, alias.rhs)?;
    }

    for imp in &def.lifetime_impls {
        writeln!(
            file,
            "impl {}<'_> {{
                #[inline]
                pub unsafe fn drop_lifetime_for_ffi(&self) -> &{} {{
                    unsafe {{ core::mem::transmute(self) }}
                }}
            }}",
            imp.qualified_name, imp.c_name
        )?;
    }

    writeln!(file, "}}\n")?;
    Ok(())
}
