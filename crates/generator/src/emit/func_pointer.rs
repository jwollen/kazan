//! Emit layer: FuncPointerDef / CommandAliasDef → Rust source.

use std::io::Write;

use anyhow::Result;

use crate::{
    analysis::Analysis,
    model::func_pointer::{CommandAliasDef, FuncPointerDef},
    write_doc_link, xml,
};

pub fn generate_func_pointer_typedefs(
    file: &mut impl Write,
    analysis: &Analysis,
    funcpointers: &[&xml::FuncPointer],
) -> Result<()> {
    for ty in funcpointers {
        let model = crate::model::func_pointer::build_func_pointer_typedef(analysis, ty);
        emit_func_pointer(file, &model)?;
    }
    writeln!(file)?;
    Ok(())
}

pub fn generate_command_func_pointers(
    file: &mut impl Write,
    analysis: &Analysis,
    commands: &[&xml::Command],
) -> Result<()> {
    for command in commands {
        let model = crate::model::func_pointer::build_command_func_pointer(analysis, command);
        emit_func_pointer(file, &model)?;
    }
    writeln!(file)?;
    Ok(())
}

pub fn generate_command_aliases(
    file: &mut impl Write,
    command_aliases: &[&xml::Alias],
) -> Result<()> {
    for alias in command_aliases {
        let m = crate::model::func_pointer::build_command_alias(alias);
        emit_command_alias(file, &m)?;
    }
    writeln!(file)?;
    Ok(())
}

fn emit_func_pointer(file: &mut impl Write, fp: &FuncPointerDef) -> Result<()> {
    write_doc_link(file, fp.c_name)?;
    writeln!(
        file,
        "pub type {}{} = unsafe extern \"system\" fn(",
        fp.prefix, fp.c_name
    )?;
    for param in &fp.params {
        writeln!(file, "    {}: {},", param.name, param.ty.to_tokens())?;
    }
    if let Some(ref return_type) = fp.return_type {
        writeln!(file, ") -> {};", return_type.to_tokens())?;
    } else {
        writeln!(file, ");")?;
    }
    Ok(())
}

fn emit_command_alias(file: &mut impl Write, a: &CommandAliasDef) -> Result<()> {
    writeln!(file, "pub type PFN_{} = PFN_{};", a.name, a.target)?;
    Ok(())
}
