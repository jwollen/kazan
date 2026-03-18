use crate::build::type_conv::{self, ArrayParamKind, LengthKind, TypeRole};
use crate::model::command::*;
use crate::model::rust_type::RustType;
use crate::{
    analysis::{Analysis, CommandType},
    cdecl::CType,
    ctype, normalize_command_name, normalize_param_name, normalize_ty_name, overrides, xml,
};

// ── Resolve / classify / collect (mirrors emit::command logic) ───────────────

/// Resolved command info for internal use during building.
struct CommandInfo<'a> {
    command: &'a xml::Command,
    required_name: &'a str,
    conditionally_required: bool,
}

/// Resolve a required command name, following aliases.
fn resolve_command<'a>(
    analysis: &'a Analysis,
    req_cmd: &'a xml::RequireCommand,
    conditionally_required: bool,
) -> CommandInfo<'a> {
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
        conditionally_required,
    }
}

/// Classify which dispatch table a command belongs to.
fn classify_command(analysis: &Analysis, cmd: &CommandInfo) -> Option<CommandType> {
    use overrides::CommandTypeOp;
    match overrides::command_type_override(cmd.command.name) {
        CommandTypeOp::Skip => None,
        CommandTypeOp::Override(ty) => Some(ty),
        CommandTypeOp::Default => {
            let ty = &cmd.command.params.first().unwrap().c_decl.ty;
            let category = ctype::CTypeCategory::from_ctype(ty, analysis);
            match category {
                ctype::CTypeCategory::Base(name) => Some(
                    analysis
                        .handle_command_types()
                        .get(name)
                        .copied()
                        .unwrap_or(CommandType::Entry),
                ),
                _ => Some(CommandType::Entry),
            }
        }
    }
}

/// Collect commands from `requires` blocks that belong to `cmd_type`, grouped by require block.
fn collect_command_groups<'a>(
    analysis: &'a Analysis,
    requires: &[&'a xml::Require],
    cmd_type: CommandType,
) -> Vec<Vec<CommandInfo<'a>>> {
    requires
        .iter()
        .filter_map(|require| {
            let commands: Vec<_> = require
                .commands
                .iter()
                .map(|req_cmd| resolve_command(analysis, req_cmd, !require.depends.is_empty()))
                .filter(|cmd| classify_command(analysis, cmd) == Some(cmd_type))
                .collect();

            if commands.is_empty() {
                None
            } else {
                Some(commands)
            }
        })
        .collect()
}

// ── Analysis types (mirrors emit::command internal types) ────────────────────

/// Info for the two-call enumeration pattern.
struct EnumerationInfo {
    len_param_index: usize,
    array_param_indices: Vec<usize>,
}

struct ParamClassification {
    is_output_param: bool,
    is_output_opaque_param: bool,
    is_return_param: bool,
}

/// A group of array parameters sharing the same length/count parameter.
struct LengthGroupInfo {
    count_param_index: usize,
    count_optional: bool,
    /// (param_index, nullable) for each array in the group.
    arrays: Vec<(usize, bool)>,
}

struct ParamInfo<'a> {
    name: String,
    param: &'a xml::CommandParam,
    len: Option<LengthKind<'a>>,
    optional: (bool, bool),
    nullable: bool,
    array_param_kind: ArrayParamKind,
    is_output_param: bool,
    is_return_param: bool,
    is_output_opaque_param: bool,
}

enum WrapperReturnKind {
    None,
    CommandReturnValue { ty: RustType },
    OutputParams(Vec<ReturnParamInfo>),
}

struct ReturnParamInfo {
    name: String,
    ty: RustType,
}

struct WrapperParamInfo {
    name: String,
    param_index: usize,
    ty: RustType,
    is_enumeration_array: bool,
}

struct AnalyzedCommand<'a> {
    command: &'a xml::Command,
    name: String,
    enumeration_info: Option<EnumerationInfo>,
    lifetime_param: Option<&'a str>,
    wrapper_params: Vec<WrapperParamInfo>,
    wrapper_return: WrapperReturnKind,
    params: Vec<ParamInfo<'a>>,
    is_fallible: bool,
    length_groups: Vec<LengthGroupInfo>,
}

// ── Core analysis functions ──────────────────────────────────────────────────

/// True only for pointer-to-pointer C types.
fn is_pointer_to_pointer(ty: &CType) -> bool {
    matches!(
        ty,
        CType::Ptr {
            pointee,
            ..
        } if matches!(pointee.as_ref(), CType::Ptr { .. })
    )
}

/// Detect enumeration (two-call) pattern.
fn compute_enumeration_info(
    analysis: &Analysis,
    len_kinds: &[Option<LengthKind<'_>>],
) -> Option<EnumerationInfo> {
    let len_param_index = len_kinds.iter().find_map(|len_kind| {
        let LengthKind::Param {
            index: len_param_index,
            c_decl,
        } = len_kind.as_ref()?
        else {
            return None;
        };
        let category = ctype::CTypeCategory::from_ctype(&c_decl.ty, analysis);
        match category {
            ctype::CTypeCategory::TypedPointer { is_const, .. } if !is_const => {
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

    Some(EnumerationInfo {
        len_param_index,
        array_param_indices,
    })
}

/// Classify a single command parameter's output/return characteristics.
fn classify_param(
    analysis: &Analysis,
    param: &xml::CommandParam,
    param_index: usize,
    len_kinds: &[Option<LengthKind<'_>>],
    _enumeration_info: &Option<EnumerationInfo>,
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
        let category = ctype::CTypeCategory::from_ctype(&param.c_decl.ty, analysis);
        match category {
            ctype::CTypeCategory::TypedPointer { is_const, pointee } => {
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

    let is_output_opaque_param = {
        let category = ctype::CTypeCategory::from_ctype(&param.c_decl.ty, analysis);
        let non_const_ptr = match category {
            ctype::CTypeCategory::OpaquePointer { is_const, .. } => !is_const,
            ctype::CTypeCategory::TypedPointer { is_const, pointee } => {
                !is_const && analysis.is_opaque_type(pointee)
            }
            _ => false,
        };
        non_const_ptr && param.len.is_none() && is_pointer_to_pointer(&param.c_decl.ty)
    };

    let is_extensible_output = is_output_param && {
        let category = ctype::CTypeCategory::from_ctype(&param.c_decl.ty, analysis);
        matches!(
            category,
            ctype::CTypeCategory::TypedPointer { pointee: CType::Base(base), .. }
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
                let ty = if ctype::is_bool32(pointee) {
                    RustType::Bool
                } else {
                    type_conv::resolve_ctype(analysis, pointee, None)
                };
                ReturnParamInfo {
                    name: normalize_param_name(param.c_decl.name),
                    ty,
                }
            })
            .collect();

        WrapperReturnKind::OutputParams(return_wrapper_params)
    } else if has_regular_return {
        let ret_ty = command.return_type.as_ref().unwrap();
        WrapperReturnKind::CommandReturnValue {
            ty: if ctype::is_bool32(ret_ty) {
                RustType::Bool
            } else {
                type_conv::resolve_ctype(analysis, ret_ty, None)
            },
        }
    } else {
        WrapperReturnKind::None
    }
}

/// The main analysis function: analyze a command and produce internal analysis data.
fn analyze_command<'a>(analysis: &'a Analysis, info: &CommandInfo<'a>) -> AnalyzedCommand<'a> {
    let command = info.command;
    let len_kinds: Vec<_> = command
        .params
        .iter()
        .map(|param| {
            param
                .len
                .map(|len| type_conv::get_len_kind(analysis, &command.params, len))
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

    // First pass: compute optional, nullable, classification for each param.
    struct FirstPassInfo {
        name: String,
        optional: (bool, bool),
        nullable: bool,
        classification: ParamClassification,
        is_enumeration_array: bool,
    }
    let first_pass: Vec<_> = command
        .params
        .iter()
        .enumerate()
        .map(|(param_index, param)| {
            let name = normalize_param_name(param.c_decl.name);
            let optional: (bool, bool) = (
                param
                    .optional
                    .first()
                    .and_then(|s| s.parse().ok())
                    .unwrap_or_default(),
                param
                    .optional
                    .get(1)
                    .and_then(|s| s.parse().ok())
                    .unwrap_or_default(),
            );

            let nullable = if param.len.is_some() && matches!(&param.c_decl.ty, CType::Ptr { .. }) {
                if optional.0 {
                    true
                } else if param.noautovalidity {
                    overrides::noautovalidity_pointer_nullable(command.name, param.c_decl.name)
                } else {
                    false
                }
            } else {
                optional.0
            };

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

            FirstPassInfo {
                name,
                optional,
                nullable,
                classification,
                is_enumeration_array,
            }
        })
        .collect();

    // Build length groups: group array params by their shared count param.
    let mut length_groups: Vec<LengthGroupInfo> = Vec::new();
    for param_index in 0..command.params.len() {
        if let Some(LengthKind::Param {
            index: count_index, ..
        }) = &len_kinds[param_index]
        {
            // Only consider input array params (not output/enumeration).
            if first_pass[param_index].classification.is_output_param
                || first_pass[param_index].classification.is_return_param
                || first_pass[param_index].is_enumeration_array
            {
                continue;
            }
            if let Some(group) = length_groups
                .iter_mut()
                .find(|g| g.count_param_index == *count_index)
            {
                group
                    .arrays
                    .push((param_index, first_pass[param_index].nullable));
            } else {
                let count_optional: bool = command.params[*count_index]
                    .optional
                    .first()
                    .and_then(|s| s.parse().ok())
                    .unwrap_or_default();
                length_groups.push(LengthGroupInfo {
                    count_param_index: *count_index,
                    count_optional,
                    arrays: vec![(param_index, first_pass[param_index].nullable)],
                });
            }
        }
    }

    // Classify each array param kind based on its length group.
    let mut array_param_kinds: Vec<ArrayParamKind> =
        vec![ArrayParamKind::Standard; command.params.len()];
    for group in &length_groups {
        let has_required = group.arrays.iter().any(|(_, nullable)| !nullable);
        let has_nullable = group.arrays.iter().any(|(_, nullable)| *nullable);
        let all_nullable = !has_required;

        if !has_nullable {
            continue;
        }

        if !all_nullable {
            continue;
        }

        let (primary_index, _) = group.arrays[0];
        if group.count_optional {
            array_param_kinds[primary_index] = ArrayParamKind::OptionSliceOrLen;
        } else {
            array_param_kinds[primary_index] = ArrayParamKind::SliceOrLen;
        }
    }

    // Second pass: build wrapper params and param infos.
    for (param_index, param) in command.params.iter().enumerate() {
        let fp = &first_pass[param_index];

        let is_implicit_length = len_kinds.iter().any(
            |len| matches!(len, Some(LengthKind::Param { index, .. }) if *index == param_index),
        );

        if !is_implicit_length {
            if fp.classification.is_return_param {
                return_params.push(param_index);
            } else {
                let name = normalize_param_name(param.c_decl.name);
                let ty = type_conv::convert_type(
                    analysis,
                    &param.c_decl.ty,
                    &TypeRole::CommandParam {
                        len: len_kinds[param_index].as_ref(),
                        optional: fp.optional,
                        nullable: fp.nullable,
                        lifetime: lifetime_param,
                        is_output: fp.classification.is_output_param
                            || fp.classification.is_output_opaque_param,
                        array_kind: array_param_kinds[param_index],
                    },
                );

                wrapper_params.push(WrapperParamInfo {
                    name: name.clone(),
                    param_index,
                    ty,
                    is_enumeration_array: fp.is_enumeration_array,
                });
            }
        }

        params.push(ParamInfo {
            name: fp.name.clone(),
            param,
            len: len_kinds[param_index].clone(),
            optional: fp.optional,
            nullable: fp.nullable,
            array_param_kind: array_param_kinds[param_index],
            is_output_param: fp.classification.is_output_param,
            is_return_param: fp.classification.is_return_param,
            is_output_opaque_param: fp.classification.is_output_opaque_param,
        });
    }

    let name = normalize_command_name(info.required_name, analysis.registry());
    let wrapper_return = build_wrapper_return(
        analysis,
        command,
        return_params,
        &params,
        has_regular_return,
    );

    AnalyzedCommand {
        command,
        name,
        enumeration_info,
        wrapper_params,
        wrapper_return,
        params,
        is_fallible,
        lifetime_param,
        length_groups,
    }
}

// ── ArgEmitKind determination (mirrors emit::command::arg_emit_kind) ─────────

/// Determine how to emit this parameter as an argument in the generated FFI call site.
fn arg_emit_kind(
    param_index: usize,
    param: &ParamInfo,
    wrapper: &AnalyzedCommand,
    analysis: &Analysis,
) -> FfiArg {
    let ty = &param.param.c_decl.ty;
    let category = ctype::CTypeCategory::from_ctype(ty, analysis);
    let param_name = &param.name;

    // Is this param the length for another param?
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
        if matches!(ty, CType::Ptr { .. }) {
            return FfiArg::Direct {
                param: param_name.clone(),
            };
        }
        if let Some(array_param) = array_param_for_len {
            match array_param.array_param_kind {
                ArrayParamKind::SliceOrLen => {
                    return FfiArg::LenFromSliceOrLen {
                        param: array_param.name.clone(),
                        option_wrapped: false,
                    };
                }
                ArrayParamKind::OptionSliceOrLen => {
                    return FfiArg::LenFromSliceOrLen {
                        param: array_param.name.clone(),
                        option_wrapped: true,
                    };
                }
                ArrayParamKind::Standard => {
                    return FfiArg::LenFromSlice {
                        slice: array_param.name.clone(),
                    };
                }
            }
        }
    }

    if let Some(enumeration_info) = &wrapper.enumeration_info
        && enumeration_info.array_param_indices.contains(&param_index)
    {
        return FfiArg::EnumerationBuf {
            param: param_name.clone(),
        };
    }

    if let Some(len) = &param.len
        && !matches!(len, LengthKind::Literal(1))
    {
        match param.array_param_kind {
            ArrayParamKind::SliceOrLen | ArrayParamKind::OptionSliceOrLen => {
                return FfiArg::SliceOrLenAsPtr {
                    param: param_name.clone(),
                };
            }
            ArrayParamKind::Standard => {}
        }

        let is_const = match category {
            ctype::CTypeCategory::TypedPointer { is_const, .. }
            | ctype::CTypeCategory::CharPointer { is_const }
            | ctype::CTypeCategory::OpaquePointer { is_const, .. } => is_const,
            _ => false,
        };
        return FfiArg::SliceAsPtr {
            param: param_name.clone(),
            is_const,
            optional: param.nullable,
        };
    }

    if param.is_return_param {
        return FfiArg::OutputMutPtr {
            param: param_name.clone(),
        };
    }

    if matches!(
        category,
        ctype::CTypeCategory::TypedPointer { .. }
            | ctype::CTypeCategory::OpaquePointer { .. }
            | ctype::CTypeCategory::CharPointer { .. }
    ) && param.optional.0
        && !param.is_output_opaque_param
    {
        let is_const = match category {
            ctype::CTypeCategory::TypedPointer { is_const, .. }
            | ctype::CTypeCategory::CharPointer { is_const }
            | ctype::CTypeCategory::OpaquePointer { is_const, .. } => is_const,
            _ => true,
        };
        return FfiArg::OptionalPtrToRaw {
            param: param_name.clone(),
            is_const,
        };
    }

    if ctype::is_bool32(ty) {
        return FfiArg::BoolInto {
            param: param_name.clone(),
        };
    }

    FfiArg::Direct {
        param: param_name.clone(),
    }
}

// ── Length assertion building ─────────────────────────────────────────────────

/// Build length assertions for multi-array groups (2+ arrays sharing a count).
fn build_length_assertions(wrapper: &AnalyzedCommand) -> Vec<LengthAssertion> {
    let mut assertions = Vec::new();
    for group in &wrapper.length_groups {
        if group.arrays.len() < 2 {
            continue;
        }

        // Find the "primary" array — first required, or first SliceOrLen, or first in list.
        let primary_index = group
            .arrays
            .iter()
            .find(|(_, nullable)| !nullable)
            .or_else(|| {
                group.arrays.iter().find(|(idx, _)| {
                    matches!(
                        wrapper.params[*idx].array_param_kind,
                        ArrayParamKind::SliceOrLen | ArrayParamKind::OptionSliceOrLen
                    )
                })
            })
            .map(|(idx, _)| *idx)
            .unwrap_or(group.arrays[0].0);

        let primary_name = &wrapper.params[primary_index].name;
        let primary_nullable = wrapper.params[primary_index].nullable;
        let primary_kind = wrapper.params[primary_index].array_param_kind;

        // Build the expression for the primary array's length.
        let primary_len_expr = match primary_kind {
            ArrayParamKind::SliceOrLen => format!("{primary_name}.len()"),
            ArrayParamKind::OptionSliceOrLen => {
                format!("{primary_name}.as_ref().map_or(0, SliceOrLen::len)")
            }
            ArrayParamKind::Standard if primary_nullable => {
                format!("{primary_name}.map_or(0, |s| s.len())")
            }
            ArrayParamKind::Standard => format!("{primary_name}.len()"),
        };

        let array_assertions: Vec<ArrayLenAssertion> = group
            .arrays
            .iter()
            .filter(|(idx, _)| *idx != primary_index)
            .map(|(idx, nullable)| ArrayLenAssertion {
                array_name: wrapper.params[*idx].name.clone(),
                nullable: *nullable,
            })
            .collect();

        assertions.push(LengthAssertion {
            primary_len_expr,
            assertions: array_assertions,
        });
    }
    assertions
}

// ── Model building from analysis ─────────────────────────────────────────────

/// Build a `CommandReturn` model from the analyzed wrapper return info.
fn build_command_return(wrapper: &AnalyzedCommand, info: &CommandInfo) -> CommandReturn {
    let ok_codes_override = overrides::ok_codes(info.required_name);
    let has_multiple_ok_codes = ok_codes_override
        .as_ref()
        .is_some_and(|o| o.codes.len() > 1);

    let inner_return = match &wrapper.wrapper_return {
        WrapperReturnKind::None => CommandReturn::Void,
        WrapperReturnKind::CommandReturnValue { ty } => CommandReturn::Value(ty.clone()),
        WrapperReturnKind::OutputParams(params) => {
            let output_params: Vec<OutputParam> = params
                .iter()
                .map(|p| OutputParam {
                    name: p.name.clone(),
                    bool_convert: matches!(&p.ty, RustType::Bool),
                    ty: p.ty.clone(),
                })
                .collect();
            CommandReturn::OutputParams(output_params)
        }
    };

    if wrapper.is_fallible {
        if has_multiple_ok_codes {
            let ok = ok_codes_override.as_ref().unwrap();
            let status_type = match ok.repr {
                overrides::SuccessCodeRepr::RawResult => RustType::named("VkResult"),
                overrides::SuccessCodeRepr::Bool => RustType::Bool,
            };
            CommandReturn::FallibleMultiSuccess {
                inner: Box::new(inner_return),
                status_type,
            }
        } else {
            CommandReturn::Fallible(Box::new(inner_return))
        }
    } else {
        inner_return
    }
}

/// Build `ResultHandling` for a direct (non-enumeration) call.
fn build_result_handling(
    wrapper: &AnalyzedCommand,
    info: &CommandInfo,
    in_enumeration: bool,
) -> ResultHandling {
    let ok_codes_override = overrides::ok_codes(info.required_name);

    if let WrapperReturnKind::OutputParams(_) = &wrapper.wrapper_return {
        if wrapper.is_fallible {
            let default_codes: &[&str] = &["VK_SUCCESS"];
            let codes = if in_enumeration {
                &wrapper.command.success_codes
            } else {
                ok_codes_override
                    .as_ref()
                    .map(|o| o.codes)
                    .unwrap_or(default_codes)
            };
            let expose_success_code = !in_enumeration
                && ok_codes_override
                    .as_ref()
                    .is_some_and(|o| o.codes.len() > 1);

            // Build output_expr from output params
            let output_expr = match &wrapper.wrapper_return {
                WrapperReturnKind::OutputParams(params) => {
                    let expr = match params.as_slice() {
                        [param] => {
                            let init = format!("{}.assume_init()", param.name);
                            if matches!(&param.ty, RustType::Bool) {
                                format!("{init} != 0")
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
                                    if matches!(&param.ty, RustType::Bool) {
                                        format!("{init} != 0")
                                    } else {
                                        init
                                    }
                                })
                                .collect::<Vec<_>>()
                                .join(", ")
                        ),
                    };
                    Some(expr)
                }
                _ => None,
            };

            return ResultHandling::MatchResult {
                ok_codes: codes.iter().map(|s| s.to_string()).collect(),
                output_expr,
                expose_status: expose_success_code,
            };
        }
        // Not fallible but has output params
        return ResultHandling::OutputParams;
    }

    if matches!(&wrapper.wrapper_return, WrapperReturnKind::CommandReturnValue { ty } if matches!(ty, RustType::Bool))
    {
        if wrapper.is_fallible {
            // Shouldn't happen: bool return + fallible doesn't make sense
            // but handle it as a match result
            let default_codes: &[&str] = &["VK_SUCCESS"];
            let codes = ok_codes_override
                .as_ref()
                .map(|o| o.codes)
                .unwrap_or(default_codes);
            return ResultHandling::MatchResult {
                ok_codes: codes.iter().map(|s| s.to_string()).collect(),
                output_expr: None,
                expose_status: false,
            };
        }
        return ResultHandling::ReturnDirect { bool_convert: true };
    }

    if wrapper.is_fallible {
        let default_codes: &[&str] = &["VK_SUCCESS"];
        let codes = if in_enumeration {
            &wrapper.command.success_codes
        } else {
            ok_codes_override
                .as_ref()
                .map(|o| o.codes)
                .unwrap_or(default_codes)
        };
        let expose_success_code = !in_enumeration
            && ok_codes_override
                .as_ref()
                .is_some_and(|o| o.codes.len() > 1);

        return ResultHandling::MatchResult {
            ok_codes: codes.iter().map(|s| s.to_string()).collect(),
            output_expr: None,
            expose_status: expose_success_code,
        };
    }

    ResultHandling::None
}

/// Build an `FfiCall` for the analyzed command.
fn build_ffi_call(
    wrapper: &AnalyzedCommand,
    info: &CommandInfo,
    analysis: &Analysis,
    in_enumeration: bool,
) -> FfiCall {
    let args: Vec<FfiArg> = wrapper
        .params
        .iter()
        .enumerate()
        .map(|(param_index, param)| arg_emit_kind(param_index, param, wrapper, analysis))
        .collect();

    let pre_assertions = if in_enumeration {
        Vec::new()
    } else {
        build_length_assertions(wrapper)
    };

    FfiCall {
        fn_field: wrapper.name.clone(),
        conditional: info.conditionally_required,
        args,
        pre_assertions,
    }
}

/// Build the `CommandBody` for a single command.
fn build_command_body(
    wrapper: &AnalyzedCommand,
    info: &CommandInfo,
    analysis: &Analysis,
) -> CommandBody {
    if let Some(enumeration_info) = &wrapper.enumeration_info {
        // Two-call enumeration pattern.
        let len_param = wrapper.params[enumeration_info.len_param_index]
            .name
            .clone();

        let array_params: Vec<String> = enumeration_info
            .array_param_indices
            .iter()
            .map(|idx| wrapper.params[*idx].name.clone())
            .collect();

        // Extra output params: output params in the wrapper that are not the length, not array,
        // and not return params.
        let mut extra_output_params: Vec<ExtraEnumParam> = Vec::new();
        let mut extra_optional_flags: Vec<bool> = Vec::new();
        for wp in &wrapper.wrapper_params {
            let param = &wrapper.params[wp.param_index];
            if param.is_output_param
                && wp.param_index != enumeration_info.len_param_index
                && !param.is_return_param
            {
                extra_output_params.push(ExtraEnumParam {
                    name: wp.name.clone(),
                    ty: wp.ty.clone(),
                });
                extra_optional_flags.push(param.optional.0);
            }
        }

        let inner_call = build_ffi_call(wrapper, info, analysis, true);

        let closure_ok_codes: Vec<String> = wrapper
            .command
            .success_codes
            .iter()
            .map(|s| s.to_string())
            .collect();

        CommandBody::Enumeration(EnumerationCall {
            len_param,
            array_params,
            extra_output_params,
            inner_call,
            is_fallible: wrapper.is_fallible,
            closure_ok_codes,
            extra_optional_flags,
        })
    } else {
        let ffi_call = build_ffi_call(wrapper, info, analysis, false);
        let result_handling = build_result_handling(wrapper, info, false);

        CommandBody::Direct(DirectCall {
            ffi_call,
            result_handling,
        })
    }
}

/// Build a `CommandWrapper` from analyzed command data.
fn build_wrapper(
    wrapper: &AnalyzedCommand,
    info: &CommandInfo,
    analysis: &Analysis,
) -> CommandWrapper {
    let params: Vec<WrapperParam> = wrapper
        .wrapper_params
        .iter()
        .map(|wp| WrapperParam {
            name: wp.name.clone(),
            ty: wp.ty.clone(),
            mutable_binding: wp.is_enumeration_array,
        })
        .collect();

    let return_type = build_command_return(wrapper, info);
    let body = build_command_body(wrapper, info, analysis);

    CommandWrapper {
        name: wrapper.name.clone(),
        c_name: info.required_name.to_string(),
        lifetime_param: wrapper.lifetime_param.map(|s| s.to_string()),
        params,
        return_type,
        body,
    }
}

/// Build a `DispatchEntry` for a single command.
fn build_dispatch_entry(analysis: &Analysis, info: &CommandInfo) -> DispatchEntry {
    // Check for override first.
    // write_command_override currently always returns false, but keep the check.
    let mut buf = Vec::new();
    let is_override = overrides::write_command_override(
        &mut buf,
        info.required_name,
        info.conditionally_required,
    )
    .unwrap_or(false);

    if is_override {
        let override_body = String::from_utf8(buf).unwrap_or_default();
        let name = normalize_command_name(info.required_name, analysis.registry());
        let pfn_type = format!("PFN_{}", normalize_ty_name(info.command.name));
        return DispatchEntry {
            field_name: name.clone(),
            pfn_type,
            c_entry_point: info.required_name.to_string(),
            conditional: info.conditionally_required,
            wrapper: CommandWrapper {
                name,
                c_name: info.required_name.to_string(),
                lifetime_param: None,
                params: vec![],
                return_type: CommandReturn::Void,
                body: CommandBody::Override(override_body),
            },
        };
    }

    let wrapper_data = analyze_command(analysis, info);
    let cmd_wrapper = build_wrapper(&wrapper_data, info, analysis);

    let field_name = normalize_command_name(info.required_name, analysis.registry());
    let pfn_type = format!("PFN_{}", normalize_ty_name(info.command.name));

    DispatchEntry {
        field_name,
        pfn_type,
        c_entry_point: info.required_name.to_string(),
        conditional: info.conditionally_required,
        wrapper: cmd_wrapper,
    }
}

// ── Public entry point ───────────────────────────────────────────────────────

/// Build all dispatch structs from the given `<require>` blocks.
///
/// Returns one `DispatchStruct` for each of Entry, Instance, Device (in that order),
/// skipping any that have no commands.
pub fn build_dispatch_structs(
    analysis: &Analysis,
    requires: &[&xml::Require],
) -> Vec<DispatchStruct> {
    let dispatch_types = [
        (CommandType::Entry, "EntryFn", DispatchType::Entry),
        (CommandType::Instance, "InstanceFn", DispatchType::Instance),
        (CommandType::Device, "DeviceFn", DispatchType::Device),
    ];

    let mut result = Vec::new();
    for (cmd_type, fn_type_name, dispatch_type) in dispatch_types {
        let command_groups = collect_command_groups(analysis, requires, cmd_type);
        if command_groups.is_empty() {
            continue;
        }

        let groups: Vec<DispatchGroup> = command_groups
            .into_iter()
            .map(|commands| {
                let entries: Vec<DispatchEntry> = commands
                    .iter()
                    .map(|cmd_info| build_dispatch_entry(analysis, cmd_info))
                    .collect();
                DispatchGroup { entries }
            })
            .collect();

        result.push(DispatchStruct {
            name: fn_type_name.to_string(),
            dispatch_type,
            groups,
        });
    }

    result
}
