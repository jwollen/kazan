use std::io::Write;

use anyhow::Result;

use crate::{analysis::Analysis, analysis::ModuleItems, normalize_ty_name};

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

pub fn generate_ffi_module(
    file: &mut impl Write,
    analysis: &Analysis,
    items: &ModuleItems<'_>,
) -> Result<()> {
    let registry = analysis.registry();

    writeln!(
        file,
        "#[cfg(feature = \"ffi\")]
        pub(super) mod ffi {{
        #![allow(non_camel_case_types)]
        use super::defs::*;
        "
    )?;

    // Basetypes: VkFoo -> Foo
    for ty in &items.basetypes {
        let rust_name = normalize_ty_name(ty.name);
        if rust_name != ty.name {
            writeln!(file, "pub type {} = {};", ty.name, rust_name)?;
        }
    }

    // Handles: VkFoo -> Foo
    for ty in &items.handles {
        let rust_name = normalize_ty_name(ty.name);
        let rhs = ffi_rhs(ty.name, rust_name, None);
        writeln!(file, "pub type {} = {};", ty.name, rhs)?;
    }

    // Structs: VkFoo -> Foo or Foo<'static>
    for ty in &items.structs {
        let rust_name = normalize_ty_name(ty.name);
        let type_info = analysis.get_base_type_info(ty.name).unwrap();
        let lt = if type_info.lifetime_param {
            Some("static")
        } else {
            None
        };
        let rhs = ffi_rhs(ty.name, rust_name, lt);
        writeln!(file, "pub type {} = {};", ty.name, rhs)?;
    }

    // Unions: VkFoo -> Foo or Foo<'static>
    for ty in &items.unions {
        let rust_name = normalize_ty_name(ty.name);
        let type_info = analysis.get_base_type_info(ty.name).unwrap();
        let lt = if type_info.lifetime_param {
            Some("static")
        } else {
            None
        };
        let rhs = ffi_rhs(ty.name, rust_name, lt);
        writeln!(file, "pub type {} = {};", ty.name, rhs)?;
    }

    // Enum types: VkFoo -> Foo
    for ty in &items.enums {
        let rust_name = if ty.name == "VkResult" {
            "Result"
        } else {
            normalize_ty_name(ty.name)
        };
        let rhs = ffi_rhs(ty.name, rust_name, None);
        writeln!(file, "pub type {} = {};", ty.name, rhs)?;
    }

    // Bitmask types: VkFooFlags -> FooFlags, and associated FlagBits type
    for ty in &items.bitmask_types {
        let rust_name = normalize_ty_name(ty.name);
        let rhs = ffi_rhs(ty.name, rust_name, None);
        writeln!(file, "pub type {} = {};", ty.name, rhs)?;

        // Also alias the FlagBits type if it exists
        if let Some(bitmask_name) = ty.bitvalues.or(ty.requires)
            && registry.bitmasks.iter().any(|b| b.name == bitmask_name)
        {
            let rust_bitmask_name = normalize_ty_name(bitmask_name);
            let rhs = ffi_rhs(bitmask_name, rust_bitmask_name, None);
            writeln!(file, "pub type {bitmask_name} = {rhs};")?;
        }
    }

    // Type aliases (enum/handle/struct/bitmask)
    for alias in &items.type_aliases {
        let rust_name = normalize_ty_name(alias.name);
        let type_info = analysis.get_base_type_info(alias.name);
        let lt = type_info
            .filter(|info| info.lifetime_param)
            .map(|_| "static");
        let rhs = ffi_rhs(alias.name, rust_name, lt);
        writeln!(file, "pub type {} = {};", alias.name, rhs)?;
    }

    // drop_lifetime_for_ffi for structs and unions with lifetime parameters
    for ty_name in items.structs.iter().chain(&items.unions).map(|ty| ty.name) {
        let type_info = analysis.get_base_type_info(ty_name).unwrap();
        if type_info.lifetime_param {
            let rust_name = normalize_ty_name(ty_name);
            // When C name == Rust name, qualify to avoid ambiguity with the type alias above.
            let qualified = if ty_name == rust_name {
                format!("super::defs::{rust_name}")
            } else {
                rust_name.to_string()
            };
            writeln!(
                file,
                "impl {qualified}<'_> {{
                #[inline]
                pub unsafe fn drop_lifetime_for_ffi(&self) -> &{ty_name} {{
                    unsafe {{ core::mem::transmute(self) }}
                }}
            }}"
            )?;
        }
    }

    writeln!(file, "}}\n")?;
    Ok(())
}
