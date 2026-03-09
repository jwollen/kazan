//! Strongly-typed C type classification and C→Rust type conversion.

use crate::{analysis::Analysis, cdecl::CType, normalize_ty_name};

/// Classification of a C type for clear branching without deep matching.
#[derive(Debug, Clone)]
pub enum CTypeCategory<'a> {
    /// Base type (integer, float, named struct/enum/handle, etc.)
    Base(&'a str),
    /// Fixed-size array
    Array { element: &'a CType<'a> },
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
    pub fn from_ctype(ty: &'a CType<'a>, analysis: &Analysis) -> CTypeCategory<'a> {
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
                        if name == "char" || analysis.is_opaque_type_name(name) {
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
            CType::Array { element, .. } => CTypeCategory::Array { element },
            CType::Func { .. } => CTypeCategory::FuncPointer,
        }
    }
}

pub(crate) fn base_name<'a>(ty: &'a CType<'a>) -> Option<&'a str> {
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
pub fn base_ctype_to_rust_str(name: &str) -> &str {
    match name {
        "int8_t" => "i8",
        "uint8_t" => "u8",
        "int16_t" => "i16",
        "uint16_t" => "u16",
        "int32_t" => "i32",
        "uint32_t" => "u32",
        "int64_t" => "i64",
        "uint64_t" => "u64",
        "size_t" => "usize",
        "isize_t" => "isize",
        "float" => "f32",
        "double" => "f64",
        "void" => "c_void",
        "char" => "c_char",
        "int" => "c_int",
        "unsigned int" => "c_uint",
        "unsigned long" => "c_ulong",
        _ => normalize_ty_name(name),
    }
}
