use crate::{
    LengthKind, analysis::Analysis, cdecl::CType, ctype_rust, ctype_to_rust_type, get_len_kind,
    handle::CommandType, normalize_command_name, normalize_param_name, normalize_ty_name, xml,
};
use itertools::Itertools;

pub struct CommandGroup<'a> {
    pub require: &'a xml::Require,
    pub commands: Vec<CommandInfo<'a>>,
}

pub struct CommandInfo<'a> {
    pub command: &'a xml::Command,
    pub alias: &'a str,
    pub optional: bool,
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
    wrapper_params: Vec<WrapperParamInfo<'a>>,
    wrapper_return: WrapperReturnKind<'a>,

    // Original signature
    params: Vec<ParamInfo<'a>>,
    is_fallible: bool,
    has_regular_return: bool,
}

struct EnumerationCommandInfo {
    len_param: usize,
    array_params: Vec<usize>,
}

struct WrapperParamInfo<'a> {
    name: String,
    param_index: usize,
    param: &'a xml::CommandParam,
    ty: String,
    is_enumeration_array: bool,
}

enum WrapperReturnKind<'a> {
    None,
    CommandReturnValue { ty: String },
    OutputParams(Vec<WrapperParamInfo<'a>>),
}

struct ParamInfo<'a> {
    name: String,
    param: &'a xml::CommandParam,
    ty: String,
    len: Option<LengthKind<'a>>,
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
    /// Optional pointer: `.to_raw_ptr()` / `.to_raw_mut_ptr()`, or todo! when in enumeration command
    OptionalPtrToRaw {
        is_const: bool,
        in_enumeration: bool,
    },
    /// Enumeration buffer param: `{name} as _`
    TransmuteForEnumeration,
}

impl<'a> LengthKind<'a> {
    fn ty(&self) -> Option<&CType<'a>> {
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

pub fn generate_commands(
    file: &mut impl std::io::Write,
    analysis: &Analysis,
    requires: &[&xml::Require],
) {
    let mut generate_commands = |cmd_type: CommandType, fn_type_name: &str| {
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
                            alias: req_cmd.name,
                            command,
                            optional: !require.depends.is_empty(),
                        }
                    })
                    .filter(|cmd| {
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
                    })
                    .collect();

                if commands.is_empty() {
                    None
                } else {
                    Some(CommandGroup { require, commands })
                }
            })
            .collect::<Vec<_>>();

        if command_groups.is_empty() {
            return;
        }

        writeln!(file, "pub struct {} {{", fn_type_name).unwrap();
        for command_group in &command_groups {
            for command in &command_group.commands {
                let name = normalize_command_name(command.alias);
                let ty = format!("PFN_{}", normalize_ty_name(command.command.name));
                let ty = if command.optional {
                    format!("Option<{}>", ty)
                } else {
                    ty
                };
                writeln!(file, "{}: {},", name, ty).unwrap();
            }
        }
        writeln!(file, "}}").unwrap();

        writeln!(file, "impl {} {{", fn_type_name).unwrap();
        writeln!(file, "pub unsafe fn load(load: impl Fn(&CStr) -> Option<PFN_vkVoidFunction>) -> core::result::Result<Self, MissingEntryPointError> {{").unwrap();
        writeln!(file, "unsafe {{ Ok(Self {{").unwrap();
        for command_group in &command_groups {
            for command in &command_group.commands {
                let name = normalize_command_name(command.alias);
                if command.optional {
                    writeln!(file, "{}: transmute(load(c\"{}\")),", name, command.alias).unwrap();
                } else {
                    writeln!(
                        file,
                        "{}: transmute(load(c\"{}\").ok_or(MissingEntryPointError)?),",
                        name, command.alias
                    )
                    .unwrap();
                }
            }
        }
        writeln!(file, "}}) }} }} }}").unwrap();

        writeln!(file, "impl {} {{", fn_type_name).unwrap();
        for command_group in &command_groups {
            for command in &command_group.commands {
                write_command_wrapper(file, analysis, command);
            }
        }
        writeln!(file, "}}").unwrap();
    };

    generate_commands(CommandType::Entry, "EntryFn");
    generate_commands(CommandType::Instance, "InstanceFn");
    generate_commands(CommandType::Device, "DeviceFn");
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

    let enumeration_len_param =
        len_kinds
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
            });

    let enumeration_info = enumeration_len_param.map(|len_param| {
        let array_params = len_kinds
            .iter()
            .enumerate()
            .filter_map(|(param_index, len_kind)| match len_kind {
                Some(LengthKind::Param { index, .. }) if *index == len_param => Some(param_index),
                _ => None,
            })
            .collect();
        EnumerationCommandInfo {
            len_param,
            array_params,
        }
    });

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

        let optional = (
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

        let is_output_param = {
            let category = ctype_rust::CTypeCategory::from_ctype(&param.c_decl.ty, analysis);
            match category {
                ctype_rust::CTypeCategory::TypedPointer { is_const, pointee } => {
                    if is_const || param.len.is_some() {
                        false
                    } else if let CType::Base(base) = pointee {
                        !ctype_rust::is_opaque_type(base.name)
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
                    !is_const
                        && ctype_rust::is_opaque_type(
                            ctype_to_rust_type(analysis, pointee, None).as_str(),
                        )
                }
                _ => false,
            };
            non_const_ptr && param.len.is_none() && is_pointer_to_pointer(&param.c_decl.ty)
        };

        let is_return_param = (is_output_param || is_output_opaque_param)
            && !is_implicit_length
            && !has_regular_return
            && command.success_codes.len() <= 1;

        let is_enumeration_array = if let Some(enumeration_info) = &enumeration_info {
            enumeration_info.array_params.contains(&param_index)
        } else {
            false
        };

        if !is_implicit_length {
            if (is_output_param || is_output_opaque_param) && !is_return_param {
                println!(
                    "output param can't be turned into a return value: {} -> {}",
                    command.name, param.c_decl.name
                );
            }

            if is_return_param {
                return_params.push(param_index);
            } else {
                let name = normalize_param_name(param.c_decl.name);
                let ty = convert_param_type(
                    analysis,
                    &param.c_decl.ty,
                    len_kinds[param_index].as_ref(),
                    optional,
                    lifetime_param,
                    is_output_param || is_output_opaque_param,
                );

                wrapper_params.push(WrapperParamInfo {
                    name: name.clone(),
                    param_index,
                    param,
                    ty,
                    is_enumeration_array,
                });
            }
        }

        params.push(ParamInfo {
            name,
            param,
            ty: ctype_to_rust_type(analysis, &param.c_decl.ty, lifetime_param),
            len: len_kinds[param_index].clone(),
            optional,
            is_output_param,
            is_return_param,
            is_output_opaque_param,
        });
    }

    let name = normalize_command_name(info.alias);

    let wrapper_return = if !return_params.is_empty() {
        let params = return_params
            .into_iter()
            .map(|param_index| {
                let param = params[param_index].param;
                let CType::Ptr { pointee, .. } = &param.c_decl.ty else {
                    unreachable!()
                };
                WrapperParamInfo {
                    name: normalize_param_name(param.c_decl.name),
                    ty: ctype_to_rust_type(analysis, &pointee, None),
                    param_index,
                    param,
                    is_enumeration_array: false,
                }
            })
            .collect();

        WrapperReturnKind::OutputParams(params)
    } else if has_regular_return {
        WrapperReturnKind::CommandReturnValue {
            ty: ctype_to_rust_type(analysis, command.return_type.as_ref().unwrap(), None),
        }
    } else {
        WrapperReturnKind::None
    };

    WrapperCommandInfo {
        command,
        name,
        enumeration_info,
        wrapper_params,
        wrapper_return,
        params,
        is_fallible,
        has_regular_return,
        lifetime_param,
    }
}

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
                if !is_const && matches!(len.and_then(LengthKind::ty), Some(CType::Ptr { .. })) {
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
                    if matches!(len.and_then(LengthKind::ty), Some(CType::Ptr { .. })) {
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
            let s = if ctype_rust::is_opaque_type(ty.as_str()) {
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
        _ => ctype_to_rust_type(analysis, ty, lifetime_param),
    }
}

pub fn write_command_wrapper(
    file: &mut impl std::io::Write,
    analysis: &Analysis,
    info: &CommandInfo<'_>,
) {
    let wrapper = analyze_command(analysis, info);

    let lifetime_param = wrapper
        .lifetime_param
        .map(|lifetime| format!("<'{}>", lifetime))
        .unwrap_or_default();
    writeln!(
        file,
        "pub unsafe fn {}{}(&self,",
        wrapper.name, lifetime_param
    )
    .unwrap();

    for param in &wrapper.wrapper_params {
        if param.is_enumeration_array {
            write!(file, "mut ").unwrap();
        }
        writeln!(file, "{}: {},", param.name, param.ty).unwrap();
    }

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
        writeln!(
            file,
            ") -> crate::Result<{}> {{",
            return_ty.as_deref().unwrap_or("()")
        )
        .unwrap();
    } else if let Some(return_ty) = return_ty {
        writeln!(file, ") -> {} {{", return_ty).unwrap();
    } else {
        writeln!(file, ") {{").unwrap();
    }

    writeln!(file, "unsafe {{").unwrap();

    if let Some(enumeration_info) = &wrapper.enumeration_info {
        write_enumeration_fn_body(file, analysis, info, &wrapper, enumeration_info);
    } else {
        write_fn_body(file, analysis, &wrapper, info.optional);
    }

    writeln!(file, "}}").unwrap();
    writeln!(file, "}}").unwrap();
}

fn write_enumeration_fn_body(
    file: &mut impl std::io::Write,
    analysis: &Analysis,
    info: &CommandInfo<'_>,
    wrapper: &WrapperCommandInfo<'_>,
    enumeration_info: &EnumerationCommandInfo,
) {
    let len_param = &wrapper.params[enumeration_info.len_param].name;
    writeln!(file, "let call = |{len_param}, ").unwrap();
    for param in &enumeration_info.array_params {
        writeln!(file, "{}, ", wrapper.params[*param].name).unwrap();
    }
    for wrapper_param in &wrapper.wrapper_params {
        let param = &wrapper.params[wrapper_param.param_index];
        if param.is_output_param && wrapper_param.param_index != enumeration_info.len_param && !param.is_return_param {
            writeln!(file, "{}: {}, ", wrapper_param.name, wrapper_param.ty).unwrap();
        }
    }
    writeln!(file, "| {{ ").unwrap();
    write_fn_body(file, analysis, wrapper, info.optional);
    writeln!(file, "}};").unwrap();

    writeln!(file, "let mut len = 0; call(&mut len, ").unwrap();
    for _ in &enumeration_info.array_params {
        writeln!(file, "std::ptr::null_mut(), ").unwrap();
    }
    for wrapper_param in &wrapper.wrapper_params {
        let param = &wrapper.params[wrapper_param.param_index];
        if param.is_output_param && wrapper_param.param_index != enumeration_info.len_param && !param.is_return_param {
            // Skip in the first call if optional 
            if param.optional.0 {
                writeln!(file, "None, ").unwrap();
            } else {
                writeln!(file, "{}, ", param.name).unwrap();
            }
        } 
    }
    if wrapper.is_fallible {
        writeln!(file, ")?;").unwrap();
    } else {
        writeln!(file, ");").unwrap();
    }
    writeln!(
        file,
        "let capacity = len.try_into().expect(\"failed to convert `N` to usize\");"
    )
    .unwrap();

    for (array_param_index, param) in enumeration_info.array_params.iter().enumerate() {
        let param = &wrapper.params[*param];
        let param_name = &param.name;
        writeln!(
            file,
            "let {param_name}_buf = {param_name}.reserve(capacity);"
        )
        .unwrap();
        if array_param_index == 0 {
            writeln!(file, "len = {param_name}_buf.len().try_into().unwrap();").unwrap();
        } else {
            let first_param = &wrapper.params[enumeration_info.array_params[0]].name;
            writeln!(
                file,
                "assert_eq!({param_name}_buf.len(), {first_param}_buf.len());"
            )
            .unwrap();
        }
    }
    if wrapper.is_fallible {
        writeln!(file, "let result = ").unwrap();
    }
    writeln!(file, "call(&mut len, ").unwrap();
    for param in &enumeration_info.array_params {
        let param = &wrapper.params[*param];
        let param_name = &param.name;
        writeln!(file, "{param_name}_buf.as_mut_ptr() as *mut _, ").unwrap();
    }
    for wrapper_param in &wrapper.wrapper_params {
        let param = &wrapper.params[wrapper_param.param_index];
        if param.is_output_param && wrapper_param.param_index != enumeration_info.len_param && !param.is_return_param {
            writeln!(file, "{}, ", param.name).unwrap();
        }
    }
    if wrapper.is_fallible {
        writeln!(file, ")?;").unwrap();
    } else {
        writeln!(file, ");").unwrap();
    }

    for param in &enumeration_info.array_params {
        let param = &wrapper.params[*param];
        let param_name = &param.name;
        writeln!(file, "{param_name}.set_len(len.try_into().unwrap());").unwrap();
    }
    if wrapper.is_fallible {
        writeln!(file, "Ok(result)").unwrap();
    }
}

fn arg_emit_kind(
    param_index: usize,
    param: &ParamInfo,
    wrapper: &WrapperCommandInfo,
    analysis: &Analysis,
) -> ArgEmitKind {
    let ty = &param.param.c_decl.ty;
    let category = ctype_rust::CTypeCategory::from_ctype(ty, analysis);
    let in_enumeration = wrapper.enumeration_info.is_some();

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
        if enumeration_info.array_params.contains(&param_index) {
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
        return ArgEmitKind::OptionalPtrToRaw {
            is_const,
            in_enumeration,
        };
    }

    ArgEmitKind::Direct
}

fn emit_arg(file: &mut impl std::io::Write, param_name: &str, kind: ArgEmitKind) {
    match kind {
        ArgEmitKind::Direct => {
            writeln!(file, "{},", param_name).unwrap();
        }
        ArgEmitKind::LenFromSlice { slice_param_name } => {
            writeln!(file, "{}.len().try_into().unwrap(),", slice_param_name).unwrap();
        }
        ArgEmitKind::SliceAsPtr { is_const, optional } => {
            if optional {
                if is_const {
                    writeln!(file, "{}.to_raw_ptr(),", param_name).unwrap();
                } else {
                    writeln!(file, "{}.to_raw_mut_ptr(),", param_name).unwrap();
                }
            } else {
                if is_const {
                    writeln!(file, "{}.as_ptr() as _,", param_name).unwrap();
                } else {
                    writeln!(file, "{}.as_mut_ptr() as _,", param_name).unwrap();
                }
            }
        }
        ArgEmitKind::ReturnParamAsMutPtr => {
            writeln!(file, "{}.as_mut_ptr(),", param_name).unwrap();
        }
        ArgEmitKind::OptionalPtrToRaw { is_const, .. } => {
            if is_const {
                writeln!(file, "{}.to_raw_ptr(),", param_name).unwrap();
            } else {
                writeln!(file, "{}.to_raw_mut_ptr(),", param_name).unwrap();
            }
        }
        ArgEmitKind::TransmuteForEnumeration => {
            writeln!(file, "{} as _,", param_name).unwrap();
        }
    }
}

fn write_fn_body(
    file: &mut impl std::io::Write,
    analysis: &Analysis,
    wrapper: &WrapperCommandInfo,
    optional: bool,
) {
    if let WrapperReturnKind::OutputParams(params) = &wrapper.wrapper_return {
        for param in params {
            writeln!(
                file,
                "let mut {} = core::mem::MaybeUninit::uninit();",
                param.name
            )
            .unwrap();
        }
    }

    if wrapper.is_fallible {
        writeln!(file, "let result = ").unwrap();
    }

    if optional {
        writeln!(file, "(self.{}.unwrap())(", wrapper.name).unwrap();
    } else {
        writeln!(file, "(self.{})(", wrapper.name).unwrap();
    }

    for (param_index, param) in wrapper.params.iter().enumerate() {
        let kind = arg_emit_kind(param_index, param, wrapper, analysis);
        emit_arg(file, &param.name, kind);
    }
    writeln!(file, ")").unwrap();

    let return_value = match &wrapper.wrapper_return {
        WrapperReturnKind::OutputParams(params) => Some(match params.as_slice() {
            [param] => format!("{}.assume_init()", param.name),
            params => format!(
                "({})",
                params
                    .iter()
                    .map(|param| format!("{}.assume_init()", param.name))
                    .join(", ")
            ),
        }),
        _ => None,
    };

    if wrapper.is_fallible {
        writeln!(file, ";\n").unwrap();
        writeln!(file, "match result {{").unwrap();
        for success_code in &wrapper.command.success_codes {
            writeln!(
                file,
                "VkResult::{} => Ok({}),",
                success_code.strip_prefix("VK_").unwrap_or(success_code),
                return_value.as_deref().unwrap_or("()"),
            )
            .unwrap();
        }
        writeln!(file, "err => Err(err),").unwrap();
        writeln!(file, "}}").unwrap();
    } else if let Some(return_value) = return_value {
        writeln!(file, ";").unwrap();
        writeln!(file, "{}", return_value).unwrap();
    };
}
