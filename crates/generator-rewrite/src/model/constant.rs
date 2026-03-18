//! Model and builders for API constants, basetypes, and type aliases.

use std::borrow::Cow;

use crate::{
    analysis::Analysis,
    ctype::{base_ctype_to_rust_str, type_name_with_lifetime},
    normalize_const_name, normalize_ty_name, xml,
};

/// Model for a generated API constant (e.g. `pub const MAX_FOO: u32 = 42;`).
#[derive(Debug, Clone)]
pub struct ApiConstantDef {
    pub name: String,
    pub ty: String,
    pub value: String,
}

/// Model for a basetype alias (e.g. `pub type Flags = u32;`).
#[derive(Debug, Clone)]
pub struct BasetypeDef {
    pub name: String,
    pub target: String,
}

/// Model for a type alias (e.g. `pub type Foo<'a> = Bar<'a>;`).
#[derive(Debug, Clone)]
pub struct TypeAliasDef {
    pub c_name: String,
    pub name: String,
    pub target: String,
}

// ── Builders ────────────────────────────────────────────────────────────────

pub fn build_api_constant(constant: &xml::Constant) -> ApiConstantDef {
    ApiConstantDef {
        name: normalize_const_name(constant.name).to_string(),
        ty: base_ctype_to_rust_str(constant.ty).to_string(),
        value: convert_c_expr(constant.value).into_owned(),
    }
}

pub fn build_basetype(basetype: &xml::BaseType) -> BasetypeDef {
    BasetypeDef {
        name: normalize_ty_name(basetype.name).to_string(),
        target: base_ctype_to_rust_str(basetype.ty.unwrap_or("*const c_void")).to_string(),
    }
}

pub fn build_type_alias(analysis: &Analysis, alias: &xml::Alias) -> TypeAliasDef {
    TypeAliasDef {
        c_name: alias.name.to_string(),
        name: type_name_with_lifetime(analysis, alias.name, Some("a")),
        target: type_name_with_lifetime(analysis, alias.alias, Some("a")),
    }
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
