//! Model and builder for the FFI compatibility module.

use crate::{
    analysis::{Analysis, ModuleItems},
    normalize_ty_name,
};

/// Model for the FFI compatibility module within a generated Vulkan module.
#[derive(Debug, Clone)]
pub struct FfiModuleDef {
    /// Type aliases: `pub type VkFoo = Foo;` or `pub type VkFoo = super::defs::Foo;`
    pub aliases: Vec<FfiAlias>,
    /// Structs/unions with lifetime params that need `drop_lifetime_for_ffi`.
    pub lifetime_impls: Vec<FfiLifetimeImpl>,
}

/// A single FFI type alias.
#[derive(Debug, Clone)]
pub struct FfiAlias {
    pub c_name: String,
    pub rhs: String,
}

/// A `drop_lifetime_for_ffi` impl for a struct/union with a lifetime parameter.
#[derive(Debug, Clone)]
pub struct FfiLifetimeImpl {
    /// Qualified Rust name (e.g. `super::defs::Foo` or `Foo`).
    pub qualified_name: String,
    /// The C type name used in the return type.
    pub c_name: String,
}

// ── Builder ─────────────────────────────────────────────────────────────────

/// Returns the Rust name qualified with `super::defs::` when needed to avoid
/// recursive type aliases (when the C name equals the Rust name).
fn ffi_rhs(c_name: &str, rust_name: &str, lifetime: Option<&str>) -> String {
    let qualified = if c_name == rust_name {
        format!("super::defs::{rust_name}")
    } else {
        rust_name.to_string()
    };
    match lifetime {
        Some(lt) => format!("{qualified}<'{lt}>"),
        None => qualified,
    }
}

pub fn build_ffi_module(analysis: &Analysis, items: &ModuleItems<'_>) -> FfiModuleDef {
    let registry = analysis.registry();
    let mut aliases = Vec::new();
    let mut lifetime_impls = Vec::new();

    // Basetypes
    for ty in &items.basetypes {
        let rust_name = normalize_ty_name(ty.name);
        if rust_name != ty.name {
            aliases.push(FfiAlias {
                c_name: ty.name.to_string(),
                rhs: rust_name.to_string(),
            });
        }
    }

    // Handles
    for ty in &items.handles {
        let rust_name = normalize_ty_name(ty.name);
        let rhs = ffi_rhs(ty.name, rust_name, None);
        aliases.push(FfiAlias {
            c_name: ty.name.to_string(),
            rhs,
        });
    }

    // Structs
    for ty in &items.structs {
        let rust_name = normalize_ty_name(ty.name);
        let type_info = analysis.get_base_type_info(ty.name).unwrap();
        let lt = if type_info.lifetime_param {
            Some("static")
        } else {
            None
        };
        let rhs = ffi_rhs(ty.name, rust_name, lt);
        aliases.push(FfiAlias {
            c_name: ty.name.to_string(),
            rhs,
        });
    }

    // Unions
    for ty in &items.unions {
        let rust_name = normalize_ty_name(ty.name);
        let type_info = analysis.get_base_type_info(ty.name).unwrap();
        let lt = if type_info.lifetime_param {
            Some("static")
        } else {
            None
        };
        let rhs = ffi_rhs(ty.name, rust_name, lt);
        aliases.push(FfiAlias {
            c_name: ty.name.to_string(),
            rhs,
        });
    }

    // Enum types
    for ty in &items.enums {
        let rust_name = if ty.name == "VkResult" {
            "Result"
        } else {
            normalize_ty_name(ty.name)
        };
        let rhs = ffi_rhs(ty.name, rust_name, None);
        aliases.push(FfiAlias {
            c_name: ty.name.to_string(),
            rhs,
        });
    }

    // Bitmask types
    for ty in &items.bitmask_types {
        let rust_name = normalize_ty_name(ty.name);
        let rhs = ffi_rhs(ty.name, rust_name, None);
        aliases.push(FfiAlias {
            c_name: ty.name.to_string(),
            rhs,
        });

        if let Some(bitmask_name) = ty.bitvalues.or(ty.requires)
            && registry.bitmasks.iter().any(|b| b.name == bitmask_name)
        {
            let rust_bitmask_name = normalize_ty_name(bitmask_name);
            let rhs = ffi_rhs(bitmask_name, rust_bitmask_name, None);
            aliases.push(FfiAlias {
                c_name: bitmask_name.to_string(),
                rhs,
            });
        }
    }

    // Type aliases
    for alias in &items.type_aliases {
        let rust_name = normalize_ty_name(alias.name);
        let type_info = analysis.get_base_type_info(alias.name);
        let lt = type_info
            .filter(|info| info.lifetime_param)
            .map(|_| "static");
        let rhs = ffi_rhs(alias.name, rust_name, lt);
        aliases.push(FfiAlias {
            c_name: alias.name.to_string(),
            rhs,
        });
    }

    // drop_lifetime_for_ffi impls
    for ty_name in items.structs.iter().chain(&items.unions).map(|ty| ty.name) {
        let type_info = analysis.get_base_type_info(ty_name).unwrap();
        if type_info.lifetime_param {
            let rust_name = normalize_ty_name(ty_name);
            let qualified = if ty_name == rust_name {
                format!("super::defs::{rust_name}")
            } else {
                rust_name.to_string()
            };
            lifetime_impls.push(FfiLifetimeImpl {
                qualified_name: qualified,
                c_name: ty_name.to_string(),
            });
        }
    }

    FfiModuleDef {
        aliases,
        lifetime_impls,
    }
}
