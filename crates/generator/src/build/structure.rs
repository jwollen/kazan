//! Build layer: xml::Structure → StructDef model.

use crate::{
    analysis::Analysis,
    cdecl::CType,
    ctype::{self, CTypeCategory, ctype_to_rust_type, is_bool32},
    normalize_name, normalize_setter_param_name, normalize_ty_name, overrides, xml,
};

use crate::build::type_conv::{self, LengthKind, TypeRole};
use crate::model::rust_type::RustType;
use crate::model::structure::*;

/// Build a `StructDef` model from a raw XML structure definition.
pub fn build_struct(analysis: &Analysis, struct_ty: &xml::Structure) -> StructDef {
    let type_info = analysis.get_base_type_info(struct_ty.name).unwrap();
    let lifetime_param = Some("a");

    let len_kinds: Vec<Vec<_>> = struct_ty
        .members
        .iter()
        .map(|member| {
            member
                .len
                .iter()
                .map(|len| type_conv::get_len_kind(analysis, &struct_ty.members, len))
                .collect()
        })
        .collect();

    let mut has_stype_default = true;
    let mut stype_suffix: Option<&str> = None;

    // Pre-compute bitfield groups.
    let bitfield_groups = collect_bitfield_groups(analysis, &struct_ty.members);
    let mut bitfield_membership = std::collections::HashMap::new();
    for (group_idx, group) in bitfield_groups.iter().enumerate() {
        for (field_idx, _) in group.fields.iter().enumerate() {
            let xml_idx = group.first_xml_index + field_idx;
            bitfield_membership.insert(xml_idx, (group_idx, field_idx == 0));
        }
    }

    // Build fields, setters, and debug fields.
    let mut fields: Vec<StructField> = Vec::new();
    let mut setters: Vec<Setter> = Vec::new();
    let mut debug_fields: Vec<DebugField> = Vec::new();
    let mut xml_to_analyzed: Vec<Option<usize>> = vec![None; struct_ty.members.len()];

    // Track member xml refs for setter body generation.
    let mut field_member_refs: Vec<&xml::StructureMember> = Vec::new();
    // Track original type strings for value assignment detection.
    let mut field_type_strings: Vec<String> = Vec::new();

    for (member_index, member) in struct_ty.members.iter().enumerate() {
        // Handle bitfield groups.
        if let Some(&(group_idx, is_first)) = bitfield_membership.get(&member_index) {
            if is_first {
                let group = &bitfield_groups[group_idx];
                let backing_member_index = fields.len();
                for field_idx in 0..group.fields.len() {
                    xml_to_analyzed[group.first_xml_index + field_idx] = Some(backing_member_index);
                }
                fields.push(StructField {
                    name: group.backing_name.clone(),
                    ty: RustType::named(group.backing_ty.clone()),
                });
                field_member_refs.push(member);
                field_type_strings.push(group.backing_ty.clone());

                // Bitfield debug entries.
                for field in &group.fields {
                    debug_fields.push(DebugField::Bitfield {
                        name: field.name.clone(),
                        backing_field: group.backing_name.clone(),
                        offset: field.offset,
                        width: field.width,
                    });
                }

                // Bitfield setters.
                for field in &group.fields {
                    let extract = if field.width == 1 {
                        BitfieldExtract::Bool
                    } else if field.is_flags_type {
                        BitfieldExtract::AsRaw
                    } else {
                        BitfieldExtract::Direct
                    };
                    setters.push(Setter {
                        name: field.name.clone(),
                        kind: SetterKind::Bitfield {
                            backing_field: backing_member_index,
                            offset: field.offset,
                            width: field.width,
                            param_ty: RustType::named(field.setter_ty.clone()),
                            extract,
                        },
                    });
                }
            }
            continue;
        }

        let analyzed_index = fields.len();
        xml_to_analyzed[member_index] = Some(analyzed_index);

        let name = normalize_name(member.c_decl.name);
        let len = len_kinds[member_index].clone();

        if member.c_decl.name == "sType" {
            if let Some(value) = member.values {
                stype_suffix = Some(value.strip_prefix("VK_STRUCTURE_TYPE_").unwrap());
            } else {
                has_stype_default = false;
            }
        }

        let is_special_member = member.c_decl.name == "sType" || member.c_decl.name == "pNext";

        // Null-terminated string fields → CStr setter.
        if len.len() == 1
            && matches!(len.first(), Some(LengthKind::NullTerminated))
            && !is_special_member
        {
            let category = CTypeCategory::from_ctype(&member.c_decl.ty, analysis);
            let is_char_ptr = matches!(
                category,
                CTypeCategory::CharPointer { .. }
                    | CTypeCategory::OpaquePointer {
                        pointee_name: "char",
                        ..
                    }
            );
            let is_char_array = matches!(
                &category,
                CTypeCategory::Array { element, .. }
                    if matches!(element, CType::Base(b) if b.name == "char")
            );
            if is_char_ptr {
                let param_name = normalize_setter_param_name(member.c_decl.name);
                setters.push(Setter {
                    name: param_name.clone(),
                    kind: SetterKind::CStrToPtr {
                        param: SetterParam {
                            name: param_name,
                            field_index: analyzed_index,
                            ty: RustType::CSTR.into_ref(Some("a".into()), false),
                            optional: false,
                            array_assign: None,
                        },
                        field: analyzed_index,
                    },
                });
            } else if is_char_array {
                let param_name = normalize_setter_param_name(member.c_decl.name);
                setters.push(Setter {
                    name: param_name.clone(),
                    kind: SetterKind::CStrToArray {
                        param: SetterParam {
                            name: param_name,
                            field_index: analyzed_index,
                            ty: RustType::CSTR.into_ref(None, false),
                            optional: false,
                            array_assign: None,
                        },
                        field: analyzed_index,
                    },
                });

                let ty_str = ctype_to_rust_type(analysis, &member.c_decl.ty, lifetime_param);
                fields.push(StructField {
                    name: name.clone(),
                    ty: RustType::named(ty_str.clone()),
                });
                field_member_refs.push(member);
                field_type_strings.push(ty_str);

                // Debug field.
                debug_fields.push(debug_field_for_member(analysis, member));
                continue; // skip general setter logic
            }
        }

        // Members without length: either plain values or length-count fields.
        if len.is_empty() && !is_special_member {
            let param_name = normalize_setter_param_name(member.c_decl.name);
            let array_params = collect_array_params(
                analysis,
                struct_ty,
                &len_kinds,
                member_index,
                lifetime_param,
            );
            let array_param_groups = group_array_params(array_params);
            let new_setters = build_setters_for_length_member(
                analysis,
                struct_ty,
                member,
                member_index,
                &param_name,
                array_param_groups,
                lifetime_param,
            );
            setters.extend(new_setters);
        }

        let ty_str = ctype_to_rust_type(analysis, &member.c_decl.ty, lifetime_param);

        // Compute the struct field type (may differ from ty_str for overrides/funcptrs).
        let field_ty_str =
            if let Some(override_ty) = overrides::member_type_override(member.c_decl.name) {
                override_ty.to_string()
            } else {
                let category = CTypeCategory::from_ctype(&member.c_decl.ty, analysis);
                match category {
                    CTypeCategory::FuncPointer => format!("Option<{ty_str}>"),
                    _ => ty_str.clone(),
                }
            };

        fields.push(StructField {
            name: name.clone(),
            ty: RustType::named(field_ty_str.clone()),
        });
        field_member_refs.push(member);
        field_type_strings.push(ty_str);

        // Debug field.
        debug_fields.push(debug_field_for_member(analysis, member));
    }

    // Remap XML member indices in setters to analyzed member indices.
    for setter in &mut setters {
        match &mut setter.kind {
            SetterKind::Value { param, .. } => {
                if let Some(analyzed) = xml_to_analyzed.get(param.field_index).copied().flatten() {
                    param.field_index = analyzed;
                }
            }
            SetterKind::Array {
                len_field, params, ..
            } => {
                if let Some(analyzed) = xml_to_analyzed.get(*len_field).copied().flatten() {
                    *len_field = analyzed;
                }
                for param in params {
                    if let Some(analyzed) =
                        xml_to_analyzed.get(param.field_index).copied().flatten()
                    {
                        param.field_index = analyzed;
                    }
                }
            }
            SetterKind::Bitfield { .. }
            | SetterKind::CStrToPtr { .. }
            | SetterKind::CStrToArray { .. } => {}
        }
    }

    // Remove setters that overrides want suppressed.
    setters.retain(|s| !overrides::skip_setter(struct_ty.name, &s.name));

    let name = normalize_ty_name(struct_ty.name).to_string();

    // Compute derives.
    let has_derived_default = has_stype_default && type_info.default;
    let mut derives = vec!["Copy".to_string(), "Clone".to_string()];
    if has_derived_default {
        derives.push("Default".to_string());
    }

    // Struct kind.
    let kind = match stype_suffix {
        Some(suffix) => StructKind::Extensible {
            stype_suffix: suffix,
        },
        None => StructKind::Plain,
    };

    // Trait impls.
    let mut trait_impls = Vec::new();
    if let Some(suffix) = stype_suffix {
        trait_impls.push(TraitImpl::TaggedStructure {
            stype_suffix: suffix,
        });
    }
    for extends in &struct_ty.structextends {
        let provisional = analysis.is_provisional_type(extends);
        trait_impls.push(TraitImpl::Extends {
            target: ctype::base_ctype_to_rust_str(extends),
            provisional,
        });
    }

    // Default impl (manual, when sType default exists but zeroed memory isn't valid).
    let default_impl = if has_stype_default && !type_info.default {
        let mut field_defaults = Vec::new();
        for (i, _field) in fields.iter().enumerate() {
            let member = field_member_refs[i];
            if member.c_decl.name == "sType" {
                field_defaults.push(FieldDefault::StructureType);
            } else {
                field_defaults.push(default_value_for_type(analysis, &member.c_decl.ty));
            }
        }
        Some(DefaultImpl {
            field_defaults,
            has_phantom: type_info.lifetime_param,
        })
    } else {
        None
    };

    // Debug impl (manual, when trivial_debug is false).
    let debug_impl = if !type_info.trivial_debug {
        Some(DebugImpl {
            fields: debug_fields,
        })
    } else {
        None
    };

    StructDef {
        name,
        c_name: struct_ty.name,
        kind,
        fields,
        has_lifetime: type_info.lifetime_param,
        derives,
        setters,
        trait_impls,
        default_impl,
        debug_impl,
    }
}

// ---- Helper types and functions (mirrored from old structs.rs analysis) ----

struct BitfieldGroup<'a> {
    first_xml_index: usize,
    backing_name: String,
    backing_ty: String,
    fields: Vec<BitfieldField<'a>>,
}

struct BitfieldField<'a> {
    #[allow(dead_code)]
    member: &'a xml::StructureMember,
    name: String,
    offset: u8,
    width: u8,
    setter_ty: String,
    is_flags_type: bool,
}

fn c_type_bit_width(name: &str) -> u8 {
    match name {
        "uint8_t" | "int8_t" => 8,
        "uint16_t" | "int16_t" => 16,
        "uint32_t" | "int32_t" => 32,
        "uint64_t" | "int64_t" => 64,
        _ => panic!("unhandled underlying type for bit field: {name}"),
    }
}

fn backing_type_for_fields(analysis: &Analysis, fields: &[BitfieldField<'_>]) -> String {
    fields
        .iter()
        .find_map(|f| {
            let name = ctype::base_name(&f.member.c_decl.ty)?;
            if analysis.is_bitmask_type(name) {
                None
            } else {
                Some(ctype::base_ctype_to_rust_str(name).to_string())
            }
        })
        .unwrap_or_else(|| "u32".to_string())
}

fn collect_bitfield_groups<'a>(
    analysis: &Analysis,
    members: &'a [xml::StructureMember],
) -> Vec<BitfieldGroup<'a>> {
    let mut groups = Vec::new();
    let mut i = 0;
    while i < members.len() {
        if members[i].c_decl.bitfield_width.is_some() {
            let mut start = i;
            let mut fields = Vec::new();
            let mut offset: u8 = 0;

            let type_width = {
                let mut width = 32u8;
                let mut j = i;
                while j < members.len() && members[j].c_decl.bitfield_width.is_some() {
                    let base = ctype::base_name(&members[j].c_decl.ty).unwrap_or("uint32_t");
                    if !analysis.is_bitmask_type(base) {
                        width = c_type_bit_width(base);
                        break;
                    }
                    j += 1;
                }
                width
            };

            while i < members.len() {
                if let Some(width) = members[i].c_decl.bitfield_width {
                    let w = width.get();

                    if offset > 0 && offset + w > type_width {
                        let backing_ty = backing_type_for_fields(analysis, &fields);
                        groups.push(BitfieldGroup {
                            first_xml_index: start,
                            backing_name: format!("_bitfield_{}", groups.len()),
                            backing_ty,
                            fields,
                        });
                        fields = Vec::new();
                        start = i;
                        offset = 0;
                    }

                    let name = normalize_name(members[i].c_decl.name);
                    let base_name = ctype::base_name(&members[i].c_decl.ty).unwrap_or("uint32_t");
                    let is_flags = analysis.is_bitmask_type(base_name);
                    let setter_ty = if w == 1 {
                        "bool".to_string()
                    } else if is_flags {
                        ctype_to_rust_type(analysis, &members[i].c_decl.ty, Some("a"))
                    } else {
                        ctype::base_ctype_to_rust_str(base_name).to_string()
                    };
                    fields.push(BitfieldField {
                        member: &members[i],
                        name,
                        offset,
                        width: w,
                        setter_ty,
                        is_flags_type: is_flags,
                    });
                    offset += w;
                    i += 1;
                } else {
                    break;
                }
            }

            if !fields.is_empty() {
                let backing_ty = backing_type_for_fields(analysis, &fields);
                groups.push(BitfieldGroup {
                    first_xml_index: start,
                    backing_name: format!("_bitfield_{}", groups.len()),
                    backing_ty,
                    fields,
                });
            }
        } else {
            i += 1;
        }
    }
    groups
}

fn debug_field_for_member(analysis: &Analysis, member: &xml::StructureMember) -> DebugField {
    let field_name = normalize_name(member.c_decl.name);
    let category = CTypeCategory::from_ctype(&member.c_decl.ty, analysis);
    match &category {
        CTypeCategory::CharPointer { .. } if member.len.contains(&"null-terminated") => {
            DebugField::CStrPtr(field_name)
        }
        CTypeCategory::FuncPointer => DebugField::FuncPointer(field_name),
        _ => DebugField::Normal(field_name),
    }
}

fn default_value_for_type(analysis: &Analysis, ty: &CType) -> FieldDefault {
    let category = CTypeCategory::from_ctype(ty, analysis);
    match category {
        CTypeCategory::Array { element, .. } => {
            if matches!(element, CType::Base(b) if b.name == "char") {
                FieldDefault::Zero // ArrayCStr uses Default::default()
            } else {
                FieldDefault::ArrayFill(Box::new(default_value_for_type(analysis, element)))
            }
        }
        CTypeCategory::OpaquePointer { is_const, .. }
        | CTypeCategory::CharPointer { is_const }
        | CTypeCategory::TypedPointer { is_const, .. } => FieldDefault::Null { mutable: !is_const },
        _ => FieldDefault::Zero,
    }
}

/// Determine setter assignment kind from C type.
fn setter_assignment_kind(analysis: &Analysis, ty: &CType) -> Option<ArrayAssignment> {
    let category = CTypeCategory::from_ctype(ty, analysis);
    match category {
        CTypeCategory::Array { .. } => Some(ArrayAssignment::CopyFromSlice),
        CTypeCategory::OpaquePointer { is_const, .. } | CTypeCategory::CharPointer { is_const } => {
            Some(ArrayAssignment::PtrFromSlice { is_const })
        }
        CTypeCategory::TypedPointer { is_const, pointee } => {
            if matches!(pointee, CType::Ptr { .. }) {
                Some(ArrayAssignment::PtrFromSlice { is_const })
            } else {
                Some(ArrayAssignment::PtrFromRef { is_const })
            }
        }
        _ => Some(ArrayAssignment::CopyFromSlice),
    }
}

/// Determine the value assignment kind for a value setter.
fn value_assignment_for_member(ty_str: &str, member: &xml::StructureMember) -> ValueAssignment {
    if ty_str.starts_with("PFN_") {
        ValueAssignment::WrapInSome
    } else if is_bool32(&member.c_decl.ty) {
        ValueAssignment::BoolInto
    } else {
        ValueAssignment::Direct
    }
}

/// Collect array members whose length is derived from `len_member_index`.
fn collect_array_params(
    analysis: &Analysis,
    struct_ty: &xml::Structure,
    len_kinds: &[Vec<LengthKind<'_>>],
    len_member_index: usize,
    lifetime_param: Option<&str>,
) -> Vec<(SetterParam, bool)> {
    let mut array_params = Vec::new();
    for (array_member_index, array_member) in struct_ty.members.iter().enumerate() {
        let member_len_kinds = len_kinds[array_member_index].as_slice();
        let len = member_len_kinds.iter().next();
        if let Some(LengthKind::Param { index, .. }) = len
            && *index == len_member_index
        {
            let optional: Vec<_> = array_member
                .optional
                .iter()
                .map(|s| s.parse::<bool>().unwrap())
                .collect();

            let array_param_ty = type_conv::convert_type(
                analysis,
                &array_member.c_decl.ty,
                &TypeRole::SetterParam {
                    lengths: member_len_kinds,
                    optional: &optional,
                    lifetime: lifetime_param,
                },
            );

            let mut name = normalize_setter_param_name(array_member.c_decl.name);
            let category = CTypeCategory::from_ctype(&array_member.c_decl.ty, analysis);
            if matches!(
                category,
                CTypeCategory::OpaquePointer {
                    pointee_name: "char",
                    ..
                }
            ) && let Some(stripped) = name.strip_suffix("_ptrs")
            {
                name = stripped.to_string();
            }
            let assignment = setter_assignment_kind(analysis, &array_member.c_decl.ty);
            let is_optional = optional.first().copied().unwrap_or(false);
            let noautovalidity = array_member.noautovalidity;

            array_params.push((
                SetterParam {
                    name,
                    field_index: array_member_index,
                    ty: array_param_ty,
                    optional: is_optional,
                    array_assign: assignment,
                },
                noautovalidity,
            ));
        }
    }
    array_params
}

fn group_array_params(array_params: Vec<(SetterParam, bool)>) -> Vec<Vec<SetterParam>> {
    if array_params.is_empty() {
        vec![vec![]]
    } else if array_params.len() >= 2 {
        vec![
            array_params
                .into_iter()
                .map(|(mut p, noauto)| {
                    if noauto {
                        p.optional = true;
                    }
                    p
                })
                .collect(),
        ]
    } else {
        array_params
            .into_iter()
            .map(|(mut p, _noauto)| {
                p.optional = false;
                vec![p]
            })
            .collect::<Vec<_>>()
    }
}

fn build_setters_for_length_member(
    analysis: &Analysis,
    struct_ty: &xml::Structure,
    member: &xml::StructureMember,
    member_index: usize,
    param_name: &str,
    array_param_groups: Vec<Vec<SetterParam>>,
    lifetime_param: Option<&str>,
) -> Vec<Setter> {
    let mut setters = Vec::new();
    for array_params in array_param_groups {
        let setter_name = if array_params.len() == 1 {
            array_params[0].name.clone()
        } else if !array_params.is_empty() {
            let base = param_name.strip_suffix("_count").unwrap_or(param_name);
            overrides::merged_setter_name(struct_ty.name, member.c_decl.name, base)
        } else {
            param_name.to_string()
        };

        let kind = if array_params.is_empty() {
            let optional: Vec<_> = member
                .optional
                .iter()
                .map(|s| s.parse::<bool>().unwrap())
                .collect();

            let param_ty = match overrides::member_type_override(member.c_decl.name) {
                Some(s) => RustType::named(s),
                None => type_conv::convert_type(
                    analysis,
                    &member.c_decl.ty,
                    &TypeRole::SetterParam {
                        lengths: &[],
                        optional: &optional,
                        lifetime: lifetime_param,
                    },
                ),
            };

            let ty_str = ctype_to_rust_type(analysis, &member.c_decl.ty, lifetime_param);
            let assignment = value_assignment_for_member(&ty_str, member);

            SetterKind::Value {
                param: SetterParam {
                    name: param_name.to_string(),
                    field_index: member_index,
                    ty: param_ty,
                    optional: false,
                    array_assign: None,
                },
                assignment,
            }
        } else {
            SetterKind::Array {
                len_field: member_index,
                params: array_params,
            }
        };

        setters.push(Setter {
            name: setter_name,
            kind,
        });
    }
    setters
}
