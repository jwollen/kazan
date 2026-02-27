use std::collections::BTreeMap;

use crate::{
    LengthKind, analysis::Analysis, cdecl::CType, ctype_to_rust_type, get_len_kind, is_opaque_type,
    normalize_name, normalize_param_name, normalize_setter_param_name, normalize_ty_name, xml,
};
use itertools::Itertools;

#[derive(Debug)]
struct StructInfo<'a> {
    name: String,
    ty: &'a xml::Structure,
    tag: Option<&'static str>,
    has_default: bool,
    members: Vec<MemberInfo<'a>>,
    setters: Vec<MemberSetterInfo>,
}

#[derive(Debug)]
struct MemberSetterInfo {
    name: String,
    kind: SetterKind,
}

#[derive(Debug)]
enum SetterKind {
    Value(SetterParamInfo),
    Array {
        len_member_index: usize,
        params: Vec<SetterParamInfo>,
    },
}

#[derive(Debug)]
struct SetterParamInfo {
    name: String,
    member_index: usize,
    ty: String,
}

#[derive(Debug)]
struct MemberInfo<'a> {
    member: &'a xml::StructureMember,
    name: String,
    ty: String,
    len: Vec<LengthKind<'a>>,
}

fn analyze_struct<'a>(analysis: &'a Analysis, struct_ty: &'a xml::Structure) -> StructInfo<'a> {
    let len_kinds: Vec<Vec<_>> = struct_ty
        .members
        .iter()
        .map(|member| {
            member
                .len
                .iter()
                .map(|len| get_len_kind(analysis, &struct_ty.members, len))
                .collect()
        })
        .collect();

    let mut has_default = true;
    let mut tag = None;
    let lifetime_param = Some("a");

    let mut members = Vec::new();
    let mut setters = Vec::new();

    for (member_index, member) in struct_ty.members.iter().enumerate() {
        let name = normalize_name(member.c_decl.name);
        let len = len_kinds[member_index].clone();

        if member.c_decl.name == "sType" {
            if let Some(value) = member.values {
                tag = Some(value.strip_prefix("VK_STRUCTURE_TYPE_").unwrap());
            } else {
                has_default = false;
            }
        }

        let is_special_member = member.c_decl.name == "sType" || member.c_decl.name == "pNext";

        if len.is_empty() && !is_special_member {
            let param_name = normalize_setter_param_name(member.c_decl.name);

            let mut array_params = Vec::new();
            for (array_member_index, array_member) in struct_ty.members.iter().enumerate() {
                let len_kinds = len_kinds[array_member_index].as_slice();
                let len = len_kinds.iter().next();
                if let Some(LengthKind::Param { index, .. }) = len
                    && *index == member_index
                {
                    let optional: Vec<_> = array_member
                        .optional
                        .iter()
                        .map(|s| s.parse::<bool>().unwrap())
                        .collect();

                    let array_param_ty = convert_setter_param_type(
                        analysis,
                        &array_member.c_decl.ty,
                        len_kinds,
                        &optional,
                        lifetime_param,
                    );

                    let name = normalize_setter_param_name(array_member.c_decl.name);

                    array_params.push(SetterParamInfo {
                        name,
                        member_index: array_member_index,
                        ty: array_param_ty,
                    });
                }
            }

            // TODO: Use one setter for all array params need to have the same length and are not optional?
            let merge_array_setters = false;
            let array_param_groups = if array_params.is_empty() {
                vec![vec![]]
            } else if merge_array_setters {
                vec![array_params]
            } else {
                array_params
                    .into_iter()
                    .map(|param| vec![param])
                    .collect::<Vec<_>>()
            };

            for array_params in array_param_groups {
                let setter_name = if array_params.len() == 1 {
                    array_params[0].name.clone()
                } else if let Some(name) = param_name.strip_suffix("_count")
                    && !array_params.is_empty()
                {
                    name.to_string()
                } else {
                    if !array_params.is_empty() {
                        println!(
                            "length member that doesn't end in 'count': {}.{}",
                            struct_ty.name, member.c_decl.name
                        );
                    }
                    param_name.to_string()
                };

                let kind = if array_params.is_empty() {
                    let optional: Vec<_> = member
                        .optional
                        .iter()
                        .map(|s| s.parse::<bool>().unwrap())
                        .collect();

                    let param_ty = convert_setter_param_type(
                        analysis,
                        &member.c_decl.ty,
                        &[],
                        &optional,
                        lifetime_param,
                    );
                    SetterKind::Value(SetterParamInfo {
                        name: param_name.clone(),
                        member_index,
                        ty: param_ty,
                    })
                } else {
                    SetterKind::Array {
                        len_member_index: member_index,
                        params: array_params,
                    }
                };

                setters.push(MemberSetterInfo {
                    name: setter_name,
                    kind,
                });
            }
        }

        let ty = ctype_to_rust_type(analysis, &member.c_decl.ty, lifetime_param);
        members.push(MemberInfo {
            member,
            name,
            ty,
            len,
        });
    }

    let name = normalize_ty_name(struct_ty.name).to_string();
    StructInfo {
        name,
        ty: struct_ty,
        members,
        tag,
        has_default,
        setters,
    }
}

pub fn write_struct(file: &mut impl std::io::Write, analysis: &Analysis, ty: &xml::Structure) {
    let info = analyze_struct(analysis, ty);

    let type_info = analysis.get_base_type_info(ty.name).unwrap();

    let lifetime_spec = if type_info.lifetime_param { "<'a>" } else { "" };
    let lifetime_spec_anon = if type_info.lifetime_param { "<'_>" } else { "" };

    writeln!(
        file,
        "#[repr(C)]
        #[derive(Copy, Clone{})]
        pub struct {}{} {{",
        if info.has_default && type_info.default {
            ", Default"
        } else {
            ""
        },
        normalize_ty_name(ty.name),
        lifetime_spec
    )
    .unwrap();
    for member in &ty.members {
        let name = normalize_name(member.c_decl.name);

        let field_ty = ctype_to_rust_type(analysis, &member.c_decl.ty, Some("a"));
        let field_ty = if let CType::Base(base) = &member.c_decl.ty
            && base.name.starts_with("PFN_")
        {
            format!("Option<{}>", field_ty)
        } else {
            field_ty
        };

        writeln!(file, "pub {}: {},", name, field_ty).unwrap();
    }
    if type_info.lifetime_param {
        writeln!(file, "pub _marker: PhantomData<&'a ()>,",).unwrap();
    }
    writeln!(file, "}}").unwrap();

    if info.has_default && !type_info.default {
        writeln!(
            file,
            "impl Default for {}{} {{
            fn default() -> Self {{
            Self {{",
            info.name, lifetime_spec_anon
        )
        .unwrap();
        for member in &info.members {
            write!(file, "{}: ", member.name).unwrap();
            if member.member.c_decl.name == "sType" {
                writeln!(file, "StructureType::{}", info.tag.unwrap()).unwrap()
            } else {
                write!(file, "{}", default_value(&member.member.c_decl.ty)).unwrap();
            }
            writeln!(file, ",").unwrap();
        }
        if type_info.lifetime_param {
            writeln!(file, "_marker: PhantomData",).unwrap();
        }
        writeln!(file, "}} }} }}").unwrap();
    }

    writeln!(
        file,
        "impl{} {}{}{{",
        lifetime_spec, info.name, lifetime_spec
    )
    .unwrap();
    for setter in &info.setters {
        writeln!(file, "pub fn {}(mut self,", setter.name).unwrap();

        match &setter.kind {
            SetterKind::Value(param) => {
                writeln!(file, "{}: {},", param.name, param.ty).unwrap();
            }
            SetterKind::Array { params, .. } => {
                for param in params {
                    writeln!(file, "{}: {},", param.name, param.ty).unwrap();
                }
            }
        }

        writeln!(file, ") -> Self {{").unwrap();

        match &setter.kind {
            SetterKind::Value(param) => {
                let member = &info.members[param.member_index];
                if member.ty.starts_with("PFN_") {
                    writeln!(file, "self.{} = Some({});", member.name, param.name).unwrap();
                } else {
                    writeln!(file, "self.{} = {};", member.name, param.name).unwrap();
                }
            }
            SetterKind::Array {
                len_member_index,
                params,
            } => {
                // Write length checks
                if params.len() > 1 {
                    for param in params.iter().skip(1) {
                        writeln!(
                            file,
                            "assert_eq!({}.len(), {}.len());",
                            param.name, params[0].name
                        )
                        .unwrap();
                    }
                }

                let len_member = &info.members[*len_member_index];
                writeln!(
                    file,
                    "self.{} = {}.len().try_into().unwrap();",
                    len_member.name, params[0].name
                )
                .unwrap();
                for param in params {
                    let member = &info.members[param.member_index];
                    if let CType::Array { element, .. } = &member.member.c_decl.ty {
                        writeln!(
                            file,
                            "self.{}[..{}.len()].copy_from_slice({});",
                            member.name, param.name, param.name
                        )
                        .unwrap();
                    } else if let CType::Ptr {
                        is_const, pointee, ..
                    } = &member.member.c_decl.ty
                    {
                        if let CType::Base(base) = &pointee.as_ref()
                            && (base.name == "void" || base.name == "char")
                        {
                            if *is_const {
                                writeln!(
                                    file,
                                    "self.{} = {}.as_ptr() as _;",
                                    member.name, param.name
                                )
                                .unwrap();
                            } else {
                                writeln!(
                                    file,
                                    "self.{} = {}.as_mut_ptr() as _;",
                                    member.name, param.name
                                )
                                .unwrap();
                            }
                        } else {
                            if let CType::Ptr {
                                pointee, is_const, ..
                            } = &pointee.as_ref()
                            {
                                if *is_const {
                                    writeln!(
                                        file,
                                        "self.{} = {}.as_ptr() as _;",
                                        member.name, param.name
                                    )
                                    .unwrap();
                                } else {
                                    writeln!(
                                        file,
                                        "self.{} = {}.as_mut_ptr() as _;",
                                        member.name, param.name
                                    )
                                    .unwrap();
                                }
                            } else {
                                if *is_const {
                                    writeln!(
                                        file,
                                        "self.{} = {}.as_ptr();",
                                        member.name, param.name
                                    )
                                    .unwrap();
                                } else {
                                    writeln!(
                                        file,
                                        "self.{} = {}.as_mut_ptr();",
                                        member.name, param.name
                                    )
                                    .unwrap();
                                }
                            }
                        }
                    }
                }
            }
        }

        writeln!(file, "self").unwrap();
        writeln!(file, "}}").unwrap();
    }
    writeln!(file, "}}").unwrap();
}

fn default_value(ty: &CType) -> std::borrow::Cow<'static, str> {
    if let CType::Array { element, .. } = ty {
        format!("[{}; _]", default_value(element)).into()
    } else if let CType::Ptr { is_const, .. } = ty {
        if *is_const {
            "core::ptr::null()".into()
        } else {
            "core::ptr::null_mut()".into()
        }
    // } else if let CType::Base(base) = &member.member.c_decl.ty
    //     && base.name.starts_with("PFN_")
    // {
    //     write!(file, "core::ptr::null()").unwrap();
    } else {
        "Default::default()".into()
    }
}

pub fn convert_setter_param_type(
    analysis: &Analysis,
    ty: &CType,
    lengths: &[LengthKind<'_>],
    optional: &[bool],
    lifetime_param: Option<&str>,
) -> String {
    if let Some(len) = lengths.iter().next()
        && !matches!(len, LengthKind::Literal(1))
    {
        let ty = match ty {
            CType::Ptr {
                pointee, is_const, ..
            } => {
                if matches!(pointee.as_ref(), CType::Base(base) if base.name == "char") {
                    let pointee = "CStr";
                    if *is_const {
                        format!("&'a {}", pointee)
                    } else {
                        format!("&'amut {}", pointee)
                    }
                } else {
                    let lengths = if lengths.len() > 1 {
                        &lengths[1..]
                    } else {
                        &[]
                    };
                    let optional = if optional.len() > 1 {
                        &optional[1..]
                    } else {
                        &[]
                    };
                    let element_ty = convert_setter_param_type(
                        analysis,
                        pointee.as_ref(),
                        lengths,
                        optional,
                        lifetime_param,
                    );
                    let element_ty = if element_ty == "c_void" {
                        "u8"
                    } else {
                        &element_ty
                    };

                    if *is_const {
                        format!("&'a [{}]", element_ty)
                    } else {
                        format!("&'a mut [{}]", element_ty)
                    }
                }
            }
            CType::Array { element, .. } => {
                let element_ty = ctype_to_rust_type(analysis, element.as_ref(), lifetime_param);
                format!("&[{}]", element_ty)
            }

            _ => panic!(
                "expected pointer or arraytype because of length {:?}, got {:?}",
                len, ty
            ),
        };

        ty
        // if matches!(optional, [true, ..]) {
        //     format!("Option<{}>", ty)
        // } else {
        //     ty
        // }
    } else if let CType::Ptr {
        pointee, is_const, ..
    } = ty
    {
        let ty = ctype_to_rust_type(analysis, &pointee, lifetime_param);

        if is_opaque_type(ty.as_str()) {
            if *is_const {
                format!("*const {}", ty)
            } else {
                format!("*mut {}", ty)
            }
        } else {
            let ty = if *is_const {
                format!("&'a {}", ty)
            } else {
                format!("&'a mut {}", ty)
            };

            ty
            // if matches!(optional, [true, ..]) {
            //     format!("Option<{}>", ty)
            // } else {
            //     ty
            // }
        }
    } else {
        ctype_to_rust_type(analysis, ty, lifetime_param)
    }
}
