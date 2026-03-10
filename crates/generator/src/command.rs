use std::io::Write;

use anyhow::Result;
use itertools::Itertools;

use crate::{
    LengthKind, analysis::Analysis, cdecl::CType, ctype_rust, ctype_to_rust_type, get_len_kind,
    handle::CommandType, normalize_command_name, normalize_name, normalize_param_name,
    normalize_ty_name, overrides, write_doc_link, xml,
};

pub struct CommandGroup<'a> {
    pub commands: Vec<CommandInfo<'a>>,
}

pub struct CommandInfo<'a> {
    pub command: &'a xml::Command,
    /// The name from the `<require>` element — may differ from `command.name` when this is an alias.
    pub required_name: &'a str,
    /// True when the `<require>` block has a `depends` attribute (entry point may be absent at runtime).
    pub conditionally_required: bool,
}

struct WrapperCommandInfo<'a> {
    // The original xml command
    command: &'a xml::Command,

    // The normalized command name
    name: String,

    // Info about functions that can either output a length or enumerate items
    enumeration_info: Option<EnumerationCommandInfo>,

    // Lifetime parameter for the command
    lifetime_param: Option<&'a str>,

    // Wrapper signature
    wrapper_params: Vec<WrapperParamInfo>,
    wrapper_return: WrapperReturnKind,

    // Original signature
    params: Vec<ParamInfo<'a>>,
    is_fallible: bool,
}

/// Info for the two-call enumeration pattern (vkEnumerate* / vkGet*).
struct EnumerationCommandInfo {
    /// Index into the command's parameter list for the count/length parameter.
    len_param_index: usize,
    /// Indices into the command's parameter list for the array output parameter(s).
    array_param_indices: Vec<usize>,
}

struct WrapperParamInfo {
    name: String,
    param_index: usize,
    /// Rust type used in the safe wrapper signature (not the raw C type).
    ty: String,
    is_enumeration_array: bool,
}

enum WrapperReturnKind {
    None,
    CommandReturnValue { ty: String },
    OutputParams(Vec<WrapperParamInfo>),
}

struct ParamInfo<'a> {
    name: String,
    param: &'a xml::CommandParam,
    len: Option<LengthKind<'a>>,
    /// (caller_may_pass_null, output_length_may_be_zero) — from the XML `optional` attribute.
    optional: (bool, bool),
    is_output_param: bool,
    is_return_param: bool,
    is_output_opaque_param: bool,
}

/// Describes how to emit a single argument in the generated FFI call.
#[derive(Debug, Clone)]
enum ArgEmitKind {
    /// Pass the wrapper param name as-is.
    Direct,
    /// Emit length from a slice param: `{slice_name}.len().try_into().unwrap()`
    LenFromSlice { slice_param_name: String },
    /// Slice/array param: `.as_ptr() as _` / `.as_mut_ptr() as _`, or optional `.to_raw_ptr()` / `.to_raw_mut_ptr()`
    SliceAsPtr { is_const: bool, optional: bool },
    /// Return (output) param: `{name}.as_mut_ptr()`
    ReturnParamAsMutPtr,
    /// Optional pointer: `.to_raw_ptr()` / `.to_raw_mut_ptr()`
    OptionalPtrToRaw { is_const: bool },
    /// Enumeration buffer param: `{name} as _`
    TransmuteForEnumeration,
    /// Bool param: `{name}.into()` (bool → Bool32)
    BoolInto,
}

impl<'a> LengthKind<'a> {
    /// Returns the C type of the parameter/field that specifies the length, if any.
    fn len_ctype(&self) -> Option<&CType<'a>> {
        match self {
            LengthKind::Param { c_decl, .. } => Some(&c_decl.ty),
            LengthKind::ParamField { field, .. } => Some(&field.c_decl.ty),
            _ => None,
        }
    }
}

/// True only for pointer-to-pointer C types (e.g. void**, T**), not single pointers (void*, T*).
fn is_pointer_to_pointer(ty: &CType) -> bool {
    matches!(
        ty,
        CType::Ptr {
            pointee,
            ..
        } if matches!(pointee.as_ref(), CType::Ptr { .. })
    )
}

pub fn generate_funcpointers(
    file: &mut impl Write,
    analysis: &Analysis,
    funcpointers: &[&xml::FuncPointer],
) -> Result<()> {
    for ty in funcpointers {
        write_doc_link(file, ty.name)?;
        writeln!(file, "pub type {} = unsafe extern \"system\" fn(", ty.name)?;
        for param in &ty.params {
            writeln!(
                file,
                "    {}: {},",
                normalize_name(param.c_decl.name),
                ctype_to_rust_type(analysis, &param.c_decl.ty, None)
            )?;
        }
        if let Some(ref return_type) = ty.return_type {
            writeln!(
                file,
                ") -> {};",
                ctype_to_rust_type(analysis, &return_type, None)
            )?;
        } else {
            writeln!(file, ");")?;
        }
    }
    writeln!(file)?;
    Ok(())
}

pub fn generate_functions(
    file: &mut impl Write,
    analysis: &Analysis,
    commands: &[&xml::Command],
) -> Result<()> {
    for command in commands {
        write_doc_link(file, command.name)?;
        writeln!(
            file,
            "pub type PFN_{} = unsafe extern \"system\" fn(",
            command.name
        )?;
        for param in &command.params {
            writeln!(
                file,
                "    {}: {},",
                normalize_name(param.c_decl.name),
                ctype_to_rust_type(analysis, &param.c_decl.ty, None)
            )?;
        }
        if let Some(ref return_type) = command.return_type {
            writeln!(
                file,
                ") -> {};",
                ctype_to_rust_type(analysis, &return_type, None)
            )?;
        } else {
            writeln!(file, ");")?;
        }
    }
    writeln!(file)?;
    Ok(())
}

pub fn generate_commands(
    file: &mut impl std::io::Write,
    analysis: &Analysis,
    requires: &[&xml::Require],
) -> Result<()> {
    let mut generate_commands = |cmd_type: CommandType, fn_type_name: &str| -> Result<()> {
        let command_groups: Vec<_> = requires
            .iter()
            .flat_map(|require| {
                let commands: Vec<_> = require
                    .commands
                    .iter()
                    .map(|req_cmd| {
                        let alias = analysis
                            .registry()
                            .command_aliases
                            .iter()
                            .find_map(|alias| {
                                if alias.name == req_cmd.name {
                                    Some(alias.alias)
                                } else {
                                    None
                                }
                            });
                        let name = alias.unwrap_or(req_cmd.name);
                        let command = analysis
                            .registry()
                            .commands
                            .iter()
                            .find(|cmd| cmd.name == name)
                            .unwrap();
                        CommandInfo {
                            required_name: req_cmd.name,
                            command,
                            conditionally_required: !require.depends.is_empty(),
                        }
                    })
                    .filter(|cmd| {
                        use overrides::CommandTypeOp;
                        match overrides::command_type_override(cmd.command.name) {
                            CommandTypeOp::Skip => false,
                            CommandTypeOp::Override(ty) => ty == cmd_type,
                            CommandTypeOp::Default => {
                                let ty = &cmd.command.params.iter().next().unwrap().c_decl.ty;
                                let category = ctype_rust::CTypeCategory::from_ctype(ty, analysis);
                                match category {
                                    ctype_rust::CTypeCategory::Base(name) => {
                                        analysis
                                            .handle_command_types()
                                            .get(name)
                                            .copied()
                                            .unwrap_or(CommandType::Entry)
                                            == cmd_type
                                    }
                                    _ => cmd_type == CommandType::Entry,
                                }
                            }
                        }
                    })
                    .collect();

                if commands.is_empty() {
                    None
                } else {
                    Some(CommandGroup { commands })
                }
            })
            .collect::<Vec<_>>();

        if command_groups.is_empty() {
            return Ok(());
        }

        writeln!(file, "pub struct {} {{", fn_type_name)?;
        for command_group in &command_groups {
            for command in &command_group.commands {
                let name = normalize_command_name(command.required_name);
                let ty = format!("PFN_{}", normalize_ty_name(command.command.name));
                let ty = if command.conditionally_required {
                    format!("Option<{}>", ty)
                } else {
                    ty
                };
                writeln!(file, "{}: {},", name, ty)?;
            }
        }
        writeln!(file, "}}\n")?;

        writeln!(file, "impl {} {{", fn_type_name)?;
        writeln!(
            file,
            "pub unsafe fn load(load: impl Fn(&CStr) -> Option<PFN_vkVoidFunction>) -> core::result::Result<Self, MissingEntryPointError> {{"
        )?;
        writeln!(file, "unsafe {{ Ok(Self {{")?;
        for command_group in &command_groups {
            for command in &command_group.commands {
                let name = normalize_command_name(command.required_name);
                if command.conditionally_required {
                    writeln!(
                        file,
                        "{}: transmute(load(c\"{}\")),",
                        name, command.required_name
                    )?;
                } else {
                    writeln!(
                        file,
                        "{}: transmute(load(c\"{}\").ok_or(MissingEntryPointError)?),",
                        name, command.required_name
                    )?;
                }
            }
        }
        writeln!(file, "}}) }} }} }}\n")?;

        writeln!(file, "impl {} {{", fn_type_name)?;
        for command_group in &command_groups {
            for command in &command_group.commands {
                write_command_wrapper(file, analysis, command)?;
            }
        }
        writeln!(file, "}}\n")?;
        Ok(())
    };

    generate_commands(CommandType::Entry, "EntryFn")?;
    generate_commands(CommandType::Instance, "InstanceFn")?;
    generate_commands(CommandType::Device, "DeviceFn")?;
    Ok(())
}

/// Detect enumeration (two-call) pattern: a length param that is a mutable pointer,
/// with one or more array params whose length refers to it.
fn compute_enumeration_info(
    analysis: &Analysis,
    len_kinds: &[Option<LengthKind<'_>>],
) -> Option<EnumerationCommandInfo> {
    let len_param_index = len_kinds
        .iter()
        .enumerate()
        .find_map(|(_param_index, len_kind)| {
            let LengthKind::Param {
                index: len_param_index,
                c_decl,
            } = len_kind.as_ref()?
            else {
                return None;
            };
            let category = ctype_rust::CTypeCategory::from_ctype(&c_decl.ty, analysis);
            match category {
                ctype_rust::CTypeCategory::TypedPointer { is_const, .. } if !is_const => {
                    Some(*len_param_index)
                }
                _ => None,
            }
        })?;

    let array_param_indices = len_kinds
        .iter()
        .enumerate()
        .filter_map(|(param_index, len_kind)| match len_kind {
            Some(LengthKind::Param { index, .. }) if *index == len_param_index => Some(param_index),
            _ => None,
        })
        .collect();

    Some(EnumerationCommandInfo {
        len_param_index,
        array_param_indices,
    })
}

/// Classify a single command parameter's output/return characteristics.
struct ParamClassification {
    is_output_param: bool,
    is_output_opaque_param: bool,
    is_return_param: bool,
}

fn classify_param(
    analysis: &Analysis,
    param: &xml::CommandParam,
    param_index: usize,
    len_kinds: &[Option<LengthKind<'_>>],
    _enumeration_info: &Option<EnumerationCommandInfo>,
    has_regular_return: bool,
) -> ParamClassification {
    let optional_0: bool = param
        .optional
        .first()
        .and_then(|s| s.parse().ok())
        .unwrap_or_default();

    let is_implicit_length = len_kinds
        .iter()
        .any(|len| matches!(len, Some(LengthKind::Param { index, .. }) if *index == param_index));

    let is_output_param = {
        let category = ctype_rust::CTypeCategory::from_ctype(&param.c_decl.ty, analysis);
        match category {
            ctype_rust::CTypeCategory::TypedPointer { is_const, pointee } => {
                if is_const || param.len.is_some() {
                    false
                } else if let CType::Base(base) = pointee {
                    !analysis.is_opaque_type_name(base.name)
                } else {
                    false
                }
            }
            _ => false,
        }
    };

    // Output opaque pointer only when C type is pointer-to-pointer (e.g. void**, OH_NativeBuffer**).
    // Single opaque pointers (void* pData) stay as *mut T or &mut [u8] when they have a length.
    let is_output_opaque_param = {
        let category = ctype_rust::CTypeCategory::from_ctype(&param.c_decl.ty, analysis);
        let non_const_ptr = match category {
            ctype_rust::CTypeCategory::OpaquePointer { is_const, .. } => !is_const,
            ctype_rust::CTypeCategory::TypedPointer { is_const, pointee } => {
                !is_const && analysis.is_opaque_type(pointee)
            }
            _ => false,
        };
        non_const_ptr && param.len.is_none() && is_pointer_to_pointer(&param.c_decl.ty)
    };

    // Extensible structs (with sType/pNext) must remain as &mut output parameters
    // so callers can pre-populate the pNext chain before calling.
    let is_extensible_output = is_output_param && {
        let category = ctype_rust::CTypeCategory::from_ctype(&param.c_decl.ty, analysis);
        matches!(
            category,
            ctype_rust::CTypeCategory::TypedPointer { pointee: CType::Base(base), .. }
                if analysis.is_extensible_struct(base.name)
        )
    };

    let is_return_param = (is_output_param || is_output_opaque_param)
        && !is_implicit_length
        && !has_regular_return
        && !is_extensible_output
        && !optional_0;

    ParamClassification {
        is_output_param,
        is_output_opaque_param,
        is_return_param,
    }
}

/// Build the wrapper return kind from collected return params and command info.
fn build_wrapper_return<'a>(
    analysis: &'a Analysis,
    command: &'a xml::Command,
    return_params: Vec<usize>,
    params: &[ParamInfo<'a>],
    has_regular_return: bool,
) -> WrapperReturnKind {
    if !return_params.is_empty() {
        let return_wrapper_params = return_params
            .into_iter()
            .map(|param_index| {
                let param = params[param_index].param;
                let CType::Ptr { pointee, .. } = &param.c_decl.ty else {
                    unreachable!()
                };
                let ty = if ctype_rust::is_bool32(&pointee) {
                    "bool".to_string()
                } else {
                    ctype_to_rust_type(analysis, &pointee, None)
                };
                WrapperParamInfo {
                    name: normalize_param_name(param.c_decl.name),
                    ty,
                    param_index,
                    is_enumeration_array: false,
                }
            })
            .collect();

        WrapperReturnKind::OutputParams(return_wrapper_params)
    } else if has_regular_return {
        let ret_ty = command.return_type.as_ref().unwrap();
        WrapperReturnKind::CommandReturnValue {
            ty: if ctype_rust::is_bool32(ret_ty) {
                "bool".to_string()
            } else {
                ctype_to_rust_type(analysis, ret_ty, None)
            },
        }
    } else {
        WrapperReturnKind::None
    }
}

fn analyze_command<'a>(analysis: &'a Analysis, info: &CommandInfo<'a>) -> WrapperCommandInfo<'a> {
    let command = info.command;
    let len_kinds: Vec<_> = command
        .params
        .iter()
        .map(|param| {
            param
                .len
                .map(|len| get_len_kind(analysis, &command.params, len))
        })
        .collect();

    let enumeration_info = compute_enumeration_info(analysis, &len_kinds);

    let is_fallible =
        matches!(&command.return_type, Some(CType::Base(base)) if base.name == "VkResult");

    let has_regular_return = command.return_type.is_some() && !is_fallible;

    let mut wrapper_params = Vec::new();
    let mut params = Vec::new();
    let mut return_params = Vec::new();
    let mut lifetime_param = None;

    if enumeration_info.is_some() {
        for param in &command.params {
            let type_info = analysis.get_type_info(&param.c_decl.ty).unwrap();
            if type_info.lifetime_param {
                lifetime_param = Some("a");
                break;
            }
        }
    }

    for (param_index, param) in command.params.iter().enumerate() {
        let name = normalize_param_name(param.c_decl.name);

        let optional: (bool, bool) = (
            param
                .optional
                .get(0)
                .and_then(|s| s.parse().ok())
                .unwrap_or_default(),
            param
                .optional
                .get(1)
                .and_then(|s| s.parse().ok())
                .unwrap_or_default(),
        );

        let is_implicit_length = len_kinds.iter().any(
            |len| matches!(len, Some(LengthKind::Param { index, .. }) if *index == param_index),
        );

        let classification = classify_param(
            analysis,
            param,
            param_index,
            &len_kinds,
            &enumeration_info,
            has_regular_return,
        );

        let is_enumeration_array = if let Some(enumeration_info) = &enumeration_info {
            enumeration_info.array_param_indices.contains(&param_index)
        } else {
            false
        };

        if !is_implicit_length {
            if classification.is_return_param {
                return_params.push(param_index);
            } else {
                let name = normalize_param_name(param.c_decl.name);
                let ty = convert_param_type(
                    analysis,
                    &param.c_decl.ty,
                    len_kinds[param_index].as_ref(),
                    optional,
                    lifetime_param,
                    classification.is_output_param || classification.is_output_opaque_param,
                );

                wrapper_params.push(WrapperParamInfo {
                    name: name.clone(),
                    param_index,
                    ty,
                    is_enumeration_array,
                });
            }
        }

        params.push(ParamInfo {
            name,
            param,
            len: len_kinds[param_index].clone(),
            optional,
            is_output_param: classification.is_output_param,
            is_return_param: classification.is_return_param,
            is_output_opaque_param: classification.is_output_opaque_param,
        });
    }

    let name = normalize_command_name(info.required_name);
    let wrapper_return = build_wrapper_return(
        analysis,
        command,
        return_params,
        &params,
        has_regular_return,
    );

    WrapperCommandInfo {
        command,
        name,
        enumeration_info,
        wrapper_params,
        wrapper_return,
        params,
        is_fallible,
        lifetime_param,
    }
}

/// Convert a C parameter type into the Rust type string used in the safe wrapper signature.
pub fn convert_param_type(
    analysis: &Analysis,
    ty: &CType,
    len: Option<&LengthKind<'_>>,
    optional: (bool, bool),
    lifetime_param: Option<&str>,
    is_output_param: bool,
) -> String {
    use ctype_rust::CTypeCategory;

    let has_non_literal_len = len
        .map(|l| !matches!(l, LengthKind::Literal(1)))
        .unwrap_or(false);

    // Parameter has a dynamic length → emit as a slice type (&[T], &mut [T], etc.)
    if has_non_literal_len {
        let category = CTypeCategory::from_ctype(ty, analysis);
        match category {
            CTypeCategory::CharPointer { is_const } => {
                let s = if is_const { "&CStr" } else { "&mut CStr" };
                let s = if optional.0 {
                    format!("Option<{}>", s)
                } else {
                    s.to_string()
                };
                return s;
            }
            CTypeCategory::OpaquePointer {
                pointee_name: "void",
                is_const,
            } => {
                // Writable void* with length-from-pointer (two-call pattern) → ExtendUninit<u8>
                if !is_const
                    && matches!(len.and_then(LengthKind::len_ctype), Some(CType::Ptr { .. }))
                {
                    return "impl ExtendUninit<u8>".to_string();
                }
                let s = if is_const { "&[u8]" } else { "&mut [u8]" };
                let s = if optional.0 {
                    format!("Option<{}>", s)
                } else {
                    s.to_string()
                };
                return s;
            }
            CTypeCategory::OpaquePointer {
                pointee_name: "char",
                is_const,
            } => {
                let s = if is_const {
                    "&[*const c_char]"
                } else {
                    "&mut [*mut c_char]"
                };
                let s = if optional.0 {
                    format!("Option<{}>", s)
                } else {
                    s.to_string()
                };
                return s;
            }
            CTypeCategory::OpaquePointer {
                pointee_name,
                is_const,
            } => {
                let inner =
                    ctype_rust::type_name_with_lifetime(analysis, pointee_name, lifetime_param);
                let s = if is_const {
                    format!("&[*const {}]", inner)
                } else {
                    format!("&mut [*mut {}]", inner)
                };
                let s = if optional.0 {
                    format!("Option<{}>", s)
                } else {
                    s
                };
                return s;
            }
            CTypeCategory::TypedPointer { is_const, pointee } => {
                let element_ty = ctype_to_rust_type(analysis, pointee, lifetime_param);
                let element_ty = if element_ty == "c_void" {
                    "u8".to_string()
                } else {
                    element_ty
                };
                let slice_ty = if is_const {
                    format!("&[{}]", element_ty)
                } else {
                    // Command-specific: length can be from a pointer (count param) → ExtendUninit
                    if matches!(len.and_then(LengthKind::len_ctype), Some(CType::Ptr { .. })) {
                        return format!("impl ExtendUninit<{}>", element_ty);
                    }
                    format!("&mut [{}]", element_ty)
                };
                let s = if optional.0 {
                    format!("Option<{}>", slice_ty)
                } else {
                    slice_ty
                };
                return s;
            }
            _ => panic!(
                "expected pointer type because of length {:?}, got {:?}",
                len, ty
            ),
        }
    }

    let category = CTypeCategory::from_ctype(ty, analysis);
    match category {
        CTypeCategory::OpaquePointer {
            is_const,
            pointee_name,
        } => {
            let inner = ctype_rust::type_name_with_lifetime(analysis, pointee_name, lifetime_param);
            let s = if is_const {
                format!("*const {}", inner)
            } else if is_output_param {
                format!("&mut *mut {}", inner)
            } else {
                format!("*mut {}", inner)
            };
            if optional.0 {
                format!("Option<{}>", s)
            } else {
                s
            }
        }
        CTypeCategory::CharPointer { is_const } => {
            let s = if is_const {
                "*const c_char".to_string()
            } else if is_output_param {
                "&mut *mut c_char".to_string()
            } else {
                "*mut c_char".to_string()
            };
            if optional.0 {
                format!("Option<{}>", s)
            } else {
                s
            }
        }
        CTypeCategory::TypedPointer { is_const, pointee } => {
            let ty = ctype_to_rust_type(analysis, pointee, lifetime_param);
            let is_opaque = analysis.is_opaque_type(pointee);
            let s = if is_opaque {
                if is_const {
                    format!("*const {}", ty)
                } else if is_output_param {
                    format!("&mut *mut {}", ty)
                } else {
                    format!("*mut {}", ty)
                }
            } else {
                if is_const {
                    format!("&{}", ty)
                } else {
                    format!("&mut {}", ty)
                }
            };
            if optional.0 {
                format!("Option<{}>", s)
            } else {
                s
            }
        }
        _ => {
            if ctype_rust::is_bool32(ty) {
                return "bool".to_string();
            }
            ctype_to_rust_type(analysis, ty, lifetime_param)
        }
    }
}

pub fn write_command_wrapper(
    file: &mut impl std::io::Write,
    analysis: &Analysis,
    info: &CommandInfo<'_>,
) -> Result<()> {
    if crate::overrides::write_command_override(
        file,
        info.required_name,
        info.conditionally_required,
    )? {
        return Ok(());
    }

    let wrapper = analyze_command(analysis, info);

    let lifetime_param = wrapper
        .lifetime_param
        .map(|lifetime| format!("<'{}>", lifetime))
        .unwrap_or_default();
    crate::write_doc_link(file, info.required_name)?;
    writeln!(
        file,
        "#[inline]
        pub unsafe fn {}{}(&self,",
        wrapper.name, lifetime_param
    )?;

    for param in &wrapper.wrapper_params {
        if param.is_enumeration_array {
            write!(file, "mut ")?;
        }
        writeln!(file, "{}: {},", param.name, param.ty)?;
    }

    let ok_codes_override = crate::overrides::ok_codes(info.required_name);
    let has_multiple_ok_codes = ok_codes_override
        .as_ref()
        .is_some_and(|o| o.codes.len() > 1);

    let return_ty = match &wrapper.wrapper_return {
        WrapperReturnKind::None => None,
        WrapperReturnKind::CommandReturnValue { ty } => Some(ty.to_string()),
        WrapperReturnKind::OutputParams(params) => Some(match params.as_slice() {
            [param] => param.ty.to_string(),
            params => format!(
                "({})",
                params.iter().map(|param| param.ty.as_str()).join(", ")
            ),
        }),
    };

    if wrapper.is_fallible {
        let inner = if has_multiple_ok_codes {
            let ok = ok_codes_override.as_ref().unwrap();
            let code_ty = match ok.repr {
                crate::overrides::SuccessCodeRepr::RawResult => "VkResult",
                crate::overrides::SuccessCodeRepr::Bool => "bool",
            };
            match return_ty.as_deref() {
                Some(ty) => format!("({}, {})", ty, code_ty),
                None => code_ty.to_string(),
            }
        } else {
            return_ty.as_deref().unwrap_or("()").to_string()
        };
        writeln!(file, ") -> crate::Result<{}> {{", inner)?;
    } else if let Some(return_ty) = return_ty {
        writeln!(file, ") -> {} {{", return_ty)?;
    } else {
        writeln!(file, ") {{")?;
    }

    writeln!(file, "unsafe {{")?;

    if let Some(enumeration_info) = &wrapper.enumeration_info {
        write_enumeration_fn_body(file, analysis, info, &wrapper, enumeration_info)?;
    } else {
        write_fn_body(
            file,
            analysis,
            &wrapper,
            info.conditionally_required,
            ok_codes_override.as_ref(),
            false,
        )?;
    }

    writeln!(file, "}} }}\n")?;
    Ok(())
}

fn write_enumeration_fn_body(
    file: &mut impl std::io::Write,
    analysis: &Analysis,
    info: &CommandInfo<'_>,
    wrapper: &WrapperCommandInfo<'_>,
    enumeration_info: &EnumerationCommandInfo,
) -> Result<()> {
    let len_param = &wrapper.params[enumeration_info.len_param_index].name;
    writeln!(file, "let call = |{len_param}, ")?;
    for param in &enumeration_info.array_param_indices {
        writeln!(file, "{}, ", wrapper.params[*param].name)?;
    }
    for wrapper_param in &wrapper.wrapper_params {
        let param = &wrapper.params[wrapper_param.param_index];
        if param.is_output_param
            && wrapper_param.param_index != enumeration_info.len_param_index
            && !param.is_return_param
        {
            writeln!(file, "{}: {}, ", wrapper_param.name, wrapper_param.ty)?;
        }
    }
    writeln!(file, "| {{ ")?;
    // Accept all declared success codes in the enumeration closure
    // (SUCCESS and INCOMPLETE) so the two-call pattern works.
    let enum_ok_codes = crate::overrides::OkCodes {
        codes: &wrapper.command.success_codes,
        repr: crate::overrides::SuccessCodeRepr::RawResult, // unused since in_enumeration=true
    };
    write_fn_body(
        file,
        analysis,
        wrapper,
        info.conditionally_required,
        Some(&enum_ok_codes),
        true,
    )?;
    writeln!(file, "}};")?;

    writeln!(file, "let mut len = 0; call(&mut len, ")?;
    for _ in &enumeration_info.array_param_indices {
        writeln!(file, "std::ptr::null_mut(), ")?;
    }
    for wrapper_param in &wrapper.wrapper_params {
        let param = &wrapper.params[wrapper_param.param_index];
        if param.is_output_param
            && wrapper_param.param_index != enumeration_info.len_param_index
            && !param.is_return_param
        {
            // Skip in the first call if optional
            if param.optional.0 {
                writeln!(file, "None, ")?;
            } else {
                writeln!(file, "{}, ", param.name)?;
            }
        }
    }
    if wrapper.is_fallible {
        writeln!(file, ")?;")?;
    } else {
        writeln!(file, ");")?;
    }
    writeln!(
        file,
        "let capacity = len.try_into().expect(\"failed to convert `N` to usize\");"
    )?;

    for (array_param_index, param) in enumeration_info.array_param_indices.iter().enumerate() {
        let param = &wrapper.params[*param];
        let param_name = &param.name;
        writeln!(
            file,
            "let {param_name}_buf = {param_name}.reserve(capacity);"
        )?;
        if array_param_index == 0 {
            writeln!(file, "len = {param_name}_buf.len().try_into().unwrap();")?;
        } else {
            let first_param = &wrapper.params[enumeration_info.array_param_indices[0]].name;
            writeln!(
                file,
                "assert_eq!({param_name}_buf.len(), {first_param}_buf.len());"
            )?;
        }
    }
    if wrapper.is_fallible {
        writeln!(file, "let result = ")?;
    }
    writeln!(file, "call(&mut len, ")?;
    for param in &enumeration_info.array_param_indices {
        let param = &wrapper.params[*param];
        let param_name = &param.name;
        writeln!(file, "{param_name}_buf.as_mut_ptr() as *mut _, ")?;
    }
    for wrapper_param in &wrapper.wrapper_params {
        let param = &wrapper.params[wrapper_param.param_index];
        if param.is_output_param
            && wrapper_param.param_index != enumeration_info.len_param_index
            && !param.is_return_param
        {
            writeln!(file, "{}, ", param.name)?;
        }
    }
    if wrapper.is_fallible {
        writeln!(file, ")?;")?;
    } else {
        writeln!(file, ");")?;
    }

    for param in &enumeration_info.array_param_indices {
        let param = &wrapper.params[*param];
        let param_name = &param.name;
        writeln!(file, "{param_name}.set_len(len.try_into().unwrap());")?;
    }
    if wrapper.is_fallible {
        writeln!(file, "Ok(result)")?;
    }
    Ok(())
}

/// Determine how to emit this parameter as an argument in the generated FFI call site.
fn arg_emit_kind(
    param_index: usize,
    param: &ParamInfo,
    wrapper: &WrapperCommandInfo,
    analysis: &Analysis,
) -> ArgEmitKind {
    let ty = &param.param.c_decl.ty;
    let category = ctype_rust::CTypeCategory::from_ctype(ty, analysis);
    // Is this param the length for another param (another param's len points to this index)?
    let is_length_for = wrapper.params.iter().any(|p| {
        matches!(
            &p.len,
            Some(LengthKind::Param { index, .. }) if *index == param_index
        )
    });
    let array_param_for_len = wrapper.params.iter().find(|p| {
        matches!(
            &p.len,
            Some(LengthKind::Param { index, .. }) if *index == param_index
        )
    });

    if is_length_for {
        // This param is a length; the "array" param (whose len we are) gets passed the slice.
        if matches!(ty, CType::Ptr { .. }) {
            return ArgEmitKind::Direct; // length param that is a pointer: pass through (e.g. two-call pattern)
        }
        if let Some(array_param) = array_param_for_len {
            return ArgEmitKind::LenFromSlice {
                slice_param_name: array_param.name.clone(),
            };
        }
    }

    if let Some(enumeration_info) = &wrapper.enumeration_info {
        if enumeration_info.array_param_indices.contains(&param_index) {
            return ArgEmitKind::TransmuteForEnumeration;
        }
    }

    if let Some(len) = &param.len {
        if !matches!(len, LengthKind::Literal(1)) {
            let is_const = match category {
                ctype_rust::CTypeCategory::TypedPointer { is_const, .. }
                | ctype_rust::CTypeCategory::CharPointer { is_const }
                | ctype_rust::CTypeCategory::OpaquePointer { is_const, .. } => is_const,
                _ => false,
            };
            return ArgEmitKind::SliceAsPtr {
                is_const,
                optional: param.optional.0,
            };
        }
    }

    if param.is_return_param {
        return ArgEmitKind::ReturnParamAsMutPtr;
    }

    if matches!(
        category,
        ctype_rust::CTypeCategory::TypedPointer { .. }
            | ctype_rust::CTypeCategory::OpaquePointer { .. }
            | ctype_rust::CTypeCategory::CharPointer { .. }
    ) && param.optional.0
        && !param.is_output_opaque_param
    {
        let is_const = match category {
            ctype_rust::CTypeCategory::TypedPointer { is_const, .. }
            | ctype_rust::CTypeCategory::CharPointer { is_const }
            | ctype_rust::CTypeCategory::OpaquePointer { is_const, .. } => is_const,
            _ => true,
        };
        return ArgEmitKind::OptionalPtrToRaw { is_const };
    }

    if ctype_rust::is_bool32(ty) {
        return ArgEmitKind::BoolInto;
    }

    ArgEmitKind::Direct
}

fn emit_arg(file: &mut impl std::io::Write, param_name: &str, kind: ArgEmitKind) -> Result<()> {
    match kind {
        ArgEmitKind::Direct => {
            writeln!(file, "{},", param_name)?;
        }
        ArgEmitKind::LenFromSlice { slice_param_name } => {
            writeln!(file, "{}.len().try_into().unwrap(),", slice_param_name)?;
        }
        ArgEmitKind::SliceAsPtr { is_const, optional } => {
            if optional {
                if is_const {
                    writeln!(file, "{}.to_raw_ptr(),", param_name)?;
                } else {
                    writeln!(file, "{}.to_raw_mut_ptr(),", param_name)?;
                }
            } else {
                if is_const {
                    writeln!(file, "{}.as_ptr() as _,", param_name)?;
                } else {
                    writeln!(file, "{}.as_mut_ptr() as _,", param_name)?;
                }
            }
        }
        ArgEmitKind::ReturnParamAsMutPtr => {
            writeln!(file, "{}.as_mut_ptr(),", param_name)?;
        }
        ArgEmitKind::OptionalPtrToRaw { is_const, .. } => {
            if is_const {
                writeln!(file, "{}.to_raw_ptr(),", param_name)?;
            } else {
                writeln!(file, "{}.to_raw_mut_ptr(),", param_name)?;
            }
        }
        ArgEmitKind::TransmuteForEnumeration => {
            writeln!(file, "{} as _,", param_name)?;
        }
        ArgEmitKind::BoolInto => {
            writeln!(file, "{}.into(),", param_name)?;
        }
    }
    Ok(())
}

fn write_fn_body(
    file: &mut impl std::io::Write,
    analysis: &Analysis,
    wrapper: &WrapperCommandInfo,
    conditionally_required: bool,
    ok_codes: Option<&crate::overrides::OkCodes>,
    in_enumeration: bool,
) -> Result<()> {
    if let WrapperReturnKind::OutputParams(params) = &wrapper.wrapper_return {
        for param in params {
            writeln!(
                file,
                "let mut {} = core::mem::MaybeUninit::uninit();",
                param.name
            )?;
        }
    }

    if wrapper.is_fallible {
        writeln!(file, "let result = ")?;
    }

    if conditionally_required {
        writeln!(file, "(self.{}.unwrap())(", wrapper.name)?;
    } else {
        writeln!(file, "(self.{})(", wrapper.name)?;
    }

    for (param_index, param) in wrapper.params.iter().enumerate() {
        let kind = arg_emit_kind(param_index, param, wrapper, analysis);
        emit_arg(file, &param.name, kind)?;
    }
    if matches!(&wrapper.wrapper_return, WrapperReturnKind::CommandReturnValue { ty } if ty == "bool")
    {
        writeln!(file, ") != 0")?;
    } else {
        writeln!(file, ")")?;
    }

    let return_value = match &wrapper.wrapper_return {
        WrapperReturnKind::OutputParams(params) => Some(match params.as_slice() {
            [param] => {
                let init = format!("{}.assume_init()", param.name);
                if param.ty == "bool" {
                    format!("{} != 0", init)
                } else {
                    init
                }
            }
            params => format!(
                "({})",
                params
                    .iter()
                    .map(|param| {
                        let init = format!("{}.assume_init()", param.name);
                        if param.ty == "bool" {
                            format!("{} != 0", init)
                        } else {
                            init
                        }
                    })
                    .join(", ")
            ),
        }),
        _ => None,
    };

    if wrapper.is_fallible {
        let default_codes: &[&str] = &["VK_SUCCESS"];
        let codes = ok_codes.map(|o| o.codes).unwrap_or(default_codes);

        // Expose the success code when the override provides multiple ok codes,
        // but not in enumeration closures (which accept INCOMPLETE silently).
        let expose_success_code = !in_enumeration && ok_codes.is_some_and(|o| o.codes.len() > 1);

        writeln!(file, ";\n")?;
        writeln!(file, "match result {{")?;

        // Multiple success codes: expose which code was returned alongside the output value.
        if expose_success_code {
            let ok_codes = ok_codes.unwrap();
            for (i, code) in codes.iter().enumerate() {
                let code_value = match ok_codes.repr {
                    crate::overrides::SuccessCodeRepr::RawResult => "result".to_string(),
                    crate::overrides::SuccessCodeRepr::Bool => {
                        if i == 0 {
                            "false".to_string()
                        } else {
                            "true".to_string()
                        }
                    }
                };
                let ok_value = match return_value.as_deref() {
                    Some(rv) => format!("({}, {})", rv, code_value),
                    None => code_value,
                };
                writeln!(
                    file,
                    "VkResult::{} => Ok({}),",
                    code.strip_prefix("VK_").unwrap_or(code),
                    ok_value,
                )?;
            }
        } else {
            let ok_value = return_value.as_deref().unwrap_or("()");
            for code in codes {
                writeln!(
                    file,
                    "VkResult::{} => Ok({}),",
                    code.strip_prefix("VK_").unwrap_or(code),
                    ok_value,
                )?;
            }
        }

        writeln!(file, "err => Err(err),")?;
        writeln!(file, "}}")?;
    } else if let Some(return_value) = return_value {
        writeln!(file, ";")?;
        writeln!(file, "{}", return_value)?;
    };
    Ok(())
}
