use crate::{
    LengthKind,
    cdecl::{CDecl, CType},
    ctype_to_rust_type, get_len_kind, normalize_name, xml,
};
use heck::ToSnakeCase;
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
    param: &'a xml::CommandParam,
    ty: String,
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
}

impl<'a> LengthKind<'a> {
    fn ty(&self) -> Option<&CType> {
        match self {
            LengthKind::Param { c_decl, .. } => Some(&c_decl.ty),
            LengthKind::ParamField { field, .. } => Some(&field.c_decl.ty),
            _ => None,
        }
    }
}

fn analyze_command<'a>(
    info: &CommandInfo<'a>,
    structs: &'a [xml::Structure],
) -> WrapperCommandInfo<'a> {
    let command = info.command;
    let len_kinds: Vec<_> = command
        .params
        .iter()
        .map(|param| {
            param
                .len
                .map(|len| get_len_kind(&command.params, structs, len))
        })
        .collect();

    let enumeration_len_param = len_kinds.iter().find_map(|len_kind| match len_kind {
        Some(LengthKind::Param { index, c_decl }) => {
            let param_ty = &c_decl.ty;
            match param_ty {
                CType::Ptr { is_const, .. } if !*is_const => Some(*index),
                _ => None,
            }
        }
        _ => None,
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

        let is_output_param = match &param.c_decl.ty {
            CType::Ptr {
                is_const, pointee, ..
            } => match pointee.as_ref() {
                CType::Base(base) => {
                    !*is_const && !is_opaque_type(base.name) && param.len.is_none()
                }
                _ => false,
            },
            _ => false,
        };

        let is_return_param = is_output_param
            && !is_implicit_length
            && !has_regular_return
            && command.success_codes.len() <= 1;

        if !is_implicit_length {
            if is_output_param && !is_return_param {
                println!(
                    "output param can't be turned into a return value: {} -> {}",
                    command.name, param.c_decl.name
                );
            }

            if is_return_param {
                return_params.push(param);
            } else {
                let name = normalize_param_name(param.c_decl.name);
                let ty =
                    convert_param_type(&param.c_decl.ty, len_kinds[param_index].as_ref(), optional);

                wrapper_params.push(WrapperParamInfo {
                    name: name.clone(),
                    param,
                    ty,
                });
            }
        }

        params.push(ParamInfo {
            name,
            param,
            ty: ctype_to_rust_type(&param.c_decl.ty),
            len: len_kinds[param_index].clone(),
            optional,
            is_output_param,
            is_return_param,
        });
    }

    let name = normalize_command_name(info.alias);

    let wrapper_return = if !return_params.is_empty() {
        let params = return_params
            .into_iter()
            .map(|param| {
                let CType::Ptr { pointee, .. } = &param.c_decl.ty else {
                    unreachable!()
                };
                WrapperParamInfo {
                    name: normalize_param_name(param.c_decl.name),
                    ty: ctype_to_rust_type(&pointee),
                    param,
                }
            })
            .collect();

        WrapperReturnKind::OutputParams(params)
    } else if has_regular_return {
        WrapperReturnKind::CommandReturnValue {
            ty: ctype_to_rust_type(command.return_type.as_ref().unwrap()),
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
    }
}

fn convert_param_type(ty: &CType, len: Option<&LengthKind<'_>>, optional: (bool, bool)) -> String {
    if let Some(len) = len
        && !matches!(len, LengthKind::Literal(1))
    {
        let CType::Ptr {
            pointee, is_const, ..
        } = ty
        else {
            panic!();
        };

        let ty = if matches!(pointee.as_ref(), CType::Base(base) if base.name == "char") {
            let pointee = "CStr";
            if *is_const {
                format!("&{}", pointee)
            } else {
                format!("&mut {}", pointee)
            }
        } else {
            let element_ty = ctype_to_rust_type(pointee.as_ref());
            let element_ty = if element_ty == "c_void" {
                "u8"
            } else {
                &element_ty
            };

            if *is_const {
                format!("&[{}]", element_ty)
            } else {
                if matches!(len.ty(), Some(CType::Base { .. })) {
                    format!("&mut [{}]", element_ty)
                } else {
                    assert!(matches!(len.ty(), Some(CType::Ptr { .. })));
                    return format!("impl ExtendUninit<{}>", element_ty);
                }
            }
        };

        if (optional).0 {
            format!("Option<{}>", ty)
        } else {
            ty
        }
    } else if let CType::Ptr {
        pointee, is_const, ..
    } = ty
    {
        let ty = ctype_to_rust_type(&pointee);

        if is_opaque_type(ty.as_str()) {
            if *is_const {
                format!("*const {}", ty)
            } else {
                format!("*mut {}", ty)
            }
        } else {
            let ty = if *is_const {
                format!("&{}", ty)
            } else {
                format!("&mut {}", ty)
            };

            if (optional).0 {
                format!("Option<{}>", ty)
            } else {
                ty
            }
        }
    } else {
        ctype_to_rust_type(ty)
    }
}

pub fn write_command_wrapper(
    file: &mut impl std::io::Write,
    info: &CommandInfo<'_>,
    structs: &[xml::Structure],
) {
    let wrapper = analyze_command(info, structs);

    writeln!(file, "pub unsafe fn {}(&self,", wrapper.name).unwrap();

    for param in &wrapper.wrapper_params {
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
        let extend_fn_name = if wrapper.is_fallible {
            match enumeration_info.array_params.len() {
                1 => "try_extend_uninit",
                2 => "try_extend_uninit2",
                _ => todo!(),
            }
        } else {
            match enumeration_info.array_params.len() {
                1 => "extend_uninit",
                _ => todo!(),
            }
        };

        let len_param = &wrapper.params[enumeration_info.len_param];
        let array_params = enumeration_info
            .array_params
            .iter()
            .map(|i| wrapper.params[*i].name.as_str())
            .collect::<Vec<_>>()
            .join(", ");

        writeln!(
            file,
            "{}({}, |{}, {}| {{",
            extend_fn_name, array_params, len_param.name, array_params
        )
        .unwrap();
        write_fn_call(file, &wrapper, info.optional);
        writeln!(file, "}})").unwrap();
    } else {
        write_fn_call(file, &wrapper, info.optional);
    }

    writeln!(file, "}}").unwrap();
    writeln!(file, "}}").unwrap();
}

fn write_fn_call(file: &mut impl std::io::Write, wrapper: &WrapperCommandInfo, optional: bool) {
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
        let array_param = wrapper.params.iter().find(
            |p| matches!(p.len, Some(LengthKind::Param { index, .. }) if index == param_index),
        );

        let ty = &param.param.c_decl.ty;

        if let Some(array_param) = array_param {
            if matches!(ty, CType::Ptr { .. }) {
                writeln!(file, "{},", param.name).unwrap();
            } else {
                writeln!(file, "{}.len().try_into().unwrap(),", array_param.name).unwrap();
            }
        } else {
            if let Some(enumeration_info) = &wrapper.enumeration_info
                && enumeration_info.array_params.contains(&param_index)
            {
                writeln!(file, "{} as _,", param.name).unwrap();
            } else if let Some(len) = &param.len
                && !matches!(len, LengthKind::Literal(1))
            {
                let CType::Ptr { is_const, .. } = ty else {
                    panic!();
                };

                if param.optional.0 {
                    if *is_const {
                        writeln!(file, "{}.to_raw_ptr(),", param.name).unwrap();
                    } else {
                        writeln!(file, "{}.to_raw_mut_ptr(),", param.name).unwrap();
                    }
                } else {
                    if *is_const {
                        writeln!(file, "{}.as_ptr() as _,", param.name).unwrap();
                    } else {
                        writeln!(file, "{}.as_mut_ptr() as _,", param.name).unwrap();
                    }
                }
            } else if param.is_return_param {
                writeln!(file, "{}.as_mut_ptr(),", param.name).unwrap();
            } else if let CType::Ptr { is_const, .. } = ty
                && param.optional.0
            {
                if *is_const {
                    writeln!(file, "{}.to_raw_ptr(),", param.name).unwrap();
                } else {
                    if wrapper.enumeration_info.is_some() {
                        writeln!(
                            file,
                            "todo!(\"output parameters in enumeration commands\"),"
                        )
                        .unwrap();
                    } else {
                        writeln!(file, "{}.to_raw_mut_ptr(),", param.name).unwrap();
                    }
                }
            } else {
                writeln!(file, "{},", param.name).unwrap();
            }
        }
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

fn normalize_param_name(name: &str) -> String {
    let name = normalize_name(name);

    name.strip_prefix("pp_")
        .or_else(|| name.strip_prefix("p_"))
        .unwrap_or(name.as_str())
        .to_string()
}

pub fn normalize_command_name(name: &str) -> String {
    name.strip_prefix("vk").unwrap().to_snake_case()
}

fn is_opaque_type(ty: &str) -> bool {
    matches!(
        ty,
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
