use std::borrow::Cow;
use std::fmt;

/// Well-known primitive and common types that appear frequently in the generated output.
///
/// Using an enum avoids allocating a `String` for each occurrence.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RustPrimitiveType {
    Bool,
    U8,
    I8,
    U16,
    I16,
    U32,
    I32,
    U64,
    I64,
    F32,
    F64,
    USize,
    ISize,
    CVoid,
    CChar,
    CInt,
    CUint,
    CUlong,
    Bool32,
    CStr,
    VkResult,
}

impl RustPrimitiveType {
    /// Try to convert a type name string to a `RustPrimitiveType`.
    pub fn parse(s: &str) -> Option<Self> {
        Some(match s {
            "bool" => Self::Bool,
            "u8" => Self::U8,
            "i8" => Self::I8,
            "u16" => Self::U16,
            "i16" => Self::I16,
            "u32" => Self::U32,
            "i32" => Self::I32,
            "u64" => Self::U64,
            "i64" => Self::I64,
            "f32" => Self::F32,
            "f64" => Self::F64,
            "usize" => Self::USize,
            "isize" => Self::ISize,
            "c_void" => Self::CVoid,
            "c_char" => Self::CChar,
            "c_int" => Self::CInt,
            "c_uint" => Self::CUint,
            "c_ulong" => Self::CUlong,
            "Bool32" => Self::Bool32,
            "CStr" => Self::CStr,
            "VkResult" => Self::VkResult,
            _ => return None,
        })
    }

    /// The Rust token string for this primitive.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Bool => "bool",
            Self::U8 => "u8",
            Self::I8 => "i8",
            Self::U16 => "u16",
            Self::I16 => "i16",
            Self::U32 => "u32",
            Self::I32 => "i32",
            Self::U64 => "u64",
            Self::I64 => "i64",
            Self::F32 => "f32",
            Self::F64 => "f64",
            Self::USize => "usize",
            Self::ISize => "isize",
            Self::CVoid => "c_void",
            Self::CChar => "c_char",
            Self::CInt => "c_int",
            Self::CUint => "c_uint",
            Self::CUlong => "c_ulong",
            Self::Bool32 => "Bool32",
            Self::CStr => "CStr",
            Self::VkResult => "VkResult",
        }
    }
}

impl fmt::Display for RustPrimitiveType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

/// Strongly typed representation of every Rust type the generator can produce.
///
/// This replaces all ad-hoc `format!("&[{ty}]")` / `format!("Option<{s}>")` patterns
/// with a single inspectable data structure that owns the full type tree.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RustType {
    /// Well-known primitive type (avoids allocation).
    Primitive(RustPrimitiveType),
    /// Named type with optional lifetime: `Foo`, `Foo<'a>`.
    Named {
        name: Cow<'static, str>,
        lifetime: Option<Cow<'static, str>>,
    },
    /// `&T` or `&mut T`.
    Ref {
        lifetime: Option<Cow<'static, str>>,
        mutable: bool,
        inner: Box<RustType>,
    },
    /// `*const T` or `*mut T`.
    RawPtr { mutable: bool, inner: Box<RustType> },
    /// `&[T]` or `&mut [T]`.
    Slice {
        lifetime: Option<Cow<'static, str>>,
        mutable: bool,
        element: Box<RustType>,
    },
    /// `[T; N]`.
    Array {
        element: Box<RustType>,
        len: Cow<'static, str>,
    },
    /// `Option<T>`.
    Option(Box<RustType>),
    /// `(A, B, ...)`.
    Tuple(Vec<RustType>),
    /// `SliceOrLen<'a, T>`.
    SliceOrLen {
        lifetime: Option<Cow<'static, str>>,
        element: Box<RustType>,
    },
    /// `impl ExtendUninit<T>`.
    ImplExtendUninit(Box<RustType>),
    /// `()`.
    Unit,
}

impl RustType {
    // Shorthand constants for commonly used primitive types.
    pub const BOOL: Self = Self::Primitive(RustPrimitiveType::Bool);
    pub const U8: Self = Self::Primitive(RustPrimitiveType::U8);
    pub const I8: Self = Self::Primitive(RustPrimitiveType::I8);
    pub const U16: Self = Self::Primitive(RustPrimitiveType::U16);
    pub const I16: Self = Self::Primitive(RustPrimitiveType::I16);
    pub const U32: Self = Self::Primitive(RustPrimitiveType::U32);
    pub const I32: Self = Self::Primitive(RustPrimitiveType::I32);
    pub const U64: Self = Self::Primitive(RustPrimitiveType::U64);
    pub const I64: Self = Self::Primitive(RustPrimitiveType::I64);
    pub const F32: Self = Self::Primitive(RustPrimitiveType::F32);
    pub const F64: Self = Self::Primitive(RustPrimitiveType::F64);
    pub const USIZE: Self = Self::Primitive(RustPrimitiveType::USize);
    pub const ISIZE: Self = Self::Primitive(RustPrimitiveType::ISize);
    pub const C_VOID: Self = Self::Primitive(RustPrimitiveType::CVoid);
    pub const C_CHAR: Self = Self::Primitive(RustPrimitiveType::CChar);
    pub const C_INT: Self = Self::Primitive(RustPrimitiveType::CInt);
    pub const C_UINT: Self = Self::Primitive(RustPrimitiveType::CUint);
    pub const C_ULONG: Self = Self::Primitive(RustPrimitiveType::CUlong);
    pub const BOOL32: Self = Self::Primitive(RustPrimitiveType::Bool32);
    pub const CSTR: Self = Self::Primitive(RustPrimitiveType::CStr);
    pub const VK_RESULT: Self = Self::Primitive(RustPrimitiveType::VkResult);

    /// Render this type to its Rust source representation.
    pub fn to_tokens(&self) -> String {
        match self {
            RustType::Primitive(p) => p.as_str().to_string(),
            RustType::Named { name, lifetime } => match lifetime {
                Some(lt) => format!("{name}<'{lt}>"),
                None => name.to_string(),
            },
            RustType::Ref {
                lifetime,
                mutable,
                inner,
            } => {
                let lt = lifetime
                    .as_ref()
                    .map(|lt| format!("'{lt} "))
                    .unwrap_or_default();
                let mutability = if *mutable { "mut " } else { "" };
                let inner = inner.to_tokens();
                format!("&{lt}{mutability}{inner}")
            }
            RustType::RawPtr { mutable, inner } => {
                let inner = inner.to_tokens();
                if *mutable {
                    format!("*mut {inner}")
                } else {
                    format!("*const {inner}")
                }
            }
            RustType::Slice {
                lifetime,
                mutable,
                element,
            } => {
                let lt = lifetime
                    .as_ref()
                    .map(|lt| format!("'{lt} "))
                    .unwrap_or_default();
                let mutability = if *mutable { "mut " } else { "" };
                let element = element.to_tokens();
                format!("&{lt}{mutability}[{element}]")
            }
            RustType::Array { element, len } => {
                let element = element.to_tokens();
                format!("[{element}; {len}]")
            }
            RustType::Option(inner) => {
                let inner = inner.to_tokens();
                format!("Option<{inner}>")
            }
            RustType::Tuple(elements) => {
                if elements.is_empty() {
                    "()".to_string()
                } else {
                    let inner: Vec<_> = elements.iter().map(|e| e.to_tokens()).collect();
                    format!("({})", inner.join(", "))
                }
            }
            RustType::SliceOrLen { lifetime, element } => {
                let lt = lifetime
                    .as_ref()
                    .map(|lt| format!("'{lt}, "))
                    .unwrap_or_default();
                let element = element.to_tokens();
                format!("SliceOrLen<{lt}{element}>")
            }
            RustType::ImplExtendUninit(inner) => {
                let inner = inner.to_tokens();
                format!("impl ExtendUninit<{inner}>")
            }
            RustType::Unit => "()".to_string(),
        }
    }

    /// Wrap this type in `Option<_>`.
    pub fn optional(self) -> Self {
        RustType::Option(Box::new(self))
    }

    /// Convert to a slice type `&[T]` or `&mut [T]`.
    pub fn into_slice(self, lifetime: Option<Cow<'static, str>>, mutable: bool) -> Self {
        RustType::Slice {
            lifetime,
            mutable,
            element: Box::new(self),
        }
    }

    /// Convert to a reference type `&T` or `&mut T`.
    pub fn into_ref(self, lifetime: Option<Cow<'static, str>>, mutable: bool) -> Self {
        RustType::Ref {
            lifetime,
            mutable,
            inner: Box::new(self),
        }
    }

    /// Convert to a raw pointer `*const T` or `*mut T`.
    pub fn into_raw_ptr(self, mutable: bool) -> Self {
        RustType::RawPtr {
            mutable,
            inner: Box::new(self),
        }
    }

    /// Returns true if this type (or any nested type) contains a lifetime parameter.
    pub fn has_lifetime(&self) -> bool {
        match self {
            RustType::Primitive(_) | RustType::Unit => false,
            RustType::Named { lifetime, .. } => lifetime.is_some(),
            RustType::Ref {
                lifetime, inner, ..
            } => lifetime.is_some() || inner.has_lifetime(),
            RustType::RawPtr { inner, .. } => inner.has_lifetime(),
            RustType::Slice {
                lifetime, element, ..
            } => lifetime.is_some() || element.has_lifetime(),
            RustType::Array { element, .. } => element.has_lifetime(),
            RustType::Option(inner) => inner.has_lifetime(),
            RustType::Tuple(elements) => elements.iter().any(|e| e.has_lifetime()),
            RustType::SliceOrLen {
                lifetime, element, ..
            } => lifetime.is_some() || element.has_lifetime(),
            RustType::ImplExtendUninit(inner) => inner.has_lifetime(),
        }
    }

    /// Create a type from a name string. If the name matches a known primitive,
    /// returns `Primitive`; otherwise returns `Named`.
    pub fn named(name: impl Into<Cow<'static, str>>) -> Self {
        let name = name.into();
        match RustPrimitiveType::parse(&name) {
            Some(p) => RustType::Primitive(p),
            None => RustType::Named {
                name,
                lifetime: None,
            },
        }
    }

    /// Create a named type with a lifetime.
    pub fn named_with_lifetime(
        name: impl Into<Cow<'static, str>>,
        lifetime: impl Into<Cow<'static, str>>,
    ) -> Self {
        RustType::Named {
            name: name.into(),
            lifetime: Some(lifetime.into()),
        }
    }

    /// Check if this is a primitive or named type matching the given name string.
    pub fn is_named(&self, expected: &str) -> bool {
        match self {
            RustType::Primitive(p) => p.as_str() == expected,
            RustType::Named { name, .. } => &**name == expected,
            _ => false,
        }
    }

    /// Check if this is a `Primitive(Bool)`.
    pub fn is_bool(&self) -> bool {
        matches!(self, RustType::Primitive(RustPrimitiveType::Bool))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn named_simple() {
        assert_eq!(RustType::named("u32").to_tokens(), "u32");
    }

    #[test]
    fn named_resolves_to_primitive() {
        assert!(matches!(
            RustType::named("u32"),
            RustType::Primitive(RustPrimitiveType::U32)
        ));
        assert!(matches!(
            RustType::named("bool"),
            RustType::Primitive(RustPrimitiveType::Bool)
        ));
        assert!(matches!(
            RustType::named("c_void"),
            RustType::Primitive(RustPrimitiveType::CVoid)
        ));
    }

    #[test]
    fn named_custom_stays_named() {
        assert!(matches!(RustType::named("MyType"), RustType::Named { .. }));
    }

    #[test]
    fn named_with_lifetime() {
        assert_eq!(
            RustType::named_with_lifetime("Foo", "a").to_tokens(),
            "Foo<'a>"
        );
    }

    #[test]
    fn ref_immutable() {
        let ty = RustType::named("Foo").into_ref(Some("a".into()), false);
        assert_eq!(ty.to_tokens(), "&'a Foo");
    }

    #[test]
    fn ref_mutable() {
        let ty = RustType::named("Foo").into_ref(Some("a".into()), true);
        assert_eq!(ty.to_tokens(), "&'a mut Foo");
    }

    #[test]
    fn ref_no_lifetime() {
        let ty = RustType::named("Foo").into_ref(None, false);
        assert_eq!(ty.to_tokens(), "&Foo");
    }

    #[test]
    fn raw_ptr_const() {
        let ty = RustType::named("c_void").into_raw_ptr(false);
        assert_eq!(ty.to_tokens(), "*const c_void");
    }

    #[test]
    fn raw_ptr_mut() {
        let ty = RustType::named("c_void").into_raw_ptr(true);
        assert_eq!(ty.to_tokens(), "*mut c_void");
    }

    #[test]
    fn slice_immutable() {
        let ty = RustType::named("u32").into_slice(Some("a".into()), false);
        assert_eq!(ty.to_tokens(), "&'a [u32]");
    }

    #[test]
    fn slice_mutable() {
        let ty = RustType::named("u32").into_slice(Some("a".into()), true);
        assert_eq!(ty.to_tokens(), "&'a mut [u32]");
    }

    #[test]
    fn slice_no_lifetime() {
        let ty = RustType::named("u32").into_slice(None, false);
        assert_eq!(ty.to_tokens(), "&[u32]");
    }

    #[test]
    fn array() {
        let ty = RustType::Array {
            element: Box::new(RustType::named("f32")),
            len: "4".into(),
        };
        assert_eq!(ty.to_tokens(), "[f32; 4]");
    }

    #[test]
    fn array_named_len() {
        let ty = RustType::Array {
            element: Box::new(RustType::named("u8")),
            len: "MAX_NAME as usize".into(),
        };
        assert_eq!(ty.to_tokens(), "[u8; MAX_NAME as usize]");
    }

    #[test]
    fn option_type() {
        let ty = RustType::named("Foo").optional();
        assert_eq!(ty.to_tokens(), "Option<Foo>");
    }

    #[test]
    fn tuple_empty() {
        assert_eq!(RustType::Tuple(vec![]).to_tokens(), "()");
    }

    #[test]
    fn tuple_two() {
        let ty = RustType::Tuple(vec![
            RustType::named("u32"),
            RustType::Primitive(RustPrimitiveType::Bool),
        ]);
        assert_eq!(ty.to_tokens(), "(u32, bool)");
    }

    #[test]
    fn slice_or_len() {
        let ty = RustType::SliceOrLen {
            lifetime: Some("a".into()),
            element: Box::new(RustType::named("Foo")),
        };
        assert_eq!(ty.to_tokens(), "SliceOrLen<'a, Foo>");
    }

    #[test]
    fn slice_or_len_no_lifetime() {
        let ty = RustType::SliceOrLen {
            lifetime: None,
            element: Box::new(RustType::named("Foo")),
        };
        assert_eq!(ty.to_tokens(), "SliceOrLen<Foo>");
    }

    #[test]
    fn impl_extend_uninit() {
        let ty = RustType::ImplExtendUninit(Box::new(RustType::named("Foo")));
        assert_eq!(ty.to_tokens(), "impl ExtendUninit<Foo>");
    }

    #[test]
    fn bool_type() {
        assert_eq!(
            RustType::Primitive(RustPrimitiveType::Bool).to_tokens(),
            "bool"
        );
        // Also via named():
        assert_eq!(RustType::named("bool").to_tokens(), "bool");
    }

    #[test]
    fn unit_type() {
        assert_eq!(RustType::Unit.to_tokens(), "()");
    }

    #[test]
    fn nested_option_ref_slice() {
        let ty = RustType::named("Foo")
            .into_slice(Some("a".into()), false)
            .optional();
        assert_eq!(ty.to_tokens(), "Option<&'a [Foo]>");
    }

    #[test]
    fn has_lifetime_named() {
        assert!(!RustType::named("u32").has_lifetime());
        assert!(RustType::named_with_lifetime("Foo", "a").has_lifetime());
    }

    #[test]
    fn has_lifetime_ref() {
        let ty = RustType::named("Foo").into_ref(Some("a".into()), false);
        assert!(ty.has_lifetime());

        let ty = RustType::named("Foo").into_ref(None, false);
        assert!(!ty.has_lifetime());
    }

    #[test]
    fn has_lifetime_nested() {
        let ty = RustType::named_with_lifetime("Foo", "a").optional();
        assert!(ty.has_lifetime());

        let ty = RustType::named("Foo").optional();
        assert!(!ty.has_lifetime());
    }

    #[test]
    fn has_lifetime_raw_ptr() {
        let ty = RustType::named_with_lifetime("Foo", "a").into_raw_ptr(false);
        assert!(ty.has_lifetime());

        let ty = RustType::named("Foo").into_raw_ptr(false);
        assert!(!ty.has_lifetime());
    }

    #[test]
    fn is_named_works() {
        assert!(RustType::named("c_void").is_named("c_void"));
        assert!(RustType::named("Foo").is_named("Foo"));
        assert!(!RustType::named("Foo").is_named("Bar"));
    }

    #[test]
    fn is_bool_works() {
        assert!(RustType::named("bool").is_bool());
        assert!(RustType::Primitive(RustPrimitiveType::Bool).is_bool());
        assert!(!RustType::named("u32").is_bool());
    }
}
