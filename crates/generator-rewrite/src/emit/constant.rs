//! Emit layer: model → Rust source for constants, basetypes, type aliases.

use std::io::Write;

use anyhow::Result;

use crate::{
    analysis::Analysis,
    model::constant::{ApiConstantDef, BasetypeDef, TypeAliasDef},
    write_doc_link, xml,
};

pub fn generate_api_constants(
    file: &mut impl Write,
    api_constants: &[xml::Constant],
) -> Result<()> {
    for constant in api_constants {
        let m = crate::model::constant::build_api_constant(constant);
        emit_api_constant(file, &m)?;
    }
    writeln!(file)?;
    Ok(())
}

pub fn generate_basetypes(file: &mut impl Write, basetypes: &[&xml::BaseType]) -> Result<()> {
    for ty in basetypes {
        let m = crate::model::constant::build_basetype(ty);
        emit_basetype(file, &m)?;
    }
    writeln!(file)?;
    Ok(())
}

pub fn generate_type_aliases(
    file: &mut impl Write,
    analysis: &Analysis,
    type_aliases: &[&xml::Alias],
) -> Result<()> {
    for alias in type_aliases {
        let m = crate::model::constant::build_type_alias(analysis, alias);
        emit_type_alias(file, &m)?;
    }
    Ok(())
}

pub fn emit_api_constant(file: &mut impl Write, c: &ApiConstantDef) -> Result<()> {
    writeln!(file, "pub const {}: {} = {};", c.name, c.ty, c.value)?;
    Ok(())
}

pub fn emit_basetype(file: &mut impl Write, b: &BasetypeDef) -> Result<()> {
    writeln!(file, "pub type {} = {};", b.name, b.target)?;
    Ok(())
}

pub fn emit_type_alias(file: &mut impl Write, a: &TypeAliasDef) -> Result<()> {
    write_doc_link(file, &a.c_name)?;
    writeln!(file, "pub type {} = {};", a.name, a.target)?;
    Ok(())
}
