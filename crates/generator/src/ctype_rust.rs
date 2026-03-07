//! Strongly-typed C type classification and C→Rust type conversion.
//!
//! Replaces ad-hoc `CType` matching with:
//! - **CTypeCategory**: what kind of C type we have (opaque ptr, char ptr, slice ptr, etc.)
//! - **RustTypeRepr**: structured Rust type that can be rendered to a string in one place.

use std::fmt;

use crate::{
    analysis::Analysis,
    cdecl::{CArrayLen, CType},
    normalize_const_name, normalize_ty_name,
};

/// Classification of a C type for clear branching without deep matching.
#[derive(Debug, Clone)]
pub enum CTypeCategory<'a> {
    /// Base type (integer, float, named struct/enum/handle, etc.)
    Base(&'a str),
    /// Fixed-size array
    Array {
        element: &'a CType<'a>,
        len: &'a CArrayLen<'a>,
    },
    /// Pointer to opaque type (void*, handle ptr, etc.) — stays as raw pointer in Rust.
    OpaquePointer {
        is_const: bool,
        pointee_name: &'a str,
    },
    /// Pointer to char — becomes CStr / &CStr in Rust.
    CharPointer { is_const: bool },
    /// Pointer to known non-opaque pointee — can become &T / &[T] in Rust.
    TypedPointer {
        is_const: bool,
        pointee: &'a CType<'a>,
    },
    /// Function pointer (e.g. PFN_*).
    FuncPointer,
}

impl<'a> CTypeCategory<'a> {
    /// Returns the category for a C type. Use this instead of matching on `CType` directly.
    pub fn from_ctype(ty: &'a CType<'a>, _analysis: &Analysis) -> CTypeCategory<'a> {
        match ty {
            CType::Base(base) => {
                if base.name.starts_with("PFN_") {
                    CTypeCategory::FuncPointer
                } else {
                    CTypeCategory::Base(base.name)
                }
            }
            CType::Ptr {
                pointee, is_const, ..
            } => {
                let pointee = pointee.as_ref();
                match pointee {
                    CType::Base(inner) => {
                        let name = inner.name;
                        if name == "void" || name == "char" || is_opaque_type(name) {
                            if name == "char" {
                                CTypeCategory::CharPointer {
                                    is_const: *is_const,
                                }
                            } else {
                                CTypeCategory::OpaquePointer {
                                    is_const: *is_const,
                                    pointee_name: name,
                                }
                            }
                        } else {
                            CTypeCategory::TypedPointer {
                                is_const: *is_const,
                                pointee,
                            }
                        }
                    }
                    CType::Ptr { .. } => {
                        let name = base_name(pointee).unwrap_or("c_void");
                        CTypeCategory::OpaquePointer {
                            is_const: *is_const,
                            pointee_name: name,
                        }
                    }
                    _ => CTypeCategory::TypedPointer {
                        is_const: *is_const,
                        pointee,
                    },
                }
            }
            CType::Array { element, len } => CTypeCategory::Array { element, len },
            CType::Func { .. } => CTypeCategory::FuncPointer,
        }
    }

    pub fn is_opaque_or_char_ptr(&self) -> bool {
        matches!(
            self,
            CTypeCategory::OpaquePointer { .. } | CTypeCategory::CharPointer { .. }
        )
    }

    pub fn is_ptr(&self) -> bool {
        matches!(
            self,
            CTypeCategory::OpaquePointer { .. }
                | CTypeCategory::CharPointer { .. }
                | CTypeCategory::TypedPointer { .. }
        )
    }
}

fn base_name<'a>(ty: &'a CType<'a>) -> Option<&'a str> {
    match ty {
        CType::Base(b) => Some(b.name),
        CType::Ptr { pointee, .. } => base_name(pointee),
        CType::Array { element, .. } => base_name(element),
        CType::Func { .. } => None,
    }
}

pub fn is_bool32(ty: &CType) -> bool {
    matches!(ty, CType::Base(b) if b.name == "VkBool32")
}

pub fn is_opaque_type(name: &str) -> bool {
    matches!(
        name,
        "void"
            | "wl_display"
            | "wl_surface"
            | "Display"
            | "xcb_connection_t"
            | "ANativeWindow"
            | "AHardwareBuffer"
            | "CAMetalLayer"
            | "IDirectFB"
            | "IDirectFBSurface"
            | "_screen_buffer"
            | "_screen_context"
            | "_screen_window"
            | "SECURITY_ATTRIBUTES"
    )
}

/// Structured representation of a Rust type. Rendered to string in one place.
#[derive(Debug, Clone)]
pub enum RustTypeRepr {
    Named(String),
    PtrConst(String),
    PtrMut(String),
    Ref {
        lifetime: Option<String>,
        is_mut: bool,
        inner: String,
    },
    Slice {
        lifetime: Option<String>,
        is_mut: bool,
        element: String,
    },
    Array {
        element: String,
        len: ArrayLenRepr,
    },
    Option(Box<RustTypeRepr>),
}

#[derive(Debug, Clone)]
pub enum ArrayLenRepr {
    Named(String),
    Literal(u128),
}

impl fmt::Display for RustTypeRepr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RustTypeRepr::Named(s) => write!(f, "{}", s),
            RustTypeRepr::PtrConst(s) => write!(f, "*const {}", s),
            RustTypeRepr::PtrMut(s) => write!(f, "*mut {}", s),
            RustTypeRepr::Ref {
                lifetime,
                is_mut,
                inner,
            } => {
                let lt = lifetime
                    .as_deref()
                    .map(|l| format!("'{} ", l))
                    .unwrap_or_default();
                if *is_mut {
                    write!(f, "&{}mut {}", lt, inner)
                } else {
                    write!(f, "&{}{}", lt, inner)
                }
            }
            RustTypeRepr::Slice {
                lifetime,
                is_mut,
                element,
            } => {
                let lt = lifetime
                    .as_deref()
                    .map(|l| format!("'{} ", l))
                    .unwrap_or_default();
                if *is_mut {
                    write!(f, "&{}mut [{}]", lt, element)
                } else {
                    write!(f, "&{}[{}]", lt, element)
                }
            }
            RustTypeRepr::Array { element, len } => {
                let len_str = match len {
                    ArrayLenRepr::Named(n) => n.clone(),
                    ArrayLenRepr::Literal(l) => l.to_string(),
                };
                write!(f, "[{}; {}]", element, len_str)
            }
            RustTypeRepr::Option(inner) => write!(f, "Option<{}>", inner),
        }
    }
}

/// Context for C→Rust conversion: struct field (raw), setter param (slices/refs), or command param.
#[derive(Debug, Clone, Copy)]
pub enum ConversionContext {
    /// FFI / struct field: pointers stay as *const T / *mut T.
    Raw,
    /// Setter parameter: pointers with length can become &[T], optional can wrap.
    SetterParam {
        has_non_literal_len: bool,
        lifetime: Option<&'static str>,
    },
    /// Command parameter: same idea as setter; used for wrapper API.
    CommandParam {
        has_non_literal_len: bool,
        optional: (bool, bool),
        lifetime: Option<&'static str>,
    },
}

/// Convert a C type to a structured Rust type representation.
pub fn ctype_to_rust_repr(
    analysis: &Analysis,
    ty: &CType,
    lifetime: Option<&str>,
    context: ConversionContext,
) -> RustTypeRepr {
    let category = CTypeCategory::from_ctype(ty, analysis);
    ctype_category_to_rust(analysis, ty, &category, lifetime, context)
}

fn ctype_category_to_rust(
    analysis: &Analysis,
    ty: &CType,
    category: &CTypeCategory,
    lifetime: Option<&str>,
    context: ConversionContext,
) -> RustTypeRepr {
    let lt = lifetime.map(String::from);
    let base_name_str = |name: &str| type_name_with_lifetime(analysis, name, lifetime);

    match (category, context) {
        (CTypeCategory::Base(name), _) => RustTypeRepr::Named(base_name_str(name).to_string()),

        (CTypeCategory::Array { element, len }, _) => {
            let element_ty = ctype_to_rust_repr(analysis, element, lifetime, context);
            let element_s = element_ty.to_string();
            let len_repr = match len {
                CArrayLen::Named(n) => {
                    ArrayLenRepr::Named(format!("{} as usize", normalize_const_name(n)))
                }
                CArrayLen::Literal(l) => ArrayLenRepr::Literal(*l),
            };
            RustTypeRepr::Array {
                element: element_s,
                len: len_repr,
            }
        }

        (
            CTypeCategory::OpaquePointer {
                is_const,
                pointee_name,
            },
            _,
        ) => {
            let inner = base_name_str(pointee_name).to_string();
            if *is_const {
                RustTypeRepr::PtrConst(inner)
            } else {
                RustTypeRepr::PtrMut(inner)
            }
        }

        (CTypeCategory::CharPointer { is_const }, ConversionContext::Raw) => {
            let inner = "c_char".to_string();
            if *is_const {
                RustTypeRepr::PtrConst(inner)
            } else {
                RustTypeRepr::PtrMut(inner)
            }
        }
        (CTypeCategory::CharPointer { is_const }, _) => {
            let inner = "CStr".to_string();
            let is_mut = !*is_const;
            RustTypeRepr::Ref {
                lifetime: lt,
                is_mut,
                inner,
            }
        }

        (CTypeCategory::TypedPointer { is_const, pointee }, ConversionContext::Raw) => {
            let inner = ctype_to_rust_repr(analysis, pointee, lifetime, context).to_string();
            if *is_const {
                RustTypeRepr::PtrConst(inner)
            } else {
                RustTypeRepr::PtrMut(inner)
            }
        }
        (
            CTypeCategory::TypedPointer { is_const, pointee },
            ConversionContext::SetterParam {
                has_non_literal_len,
                ..
            }
            | ConversionContext::CommandParam {
                has_non_literal_len,
                ..
            },
        ) => {
            if has_non_literal_len {
                let element_ty = ctype_to_rust_repr(analysis, pointee, lifetime, context);
                let element_s = if element_ty.to_string() == "c_void" {
                    "u8".to_string()
                } else {
                    element_ty.to_string()
                };
                RustTypeRepr::Slice {
                    lifetime: lt,
                    is_mut: !*is_const,
                    element: element_s,
                }
            } else {
                let inner = ctype_to_rust_repr(analysis, pointee, lifetime, context).to_string();
                if is_opaque_type(inner.as_str()) {
                    if *is_const {
                        RustTypeRepr::PtrConst(inner)
                    } else {
                        RustTypeRepr::PtrMut(inner)
                    }
                } else {
                    RustTypeRepr::Ref {
                        lifetime: lt,
                        is_mut: !*is_const,
                        inner,
                    }
                }
            }
        }

        (CTypeCategory::FuncPointer, _) => {
            let name = match ty {
                CType::Base(b) => b.name,
                _ => "c_void",
            };
            RustTypeRepr::Named(base_name_str(name).to_string())
        }
    }
}

pub fn type_name_with_lifetime(analysis: &Analysis, name: &str, lifetime: Option<&str>) -> String {
    let type_info = match analysis.get_base_type_info(name) {
        Some(t) => t,
        None => return base_ctype_to_rust_str(name).to_string(),
    };
    let name_str = base_ctype_to_rust_str(name);
    if type_info.lifetime_param {
        format!("{}<'{}>", name_str, lifetime.unwrap_or("_"))
    } else {
        name_str.to_string()
    }
}

/// Base type name only (no pointers/arrays). Used for primitives and type names in the registry.
pub fn base_ctype_to_rust_str(name: &str) -> std::borrow::Cow<'_, str> {
    let s = match name {
        "int8_t" => "i8",
        "uint8_t" => "u8",
        "int16_t" => "i16",
        "uint16_t" => "u16",
        "int32_t" => "i32",
        "uint32_t" => "u32",
        "int64_t" => "i64",
        "uint64_t" => "u64",
        "size_t" => "usize",
        "float" => "f32",
        "double" => "f64",
        "void" => "c_void",
        "char" => "c_char",
        "int" => "c_int",
        _ => return std::borrow::Cow::Owned(normalize_ty_name(name).to_string()),
    };
    std::borrow::Cow::Borrowed(s)
}
