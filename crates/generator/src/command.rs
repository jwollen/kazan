use crate::{cdecl::CType, ctype_to_rust_type, normalize_name, xml};
use heck::ToSnakeCase;

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

    wrapper_params: Vec<WrapperParamInfo<'a>>,
    params: Vec<ParamInfo<'a>>,
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

struct ParamInfo<'a> {
    name: String,
    param: &'a xml::CommandParam,
    ty: String,
    len: Option<LengthKind<'a>>,
    optional: (bool, bool),
}

#[derive(Clone)]
enum LengthKind<'a> {
    NullTerminated,
    Literal(u32),
    Param {
        index: usize,
        param: &'a xml::CommandParam,
    },
    ParamField {
        index: usize,
        param: &'a xml::CommandParam,
        field: &'a xml::StructureMember,
    },
    Unknown(&'static str),
}

impl<'a> LengthKind<'a> {
    fn ty(&self) -> Option<&CType> {
        match self {
            LengthKind::Param { param, .. } => Some(&param.c_decl.ty),
            LengthKind::ParamField { field, .. } => Some(&field.c_decl.ty),
            _ => None,
        }
    }
}

fn get_param_index(command: &xml::Command, param_name: &str) -> Option<usize> {
    command
        .params
        .iter()
        .enumerate()
        .find_map(|(index, other_param)| {
            if other_param.c_decl.name == param_name {
                Some(index)
            } else {
                None
            }
        })
}

fn get_len_kind<'a>(
    command: &'a xml::Command,
    structs: &'a [&xml::Structure],
    len: &'static str,
) -> LengthKind<'a> {
    if len == "null-terminated" {
        LengthKind::NullTerminated
    } else if let Ok(len) = len.parse() {
        LengthKind::Literal(len)
    } else if let Some((param_name, field_name)) = len.split_once("->")
        && let Some(index) = get_param_index(command, param_name)
    {
        let param = &command.params[index];
        let param_ty = &param.c_decl.ty;
        let CType::Ptr { pointee, .. } = param_ty else {
            panic!("expected pointer type, got {:?}", param_ty);
        };
        let CType::Base(base) = pointee.as_ref() else {
            panic!("expected base type, got {:?}", pointee);
        };

        let struct_ty = structs
            .iter()
            .find(|ty| ty.name == base.name)
            .unwrap_or_else(|| panic!("failed to find struct {}", base.name));

        let field = struct_ty
            .members
            .iter()
            .find(|field| field.c_decl.name == field_name)
            .unwrap_or_else(|| panic!("failed to find field {}", field_name));

        LengthKind::ParamField {
            index,
            param,
            field,
        }
    } else if let Some(index) = get_param_index(command, len) {
        let param = &command.params[index];
        LengthKind::Param { index, param }
    } else {
        LengthKind::Unknown(len)
    }
}

fn analyze_command<'a>(
    info: &CommandInfo<'a>,
    structs: &'a [&xml::Structure],
) -> WrapperCommandInfo<'a> {
    let command = info.command;
    let len_kinds: Vec<_> = command
        .params
        .iter()
        .map(|param| param.len.map(|len| get_len_kind(command, structs, len)))
        .collect();

    let enumeration_len_param = len_kinds.iter().find_map(|len_kind| match len_kind {
        Some(LengthKind::Param { index, param }) => {
            let param_ty = &param.c_decl.ty;
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

    let mut wrapper_params = Vec::new();
    let mut params = Vec::new();

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

        if !is_implicit_length {
            let name = normalize_param_name(param.c_decl.name);
            let wrapper_ty =
                convert_param_type(&param.c_decl.ty, len_kinds[param_index].as_ref(), optional);

            wrapper_params.push(WrapperParamInfo {
                name: name.clone(),
                param,
                ty: wrapper_ty,
            });
        }

        params.push(ParamInfo {
            name,
            param,
            ty: ctype_to_rust_type(&param.c_decl.ty),
            len: len_kinds[param_index].clone(),
            optional,
        });
    }

    let name = normalize_command_name(info.alias);

    WrapperCommandInfo {
        command,
        name,
        enumeration_info,
        wrapper_params,
        params,
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
        let pointee = ctype_to_rust_type(&pointee);

        let ty = if *is_const {
            format!("&{}", pointee)
        } else {
            format!("&mut {}", pointee)
        };

        if (optional).0 {
            format!("Option<{}>", ty)
        } else {
            ty
        }
    } else {
        ctype_to_rust_type(ty)
    }
}

pub fn write_command_wrapper(
    file: &mut impl std::io::Write,
    info: &CommandInfo<'_>,
    structs: &[&xml::Structure],
) {
    let command = info.command;
    let wrapper = analyze_command(info, structs);

    writeln!(file, "pub unsafe fn {}(&self,", wrapper.name).unwrap();

    for param in &wrapper.wrapper_params {
        writeln!(file, "{}: {},", param.name, param.ty).unwrap();
    }

    if let Some(ref return_type) = command.return_type {
        writeln!(file, ") -> {} {{", ctype_to_rust_type(&return_type)).unwrap();
    } else {
        writeln!(file, ") {{").unwrap();
    }

    writeln!(file, "unsafe {{").unwrap();

    if let Some(enumeration_info) = &wrapper.enumeration_info {
        let has_result = if let Some(ret_type) = &command.return_type {
            if let CType::Base(base) = ret_type {
                base.name == "VkResult"
            } else {
                false
            }
        } else {
            false
        };

        let extend_fn_name = if has_result {
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
            } else {
                if let CType::Ptr { is_const, .. } = ty
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
    }
    writeln!(file, ")").unwrap();
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
