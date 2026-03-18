pub mod command;
pub mod enumeration;
pub mod external;
pub mod ffi;
pub mod func_pointer;
pub mod handle;
pub mod module;
pub mod rust_type;
pub mod structure;
pub mod r#union;

// ── Constants, basetypes, type aliases ───────────────────────────────────────

use std::borrow::Cow;

use crate::{
    analysis::Analysis,
    ctype::{base_ctype_to_rust_str, type_name_with_lifetime},
    normalize_const_name, normalize_ty_name, xml,
};

/// Model for a generated API constant (e.g. `pub const MAX_FOO: u32 = 42;`).
#[derive(Debug, Clone)]
pub struct ApiConstantDef {
    pub name: &'static str,
    pub ty: &'static str,
    pub value: Cow<'static, str>,
}

/// Model for a basetype alias (e.g. `pub type Flags = u32;`).
#[derive(Debug, Clone)]
pub struct BasetypeDef {
    pub name: &'static str,
    pub target: &'static str,
}

/// Model for a type alias (e.g. `pub type Foo<'a> = Bar<'a>;`).
#[derive(Debug, Clone)]
pub struct TypeAliasDef {
    pub c_name: &'static str,
    pub name: String,
    pub target: String,
}

// ── Builders ────────────────────────────────────────────────────────────────

pub fn build_api_constant(constant: &xml::Constant) -> ApiConstantDef {
    ApiConstantDef {
        name: normalize_const_name(constant.name),
        ty: base_ctype_to_rust_str(constant.ty),
        value: convert_c_expr(constant.value),
    }
}

pub fn build_basetype(basetype: &xml::BaseType) -> BasetypeDef {
    BasetypeDef {
        name: normalize_ty_name(basetype.name),
        target: base_ctype_to_rust_str(basetype.ty.unwrap_or("*const c_void")),
    }
}

pub fn build_type_alias(analysis: &Analysis, alias: &xml::Alias) -> TypeAliasDef {
    TypeAliasDef {
        c_name: alias.name,
        name: type_name_with_lifetime(analysis, alias.name, Some("a")),
        target: type_name_with_lifetime(analysis, alias.alias, Some("a")),
    }
}

fn convert_c_expr(expr: &'static str) -> Cow<'static, str> {
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
