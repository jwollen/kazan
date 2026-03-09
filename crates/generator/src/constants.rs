use std::{borrow::Cow, collections::HashSet, io::Write};

use crate::{
    analysis::Analysis, ctype_to_rust_type_str, normalize_const_name, normalize_ty_name,
    type_name_with_lifetime, write_doc_link, xml,
};

pub fn generate_api_constants(
    file: &mut impl Write,
    owned: &HashSet<&str>,
    required_api_constants: impl Iterator<Item = xml::Constant>,
) {
    let constants = required_api_constants.filter(|constant| owned.contains(constant.name));

    for constant in constants {
        writeln!(
            file,
            "pub const {}: {} = {};",
            normalize_const_name(constant.name),
            ctype_to_rust_type_str(constant.ty),
            convert_c_expr(constant.value),
        )
        .unwrap();
    }
    writeln!(file).unwrap();
}

pub fn generate_basetypes(file: &mut impl Write, analysis: &Analysis, owned: &HashSet<&str>) {
    let basetypes = analysis
        .registry()
        .basetypes
        .iter()
        .filter(|ty| owned.contains(ty.name));

    for ty in basetypes {
        writeln!(
            file,
            "pub type {} = {};",
            normalize_ty_name(ty.name),
            ctype_to_rust_type_str(ty.ty.unwrap_or("*const c_void"))
        )
        .unwrap();
    }
    writeln!(file).unwrap();
}

pub fn generate_type_aliases(file: &mut impl Write, analysis: &Analysis, owned: &HashSet<&str>) {
    let registry = analysis.registry();
    let aliases = registry
        .enum_aliases
        .iter()
        .clone()
        .filter(|alias| {
            registry
                .enums
                .iter()
                .find(|ty| ty.name == alias.alias)
                .is_some()
        })
        .chain(registry.handle_aliases.iter())
        .chain(registry.struct_aliases.iter())
        .chain(registry.bitmask_aliases.iter())
        .filter(|alias| owned.contains(alias.name));

    for alias in aliases {
        write_doc_link(file, alias.name);
        writeln!(
            file,
            "pub type {} = {};",
            type_name_with_lifetime(analysis, alias.name, Some("a")),
            type_name_with_lifetime(analysis, alias.alias, Some("a"))
        )
        .unwrap();
    }

    let command_aliases = registry
        .command_aliases
        .iter()
        .filter(|alias| owned.contains(alias.name));

    for ty in command_aliases {
        writeln!(
            file,
            "pub type PFN_{} = PFN_{};",
            normalize_ty_name(ty.name),
            normalize_ty_name(ty.alias)
        )
        .unwrap();
    }
    writeln!(file).unwrap();
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
        Cow::Owned(format!("!{}", expr))
    } else {
        Cow::Borrowed(expr)
    }
}
