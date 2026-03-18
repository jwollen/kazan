//! Model and builder for handle types.

use crate::{doc_url, normalize_ty_name, xml};

/// Model for a handle type (e.g. `define_handle!(Foo, BAR, doc = "...");`).
#[derive(Debug, Clone)]
pub struct HandleDef {
    pub name: String,
    pub c_name: String,
    pub macro_name: String,
    pub obj_type: String,
    pub doc_url: String,
}

// ── Builder ─────────────────────────────────────────────────────────────────

pub fn build_handle(handle: &xml::Handle) -> HandleDef {
    let macro_name = match handle.ty {
        "VK_DEFINE_HANDLE" => "define_handle",
        "VK_DEFINE_NON_DISPATCHABLE_HANDLE" => "handle_nondispatchable",
        _ => panic!("unknown handle type: {}", handle.ty),
    };

    let name = normalize_ty_name(handle.name).to_string();
    let obj_type = handle
        .objtypeenum
        .strip_prefix("VK_OBJECT_TYPE_")
        .unwrap()
        .to_string();

    HandleDef {
        name,
        c_name: handle.name.to_string(),
        macro_name: macro_name.to_string(),
        obj_type,
        doc_url: doc_url(handle.name),
    }
}
