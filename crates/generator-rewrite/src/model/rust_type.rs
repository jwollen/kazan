/// Strongly typed representation of every Rust type the generator can produce.
///
/// This replaces all ad-hoc `format!("&[{ty}]")` / `format!("Option<{s}>")` patterns
/// with a single inspectable data structure that owns the full type tree.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RustType {
    /// Named type: `u32`, `Foo`, `c_void`, `Bool32`, etc.
    Named {
        name: String,
        lifetime: Option<String>,
    },
    /// `&T` or `&mut T`.
    Ref {
        lifetime: Option<String>,
        mutable: bool,
        inner: Box<RustType>,
    },
    /// `*const T` or `*mut T`.
    RawPtr { mutable: bool, inner: Box<RustType> },
    /// `&[T]` or `&mut [T]`.
    Slice {
        lifetime: Option<String>,
        mutable: bool,
        element: Box<RustType>,
    },
    /// `[T; N]`.
    Array { element: Box<RustType>, len: String },
    /// `Option<T>`.
    Option(Box<RustType>),
    /// `(A, B, ...)`.
    Tuple(Vec<RustType>),
    /// `SliceOrLen<'a, T>`.
    SliceOrLen {
        lifetime: Option<String>,
        element: Box<RustType>,
    },
    /// `impl ExtendUninit<T>`.
    ImplExtendUninit(Box<RustType>),
    /// Semantic `bool` (not Bool32).
    Bool,
    /// `()`.
    Unit,
}

impl RustType {
    /// Render this type to its Rust source representation.
    pub fn to_tokens(&self) -> String {
        match self {
            RustType::Named { name, lifetime } => match lifetime {
                Some(lt) => format!("{name}<'{lt}>"),
                None => name.clone(),
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
            RustType::Bool => "bool".to_string(),
            RustType::Unit => "()".to_string(),
        }
    }

    /// Wrap this type in `Option<_>`.
    pub fn optional(self) -> Self {
        RustType::Option(Box::new(self))
    }

    /// Convert to a slice type `&[T]` or `&mut [T]`.
    pub fn into_slice(self, lifetime: Option<String>, mutable: bool) -> Self {
        RustType::Slice {
            lifetime,
            mutable,
            element: Box::new(self),
        }
    }

    /// Convert to a reference type `&T` or `&mut T`.
    pub fn into_ref(self, lifetime: Option<String>, mutable: bool) -> Self {
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
            RustType::Bool | RustType::Unit => false,
        }
    }

    /// Create a named type with no lifetime.
    pub fn named(name: impl Into<String>) -> Self {
        RustType::Named {
            name: name.into(),
            lifetime: None,
        }
    }

    /// Create a named type with a lifetime.
    pub fn named_with_lifetime(name: impl Into<String>, lifetime: impl Into<String>) -> Self {
        RustType::Named {
            name: name.into(),
            lifetime: Some(lifetime.into()),
        }
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
        let ty = RustType::Tuple(vec![RustType::named("u32"), RustType::Bool]);
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
        assert_eq!(RustType::Bool.to_tokens(), "bool");
    }

    #[test]
    fn unit_type() {
        assert_eq!(RustType::Unit.to_tokens(), "()");
    }

    #[test]
    fn nested_option_ref_slice() {
        // Option<&'a [Foo]>
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
        // Option<Foo<'a>> has a lifetime
        let ty = RustType::named_with_lifetime("Foo", "a").optional();
        assert!(ty.has_lifetime());

        // Option<Foo> does not
        let ty = RustType::named("Foo").optional();
        assert!(!ty.has_lifetime());
    }

    #[test]
    fn has_lifetime_raw_ptr() {
        // *const Foo<'a>
        let ty = RustType::named_with_lifetime("Foo", "a").into_raw_ptr(false);
        assert!(ty.has_lifetime());

        // *const Foo
        let ty = RustType::named("Foo").into_raw_ptr(false);
        assert!(!ty.has_lifetime());
    }
}
