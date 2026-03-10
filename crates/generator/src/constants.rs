use std::{borrow::Cow, io::Write};

use anyhow::Result;

use crate::{
    analysis::Analysis,
    ctype_rust::{base_ctype_to_rust_str, type_name_with_lifetime},
    normalize_const_name, normalize_ty_name, write_doc_link, xml,
};

pub fn generate_api_constants(
    file: &mut impl Write,
    api_constants: &[xml::Constant],
) -> Result<()> {
    for constant in api_constants {
        writeln!(
            file,
            "pub const {}: {} = {};",
            normalize_const_name(constant.name),
            base_ctype_to_rust_str(constant.ty),
            convert_c_expr(constant.value),
        )?;
    }
    writeln!(file)?;
    Ok(())
}

pub fn generate_basetypes(file: &mut impl Write, basetypes: &[&xml::BaseType]) -> Result<()> {
    for ty in basetypes {
        writeln!(
            file,
            "pub type {} = {};",
            normalize_ty_name(ty.name),
            base_ctype_to_rust_str(ty.ty.unwrap_or("*const c_void"))
        )?;
    }
    writeln!(file)?;
    Ok(())
}

pub fn generate_type_aliases(
    file: &mut impl Write,
    analysis: &Analysis,
    type_aliases: &[&xml::Alias],
    command_aliases: &[&xml::Alias],
) -> Result<()> {
    for alias in type_aliases {
        write_doc_link(file, alias.name)?;
        writeln!(
            file,
            "pub type {} = {};",
            type_name_with_lifetime(analysis, alias.name, Some("a")),
            type_name_with_lifetime(analysis, alias.alias, Some("a"))
        )?;
    }

    for ty in command_aliases {
        writeln!(
            file,
            "pub type PFN_{} = PFN_{};",
            normalize_ty_name(ty.name),
            normalize_ty_name(ty.alias)
        )?;
    }
    writeln!(file)?;
    Ok(())
}

fn convert_c_expr<'a>(expr: &'a str) -> Cow<'a, str> {
    let expr = expr
        .strip_prefix('(')
        .and_then(|expr| expr.strip_suffix(')'))
        .unwrap_or(expr);

    let expr = expr
        .strip_suffix('f')
        .or_else(|| expr.strip_suffix('F'))
        .or_else(|| expr.strip_suffix("ULL"))
        .or_else(|| expr.strip_suffix("LL"))
        .or_else(|| expr.strip_suffix('U'))
        .unwrap_or(expr);

    if let Some(expr) = expr.strip_prefix("~") {
        Cow::Owned(format!("!{expr}"))
    } else {
        Cow::Borrowed(expr)
    }
}
