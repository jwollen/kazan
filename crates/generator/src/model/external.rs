//! Model and builder for external type definitions.

use crate::{analysis::Analysis, build::type_conv, external::external_types};

/// Model for the external type file (generated/external.rs).
#[derive(Debug, Clone)]
pub struct ExternalTypeDef {
    pub name: String,
    pub rust_type: String,
}

// ── Builder ─────────────────────────────────────────────────────────────────

pub fn build_external_types(analysis: &Analysis) -> Vec<ExternalTypeDef> {
    external_types()
        .iter()
        .map(|(name, ty)| ExternalTypeDef {
            name: name.to_string(),
            rust_type: type_conv::resolve_ctype(analysis, ty, None).to_tokens(),
        })
        .collect()
}
