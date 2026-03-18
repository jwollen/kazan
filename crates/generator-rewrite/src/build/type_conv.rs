//! Unified C-to-Rust type conversion, parameter analysis, and length classification.
//!
//! Provides `convert_type` (the main entry point for producing `RustType` values),
//! `resolve_ctype` (raw C-layout conversion), and `LengthKind` / `get_len_kind`
//! for classifying Vulkan `len` attributes.

use std::borrow::Cow;

use crate::{
    analysis::Analysis,
    cdecl::{self, CArrayLen, CType},
    r#ctype::{self, CTypeCategory},
    model::rust_type::{RustPrimitiveType, RustType},
    normalize_const_name, xml,
};

// ── Length / parameter analysis ─────────────────────────────────────────────

/// Classification of a Vulkan `len` attribute (e.g. `"null-terminated"`, `"2"`, `"dataSize"`).
#[derive(Clone, Debug)]
pub enum LengthKind<'a> {
    NullTerminated,
    Literal(u32),
    /// Length given by another parameter in the same command/struct.
    Param {
        /// Index of the length parameter in the parameter list.
        index: usize,
        c_decl: &'a cdecl::CDecl<'static>,
    },
    /// Length given by a field of a struct pointed to by another parameter (e.g. `pCreateInfo->count`).
    ParamField {
        field: &'a xml::StructureMember,
    },
    /// Unrecognized length expression — treated as no length info.
    Unknown,
}

impl<'a> LengthKind<'a> {
    /// Returns the C type of the parameter/field that specifies the length, if any.
    pub(crate) fn len_ctype(&self) -> Option<&CType<'a>> {
        match self {
            LengthKind::Param { c_decl, .. } => Some(&c_decl.ty),
            LengthKind::ParamField { field, .. } => Some(&field.c_decl.ty),
            _ => None,
        }
    }
}

pub(crate) trait Param {
    fn c_decl(&self) -> &cdecl::CDecl<'static>;
}

impl Param for xml::CommandParam {
    fn c_decl(&self) -> &cdecl::CDecl<'static> {
        &self.c_decl
    }
}

impl Param for xml::StructureMember {
    fn c_decl(&self) -> &cdecl::CDecl<'static> {
        &self.c_decl
    }
}

pub(crate) fn get_param_index(params: &[impl Param], param_name: &str) -> Option<usize> {
    params.iter().enumerate().find_map(|(index, other_param)| {
        if other_param.c_decl().name == param_name {
            Some(index)
        } else {
            None
        }
    })
}

/// Classify a `len` attribute string (e.g. `"null-terminated"`, `"2"`, `"pCreateInfo->count"`)
/// into a `LengthKind`.
pub(crate) fn get_len_kind<'a>(
    analysis: &'a Analysis,
    params: &'a [impl Param],
    len: &'static str,
) -> LengthKind<'a> {
    if len == "null-terminated" {
        LengthKind::NullTerminated
    } else if let Ok(len) = len.parse() {
        LengthKind::Literal(len)
    // Length via struct field dereference (e.g. "pAllocateInfo->commandBufferCount"):
    // resolve the struct the parameter points to, then find the named field.
    } else if let Some((param_name, field_name)) = len.split_once("->")
        && let Some(index) = get_param_index(params, param_name)
    {
        let param = &params[index];
        let param_ty = &param.c_decl().ty;
        let CType::Ptr { pointee, .. } = param_ty else {
            panic!("expected pointer type, got {param_ty:?}");
        };
        let CType::Base(base) = pointee.as_ref() else {
            panic!("expected base type, got {pointee:?}");
        };

        let struct_ty = analysis
            .registry()
            .structs
            .iter()
            .find(|ty| ty.name == base.name)
            .unwrap_or_else(|| panic!("failed to find struct {}", base.name));

        let field = struct_ty
            .members
            .iter()
            .find(|field| field.c_decl.name == field_name)
            .unwrap_or_else(|| panic!("failed to find field {field_name}"));

        LengthKind::ParamField { field }
    } else if let Some(index) = get_param_index(params, len) {
        let param = &params[index];
        LengthKind::Param {
            index,
            c_decl: param.c_decl(),
        }
    } else {
        LengthKind::Unknown
    }
}

// ── Array parameter classification ──────────────────────────────────────────

/// How a nullable array parameter should be represented in the wrapper signature.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ArrayParamKind {
    /// Regular slice (required `&[T]` or optional `Option<&[T]>`).
    Standard,
    /// `SliceOrLen<'a, T>` — length meaningful even without data.
    SliceOrLen,
    /// `Option<SliceOrLen<'a, T>>` — None maps to count=0, ptr=null.
    OptionSliceOrLen,
}

// ── Type conversion ─────────────────────────────────────────────────────────

/// Describes the role a type plays, providing the context needed for conversion.
pub(crate) enum TypeRole<'a> {
    /// Command wrapper parameter (input or output-passed-by-caller).
    CommandParam {
        len: Option<&'a LengthKind<'a>>,
        optional: (bool, bool),
        nullable: bool,
        lifetime: Option<&'a str>,
        is_output: bool,
        array_kind: ArrayParamKind,
    },
    /// Struct setter parameter.
    SetterParam {
        lengths: &'a [LengthKind<'a>],
        optional: &'a [bool],
        lifetime: Option<&'a str>,
    },
}

/// Convert a C type to its Rust representation based on its role.
pub(crate) fn convert_type(analysis: &Analysis, ty: &CType, role: &TypeRole) -> RustType {
    match role {
        TypeRole::CommandParam { .. } => convert_command_param(analysis, ty, role),
        TypeRole::SetterParam { .. } => convert_setter_param(analysis, ty, role),
    }
}

// ── Helpers ──────────────────────────────────────────────────────────────────

/// Resolve a base C type name to a `RustType`.
///
/// Mirrors `ctype::type_name_with_lifetime` but returns structured `RustType`.
fn resolve_base_type(analysis: &Analysis, name: &str, lifetime: Option<&str>) -> RustType {
    let rust_name = ctype::base_ctype_to_rust_str(name);
    // Fast path: primitives avoid allocation entirely.
    if let Some(prim) = RustPrimitiveType::parse(rust_name) {
        return RustType::Primitive(prim);
    }
    let type_info = analysis.get_base_type_info(name);
    match type_info {
        Some(info) if info.lifetime_param => RustType::named_with_lifetime(
            rust_name.to_string(),
            lifetime.unwrap_or("_").to_string(),
        ),
        _ => RustType::Named {
            name: Cow::Owned(rust_name.to_string()),
            lifetime: None,
        },
    }
}

/// Convert a full `CType` to `RustType` (raw C-layout representation).
///
/// Mirrors `ctype_to_rust_type` in lib.rs but returns `RustType`.
pub(crate) fn resolve_ctype(analysis: &Analysis, ty: &CType, lifetime: Option<&str>) -> RustType {
    match ty {
        CType::Base(base) => resolve_base_type(analysis, base.name, lifetime),
        CType::Ptr {
            pointee, is_const, ..
        } => {
            let inner = resolve_ctype(analysis, pointee, lifetime);
            RustType::RawPtr {
                mutable: !is_const,
                inner: Box::new(inner),
            }
        }
        CType::Array { element, len } => {
            let is_char = matches!(element.as_ref(), CType::Base(b) if b.name == "char");
            if is_char {
                let s = match len {
                    CArrayLen::Named(name) => {
                        format!("ArrayCStr<{{ {} as usize }}>", normalize_const_name(name))
                    }
                    CArrayLen::Literal(n) => format!("ArrayCStr<{{ {n} }}>"),
                };
                RustType::named(s)
            } else {
                let element_ty = resolve_ctype(analysis, element, lifetime);
                let len_str = match len {
                    CArrayLen::Named(name) => {
                        format!("{} as usize", normalize_const_name(name))
                    }
                    CArrayLen::Literal(n) => n.to_string(),
                };
                RustType::Array {
                    element: Box::new(element_ty),
                    len: len_str.into(),
                }
            }
        }
        CType::Func { .. } => todo!(),
    }
}

// ── Command parameter conversion ─────────────────────────────────────────────

fn convert_command_param(analysis: &Analysis, ty: &CType, role: &TypeRole) -> RustType {
    let TypeRole::CommandParam {
        len,
        optional,
        nullable,
        lifetime,
        is_output,
        array_kind,
    } = role
    else {
        unreachable!()
    };

    let has_non_literal_len = len
        .map(|l| !matches!(l, LengthKind::Literal(1)))
        .unwrap_or(false);

    if has_non_literal_len {
        return convert_command_param_with_length(
            analysis,
            ty,
            *len,
            *nullable,
            *lifetime,
            *array_kind,
        );
    }

    let category = CTypeCategory::from_ctype(ty, analysis);
    match category {
        CTypeCategory::OpaquePointer {
            is_const,
            pointee_name,
        } => {
            let inner = resolve_base_type(analysis, pointee_name, *lifetime);
            let rt = if is_const {
                inner.into_raw_ptr(false)
            } else if *is_output {
                inner.into_raw_ptr(true).into_ref(None, true)
            } else {
                inner.into_raw_ptr(true)
            };
            if optional.0 { rt.optional() } else { rt }
        }
        CTypeCategory::CharPointer { is_const } => {
            let rt = if is_const {
                RustType::C_CHAR.into_raw_ptr(false)
            } else if *is_output {
                RustType::C_CHAR.into_raw_ptr(true).into_ref(None, true)
            } else {
                RustType::C_CHAR.into_raw_ptr(true)
            };
            if optional.0 { rt.optional() } else { rt }
        }
        CTypeCategory::TypedPointer { is_const, pointee } => {
            let inner = resolve_ctype(analysis, pointee, *lifetime);
            let is_opaque = analysis.is_opaque_type(pointee);
            let rt = if is_opaque {
                if is_const {
                    inner.into_raw_ptr(false)
                } else if *is_output {
                    inner.into_raw_ptr(true).into_ref(None, true)
                } else {
                    inner.into_raw_ptr(true)
                }
            } else if is_const {
                inner.into_ref(None, false)
            } else {
                inner.into_ref(None, true)
            };
            if optional.0 { rt.optional() } else { rt }
        }
        _ => {
            if ctype::is_bool32(ty) {
                return RustType::BOOL;
            }
            resolve_ctype(analysis, ty, *lifetime)
        }
    }
}

fn convert_command_param_with_length(
    analysis: &Analysis,
    ty: &CType,
    len: Option<&LengthKind>,
    nullable: bool,
    lifetime: Option<&str>,
    array_kind: ArrayParamKind,
) -> RustType {
    let category = CTypeCategory::from_ctype(ty, analysis);
    match category {
        CTypeCategory::CharPointer { is_const } => {
            let rt = RustType::CSTR.into_ref(None, !is_const);
            if nullable { rt.optional() } else { rt }
        }
        CTypeCategory::OpaquePointer {
            pointee_name: "void",
            is_const,
        } => {
            // Writable void* with length-from-pointer (two-call pattern) → ExtendUninit<u8>
            if !is_const && matches!(len.and_then(LengthKind::len_ctype), Some(CType::Ptr { .. })) {
                return RustType::ImplExtendUninit(Box::new(RustType::U8));
            }
            let rt = RustType::U8.into_slice(None, !is_const);
            if nullable { rt.optional() } else { rt }
        }
        CTypeCategory::OpaquePointer {
            pointee_name: "char",
            is_const,
        } => {
            let element = RustType::C_CHAR.into_raw_ptr(!is_const);
            let rt = element.into_slice(None, !is_const);
            if nullable { rt.optional() } else { rt }
        }
        CTypeCategory::OpaquePointer {
            pointee_name,
            is_const,
        } => {
            let inner = resolve_base_type(analysis, pointee_name, lifetime);
            let element = inner.into_raw_ptr(!is_const);
            let rt = element.into_slice(None, !is_const);
            if nullable { rt.optional() } else { rt }
        }
        CTypeCategory::TypedPointer { is_const, pointee } => {
            let mut element = resolve_ctype(analysis, pointee, lifetime);
            // c_void → u8 for slice elements
            if element.is_named("c_void") {
                element = RustType::U8;
            }

            // SliceOrLen variants for nullable array params with meaningful count.
            match array_kind {
                ArrayParamKind::SliceOrLen => {
                    let lt: Cow<'static, str> = lifetime
                        .map(|l| Cow::Owned(l.to_string()))
                        .unwrap_or(Cow::Borrowed("_"));
                    return RustType::SliceOrLen {
                        lifetime: Some(lt),
                        element: Box::new(element),
                    };
                }
                ArrayParamKind::OptionSliceOrLen => {
                    let lt: Cow<'static, str> = lifetime
                        .map(|l| Cow::Owned(l.to_string()))
                        .unwrap_or(Cow::Borrowed("_"));
                    return RustType::SliceOrLen {
                        lifetime: Some(lt),
                        element: Box::new(element),
                    }
                    .optional();
                }
                ArrayParamKind::Standard => {}
            }

            if !is_const {
                // Writable pointer with pointer-length → ExtendUninit
                if matches!(len.and_then(LengthKind::len_ctype), Some(CType::Ptr { .. })) {
                    return RustType::ImplExtendUninit(Box::new(element));
                }
            }
            let rt = element.into_slice(None, !is_const);
            if nullable { rt.optional() } else { rt }
        }
        _ => panic!("expected pointer type because of length {len:?}, got {ty:?}"),
    }
}

// ── Setter parameter conversion ──────────────────────────────────────────────

fn convert_setter_param(analysis: &Analysis, ty: &CType, role: &TypeRole) -> RustType {
    let TypeRole::SetterParam {
        lengths,
        optional,
        lifetime,
    } = role
    else {
        unreachable!()
    };

    if let Some(len) = lengths.first()
        && !matches!(len, LengthKind::Literal(1))
    {
        return convert_setter_param_with_length(analysis, ty, lengths, optional, *lifetime);
    }

    let category = CTypeCategory::from_ctype(ty, analysis);
    match category {
        CTypeCategory::OpaquePointer {
            is_const,
            pointee_name,
        } => {
            let inner = resolve_base_type(analysis, pointee_name, *lifetime);
            if is_const {
                inner.into_raw_ptr(false)
            } else {
                inner.into_raw_ptr(true)
            }
        }
        CTypeCategory::CharPointer { is_const } => {
            if is_const {
                RustType::C_CHAR.into_raw_ptr(false)
            } else {
                RustType::C_CHAR.into_raw_ptr(true)
            }
        }
        CTypeCategory::TypedPointer { is_const, pointee } => {
            let inner = resolve_ctype(analysis, pointee, *lifetime);
            let is_opaque = analysis.is_opaque_type(pointee);
            if is_opaque {
                if is_const {
                    inner.into_raw_ptr(false)
                } else {
                    inner.into_raw_ptr(true)
                }
            } else if is_const {
                inner.into_ref(Some("a".into()), false)
            } else {
                inner.into_ref(Some("a".into()), true)
            }
        }
        _ => {
            if ctype::is_bool32(ty) {
                return RustType::BOOL;
            }
            resolve_ctype(analysis, ty, *lifetime)
        }
    }
}

fn convert_setter_param_with_length(
    analysis: &Analysis,
    ty: &CType,
    lengths: &[LengthKind],
    optional: &[bool],
    _lifetime: Option<&str>,
) -> RustType {
    let category = CTypeCategory::from_ctype(ty, analysis);
    match category {
        CTypeCategory::CharPointer { is_const } => {
            RustType::CSTR.into_ref(Some("a".into()), !is_const)
        }
        CTypeCategory::OpaquePointer {
            pointee_name: "char",
            is_const,
        } => {
            let element = RustType::C_CHAR.into_raw_ptr(!is_const);
            element.into_slice(Some("a".into()), !is_const)
        }
        CTypeCategory::OpaquePointer {
            pointee_name: "void",
            is_const,
        } => RustType::U8.into_slice(Some("a".into()), !is_const),
        CTypeCategory::OpaquePointer {
            pointee_name,
            is_const,
        } => {
            // Check if this is T** where T is a known non-opaque struct → &[&T]
            let use_slice_of_refs = matches!(
                ty,
                CType::Ptr { pointee: p, .. }
                    if matches!(p.as_ref(),
                        CType::Ptr { pointee: inner, .. }
                            if matches!(inner.as_ref(),
                                CType::Base(b)
                                    if !analysis.is_opaque_type_name(b.name)
                                       && b.name == pointee_name
                            )
                    )
            );
            let inner = resolve_base_type(analysis, pointee_name, None);
            if use_slice_of_refs {
                let element = inner.into_ref(Some("a".into()), !is_const);
                element.into_slice(Some("a".into()), !is_const)
            } else {
                let element = inner.into_raw_ptr(!is_const);
                element.into_slice(Some("a".into()), !is_const)
            }
        }
        CTypeCategory::TypedPointer { is_const, pointee } => {
            let rest_lengths = if lengths.len() > 1 {
                &lengths[1..]
            } else {
                &[] as &[LengthKind]
            };
            let rest_optional = if optional.len() > 1 {
                &optional[1..]
            } else {
                &[] as &[bool]
            };
            // Recurse for the element type (handles nested pointers).
            let mut element = convert_type(
                analysis,
                pointee,
                &TypeRole::SetterParam {
                    lengths: rest_lengths,
                    optional: rest_optional,
                    lifetime: None,
                },
            );
            // Bool32 array elements must stay as Bool32, not bool (different memory layout).
            if element.is_bool() {
                element = RustType::BOOL32;
            }
            // c_void → u8 for slice elements.
            if element.is_named("c_void") {
                element = RustType::U8;
            }
            element.into_slice(Some("a".into()), !is_const)
        }
        CTypeCategory::Array { element, .. } => {
            let element_ty = resolve_ctype(analysis, element, None);
            element_ty.into_slice(None, false)
        }
        _ => panic!("expected pointer or array type because of length, got {ty:?}"),
    }
}
