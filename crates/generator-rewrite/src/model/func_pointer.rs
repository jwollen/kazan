//! Model and builders for function pointer types and command aliases.

use super::rust_type::RustType;
use crate::{analysis::Analysis, build::type_conv, normalize_name, normalize_ty_name, xml};

/// Model for a `type [PFN_]Foo = unsafe extern "system" fn(...)` function pointer typedef.
#[derive(Debug, Clone)]
pub struct FuncPointerDef {
    pub c_name: String,
    /// Prefix for the generated type name (e.g. `"PFN_"` for commands, `""` for typedefs).
    pub prefix: &'static str,
    pub params: Vec<FuncPointerParam>,
    pub return_type: Option<RustType>,
}

/// A parameter of a function pointer type.
#[derive(Debug, Clone)]
pub struct FuncPointerParam {
    pub name: String,
    pub ty: RustType,
}

/// Model for a command alias (e.g. `pub type PFN_vkFoo = PFN_vkBar;`).
#[derive(Debug, Clone)]
pub struct CommandAliasDef {
    pub name: String,
    pub target: String,
}

// ── Builders ────────────────────────────────────────────────────────────────

/// Build a `FuncPointerDef` model from a raw XML funcpointer typedef.
pub fn build_func_pointer_typedef(analysis: &Analysis, fp: &xml::FuncPointer) -> FuncPointerDef {
    let params = fp
        .params
        .iter()
        .map(|param| FuncPointerParam {
            name: normalize_name(param.c_decl.name),
            ty: type_conv::resolve_ctype(analysis, &param.c_decl.ty, None),
        })
        .collect();

    let return_type = fp
        .return_type
        .as_ref()
        .map(|rt| type_conv::resolve_ctype(analysis, rt, None));

    FuncPointerDef {
        c_name: fp.name.to_string(),
        prefix: "",
        params,
        return_type,
    }
}

/// Build a `FuncPointerDef` model from a raw XML command definition (PFN_ prefix).
pub fn build_command_func_pointer(analysis: &Analysis, command: &xml::Command) -> FuncPointerDef {
    let params = command
        .params
        .iter()
        .map(|param| FuncPointerParam {
            name: normalize_name(param.c_decl.name),
            ty: type_conv::resolve_ctype(analysis, &param.c_decl.ty, None),
        })
        .collect();

    let return_type = command
        .return_type
        .as_ref()
        .map(|rt| type_conv::resolve_ctype(analysis, rt, None));

    FuncPointerDef {
        c_name: command.name.to_string(),
        prefix: "PFN_",
        params,
        return_type,
    }
}

/// Build a `CommandAliasDef` from a raw XML alias.
pub fn build_command_alias(alias: &xml::Alias) -> CommandAliasDef {
    CommandAliasDef {
        name: normalize_ty_name(alias.name).to_string(),
        target: normalize_ty_name(alias.alias).to_string(),
    }
}
