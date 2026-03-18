//! Model and builder for union definitions.

use super::rust_type::RustType;
use crate::{analysis::Analysis, build::type_conv, normalize_name, normalize_ty_name, xml};

/// Model for a generated Rust union definition.
#[derive(Debug, Clone)]
pub struct UnionDef {
    pub name: &'static str,
    pub c_name: &'static str,
    pub fields: Vec<UnionField>,
    pub has_lifetime: bool,
}

/// A field in the generated union.
#[derive(Debug, Clone)]
pub struct UnionField {
    pub name: String,
    pub ty: RustType,
}

// ── Builder ─────────────────────────────────────────────────────────────────

/// Build a `UnionDef` model from a raw XML union definition.
pub fn build_union(analysis: &Analysis, union_ty: &xml::Structure) -> UnionDef {
    let type_info = analysis.get_base_type_info(union_ty.name).unwrap();
    let name = normalize_ty_name(union_ty.name);

    let fields = union_ty
        .members
        .iter()
        .map(|member| {
            let field_ty = type_conv::resolve_ctype(analysis, &member.c_decl.ty, Some("a"));
            UnionField {
                name: normalize_name(member.c_decl.name),
                ty: field_ty,
            }
        })
        .collect();

    UnionDef {
        name,
        c_name: union_ty.name,
        fields,
        has_lifetime: type_info.lifetime_param,
    }
}
