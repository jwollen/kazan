//! Model and builder for handle types.

use crate::{normalize_ty_name, xml};

/// Model for a handle type (e.g. `define_handle!(Foo, BAR, doc = "...");`).
#[derive(Debug, Clone)]
pub struct HandleDef {
    pub name: &'static str,
    pub c_name: &'static str,
    pub macro_name: &'static str,
    pub obj_type: &'static str,
}

// ── Builder ─────────────────────────────────────────────────────────────────

pub fn build_handle(handle: &xml::Handle) -> HandleDef {
    let macro_name = match handle.ty {
        "VK_DEFINE_HANDLE" => "define_handle",
        "VK_DEFINE_NON_DISPATCHABLE_HANDLE" => "handle_nondispatchable",
        _ => panic!("unknown handle type: {}", handle.ty),
    };

    HandleDef {
        name: normalize_ty_name(handle.name),
        c_name: handle.name,
        macro_name,
        obj_type: handle.objtypeenum.strip_prefix("VK_OBJECT_TYPE_").unwrap(),
    }
}
