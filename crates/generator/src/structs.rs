use std::io::Write;

use anyhow::Result;

use crate::{
    LengthKind,
    analysis::Analysis,
    cdecl::CType,
    ctype_rust::{self, CTypeCategory, base_ctype_to_rust_str, is_bool32, type_name_with_lifetime},
    ctype_to_rust_type, get_len_kind, normalize_name, normalize_setter_param_name,
    normalize_ty_name, overrides, write_doc_link, xml,
};

pub fn generate_structs(
    file: &mut impl Write,
    analysis: &Analysis,
    structs: &[&xml::Structure],
) -> Result<()> {
    for ty in structs {
        write_struct(file, analysis, ty)?;
    }
    Ok(())
}

pub fn generate_unions(
    file: &mut impl Write,
    analysis: &Analysis,
    unions: &[&xml::Structure],
) -> Result<()> {
    for ty in unions {
        let name = normalize_ty_name(ty.name);
        let type_info = analysis.get_base_type_info(ty.name).unwrap();
        write_doc_link(file, ty.name)?;
        writeln!(
            file,
            "#[repr(C)]
            #[derive(Copy, Clone)]
            pub union {}{} {{",
            name,
            if type_info.lifetime_param { "<'a>" } else { "" }
        )?;
        for member in &ty.members {
            let field_ty = ctype_to_rust_type(analysis, &member.c_decl.ty, Some("a"));
            writeln!(
                file,
                "pub {}: {},",
                normalize_name(member.c_decl.name),
                field_ty
            )?;
        }
        if type_info.lifetime_param {
            writeln!(file, "pub _marker: PhantomData<&'a ()>,")?;
        }
        writeln!(file, "}}\n")?;
        let anon = if type_info.lifetime_param { "<'_>" } else { "" };
        writeln!(
            file,
            "#[cfg(feature = \"debug\")]
            impl fmt::Debug for {name}{anon} {{
                fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {{
                    f.debug_struct(\"{name}\").finish()
                }}
            }}\n"
        )?;
        writeln!(
            file,
            "impl Default for {name}{anon} {{
                fn default() -> Self {{
                    unsafe {{ core::mem::zeroed() }}
                }}
            }}\n"
        )?;
    }
    Ok(())
}

#[derive(Debug)]
struct StructInfo<'a> {
    name: String,
    /// The sType suffix (e.g. `"PHYSICAL_DEVICE_FEATURES_2"`), if this struct has a known sType.
    stype_suffix: Option<&'static str>,
    /// True if sType has a known value, enabling a manual Default impl that sets it.
    has_stype_default: bool,
    members: Vec<MemberInfo<'a>>,
    setters: Vec<MemberSetterInfo>,
    bitfield_groups: Vec<BitfieldGroup<'a>>,
}

/// A group of consecutive C bitfield members packed into a single Rust backing field.
#[derive(Debug)]
struct BitfieldGroup<'a> {
    /// Index of the first member in this group within the original XML members list.
    first_xml_index: usize,
    /// Name of the generated backing field (e.g., `_bitfield_0`).
    backing_name: String,
    /// Rust type of the backing field (e.g., `u32`).
    backing_ty: String,
    /// The individual bitfield members in this group.
    fields: Vec<BitfieldField<'a>>,
}

/// A single member within a bitfield group.
#[derive(Debug)]
struct BitfieldField<'a> {
    member: &'a xml::StructureMember,
    /// Normalized Rust name (e.g., `instance_custom_index`).
    name: String,
    /// Bit offset within the backing field.
    offset: u8,
    /// Number of bits.
    width: u8,
    /// Rust type for the setter parameter.
    setter_ty: String,
    /// Whether the type is a bitmask/flags type (needs `.as_raw()` to extract raw value).
    is_flags_type: bool,
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
        /// Index into `StructInfo::members` for the member that holds the array length.
        len_member_index: usize,
        params: Vec<SetterParamInfo>,
    },
    Bitfield {
        /// Index into `StructInfo::members` for the backing field.
        backing_member_index: usize,
        /// Bit offset within the backing field.
        offset: u8,
        /// Number of bits.
        width: u8,
        /// Rust type for the setter parameter.
        param_ty: String,
        /// Whether to use `.as_raw()` to extract the raw value from a flags type.
        is_flags_type: bool,
    },
}

#[derive(Debug)]
struct SetterParamInfo {
    name: String,
    /// Index into `StructInfo::members` for the struct field this parameter writes to.
    member_index: usize,
    /// Rust type string for this parameter in the setter signature.
    ty: String,
    /// How to assign this param to the struct member when emitting the setter body.
    assignment: SetterAssignmentKind,
    /// Whether this array member is optional (can be NULL).
    optional: bool,
}

/// Describes how to emit the assignment of a setter parameter to a struct member.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum SetterAssignmentKind {
    /// Copy slice into fixed array: `self.member[..param.len()].copy_from_slice(param)`
    CopyFromSlice,
    /// Assign raw pointer from slice (void*/char*): `self.member = param.as_ptr() as _` (or as_mut_ptr)
    PtrFromSlice { is_const: bool },
    /// Assign pointer from slice when pointee is double pointer: same as PtrFromSlice
    PtrFromRefNested { is_const: bool },
    /// Assign reference to pointer: `self.member = param.as_ptr()` (or as_mut_ptr)
    PtrFromRef { is_const: bool },
    /// Assign from CStr: `self.member = value.as_ptr()`
    CStrToPtr,
    /// Write CStr into fixed `[c_char; N]` array: `write_c_str_slice_with_nul(&mut self.member, value).expect(...)`
    CStrToArray,
}

#[derive(Debug)]
struct MemberInfo<'a> {
    member: &'a xml::StructureMember,
    name: String,
    ty: String,
}

/// Collect all array members whose length is derived from `len_member_index`.
fn collect_array_params(
    analysis: &Analysis,
    struct_ty: &xml::Structure,
    len_kinds: &[Vec<LengthKind<'_>>],
    len_member_index: usize,
    lifetime_param: Option<&str>,
) -> Vec<(SetterParamInfo, bool)> {
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

            let array_param_ty = convert_setter_param_type(
                analysis,
                &array_member.c_decl.ty,
                member_len_kinds,
                &optional,
                lifetime_param,
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
                SetterParamInfo {
                    name,
                    member_index: array_member_index,
                    ty: array_param_ty,
                    assignment,
                    optional: is_optional,
                },
                noautovalidity,
            ));
        }
    }
    array_params
}

/// Group array params into setter groups: multiple params sharing one length
/// get merged into a single setter; single params get their own setter.
/// Returns `vec![vec![]]` (one empty group) when there are no array params,
/// signaling the caller to generate a plain value setter for the length member itself.
fn group_array_params(array_params: Vec<(SetterParamInfo, bool)>) -> Vec<Vec<SetterParamInfo>> {
    if array_params.is_empty() {
        vec![vec![]]
    } else if array_params.len() >= 2 {
        // Merge all array params sharing this length into one setter.
        // Optional params and noautovalidity params become Option<...> in the signature.
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

/// Build setters for a length member and its associated array params.
fn build_setters_for_length_member(
    analysis: &Analysis,
    struct_ty: &xml::Structure,
    member: &xml::StructureMember,
    member_index: usize,
    param_name: &str,
    array_param_groups: Vec<Vec<SetterParamInfo>>,
    lifetime_param: Option<&str>,
) -> Vec<MemberSetterInfo> {
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

            let param_ty = overrides::member_type_override(member.c_decl.name)
                .map(|s| s.to_string())
                .unwrap_or_else(|| {
                    convert_setter_param_type(
                        analysis,
                        &member.c_decl.ty,
                        &[],
                        &optional,
                        lifetime_param,
                    )
                });
            SetterKind::Value(SetterParamInfo {
                name: param_name.to_string(),
                member_index,
                ty: param_ty,
                assignment: SetterAssignmentKind::CopyFromSlice, // unused for Value
                optional: false,
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
    setters
}

/// Determine the bit width of a C integer type name.
fn c_type_bit_width(name: &str) -> u8 {
    match name {
        "uint8_t" | "int8_t" => 8,
        "uint16_t" | "int16_t" => 16,
        "uint32_t" | "int32_t" => 32,
        "uint64_t" | "int64_t" => 64,
        _ => panic!("unhandled underlying type for bit field: {name}"),
    }
}

/// Scan struct members for consecutive bitfield members and group them.
/// Groups are split when the accumulated bit width reaches the backing type's total width.
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

            // Determine the backing type width from the first primitive integer member.
            let type_width = {
                let mut width = 32u8;
                let mut j = i;
                while j < members.len() && members[j].c_decl.bitfield_width.is_some() {
                    let base = ctype_rust::base_name(&members[j].c_decl.ty).unwrap_or("uint32_t");
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

                    // If adding this field would exceed the backing type width, close the
                    // current group and start a new one.
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
                    let base_name =
                        ctype_rust::base_name(&members[i].c_decl.ty).unwrap_or("uint32_t");
                    let is_flags = analysis.is_bitmask_type(base_name);
                    let setter_ty = if w == 1 {
                        "bool".to_string()
                    } else if is_flags {
                        ctype_to_rust_type(analysis, &members[i].c_decl.ty, Some("a"))
                    } else {
                        ctype_rust::base_ctype_to_rust_str(base_name).to_string()
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

/// Determine the Rust backing type for a group of bitfield fields.
fn backing_type_for_fields(analysis: &Analysis, fields: &[BitfieldField<'_>]) -> String {
    fields
        .iter()
        .find_map(|f| {
            let name = ctype_rust::base_name(&f.member.c_decl.ty)?;
            if analysis.is_bitmask_type(name) {
                None
            } else {
                Some(ctype_rust::base_ctype_to_rust_str(name).to_string())
            }
        })
        .unwrap_or_else(|| "u32".to_string())
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

    let mut has_stype_default = true;
    let mut stype_suffix = None;
    let lifetime_param = Some("a");

    let mut members = Vec::new();
    let mut setters = Vec::new();

    // Pre-compute bitfield groups.
    let bitfield_groups = collect_bitfield_groups(analysis, &struct_ty.members);

    // Build a lookup: xml_member_index -> (group_index, is_first_in_group).
    let mut bitfield_membership = std::collections::HashMap::new();
    for (group_idx, group) in bitfield_groups.iter().enumerate() {
        for (field_idx, _field) in group.fields.iter().enumerate() {
            let xml_idx = group.first_xml_index + field_idx;
            bitfield_membership.insert(xml_idx, (group_idx, field_idx == 0));
        }
    }

    // Mapping from XML member index to analyzed members Vec index.
    let mut xml_to_analyzed: Vec<Option<usize>> = vec![None; struct_ty.members.len()];

    for (member_index, member) in struct_ty.members.iter().enumerate() {
        // Handle bitfield group members.
        if let Some(&(group_idx, is_first)) = bitfield_membership.get(&member_index) {
            if is_first {
                let group = &bitfield_groups[group_idx];
                let backing_member_index = members.len();
                // Map all XML indices in this group to the backing field index.
                for field_idx in 0..group.fields.len() {
                    xml_to_analyzed[group.first_xml_index + field_idx] = Some(backing_member_index);
                }
                members.push(MemberInfo {
                    member,
                    name: group.backing_name.clone(),
                    ty: group.backing_ty.clone(),
                });

                for field in &group.fields {
                    setters.push(MemberSetterInfo {
                        name: field.name.clone(),
                        kind: SetterKind::Bitfield {
                            backing_member_index,
                            offset: field.offset,
                            width: field.width,
                            param_ty: field.setter_ty.clone(),
                            is_flags_type: field.is_flags_type,
                        },
                    });
                }
            }
            continue;
        }

        let analyzed_index = members.len();
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

        // Null-terminated string fields get a setter taking &CStr.
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
                setters.push(MemberSetterInfo {
                    name: param_name.clone(),
                    kind: SetterKind::Value(SetterParamInfo {
                        name: param_name,
                        member_index: analyzed_index,
                        ty: "&'a CStr".to_string(),
                        assignment: SetterAssignmentKind::CStrToPtr,
                        optional: false,
                    }),
                });
            } else if is_char_array {
                let param_name = normalize_setter_param_name(member.c_decl.name);
                setters.push(MemberSetterInfo {
                    name: param_name.clone(),
                    kind: SetterKind::Value(SetterParamInfo {
                        name: param_name,
                        member_index: analyzed_index,
                        ty: "&CStr".to_string(),
                        assignment: SetterAssignmentKind::CStrToArray,
                        optional: false,
                    }),
                });

                let ty = ctype_to_rust_type(analysis, &member.c_decl.ty, lifetime_param);
                members.push(MemberInfo { member, name, ty });
                continue; // CStr-to-array: skip the general setter logic below
            }
        }

        // Members without a length annotation: either plain values or length-count fields.
        // collect_array_params checks whether other members reference this one as their length.
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

        let ty = ctype_to_rust_type(analysis, &member.c_decl.ty, lifetime_param);
        members.push(MemberInfo { member, name, ty });
    }

    // Remap XML member indices in setters to analyzed member indices.
    for setter in &mut setters {
        match &mut setter.kind {
            SetterKind::Value(param) => {
                if let Some(analyzed) = xml_to_analyzed.get(param.member_index).copied().flatten() {
                    param.member_index = analyzed;
                }
            }
            SetterKind::Array {
                len_member_index,
                params,
            } => {
                if let Some(analyzed) = xml_to_analyzed.get(*len_member_index).copied().flatten() {
                    *len_member_index = analyzed;
                }
                for param in params {
                    if let Some(analyzed) =
                        xml_to_analyzed.get(param.member_index).copied().flatten()
                    {
                        param.member_index = analyzed;
                    }
                }
            }
            SetterKind::Bitfield { .. } => {} // Already uses analyzed index.
        }
    }

    // Remove setters that overrides want suppressed (e.g. reserved fields).
    setters.retain(|s| !overrides::skip_setter(struct_ty.name, &s.name));

    let name = normalize_ty_name(struct_ty.name).to_string();
    StructInfo {
        name,
        members,
        stype_suffix,
        has_stype_default,
        setters,
        bitfield_groups,
    }
}

fn write_struct_definition(
    file: &mut impl std::io::Write,
    analysis: &Analysis,
    ty: &xml::Structure,
    type_info: crate::analysis::TypeInfo,
    has_derived_default: bool,
    lifetime_spec: &str,
    bitfield_groups: &[BitfieldGroup<'_>],
) -> Result<()> {
    let mut derives = vec!["Copy", "Clone"];
    if has_derived_default {
        derives.push("Default");
    }
    let derives_str = derives.join(", ");

    crate::write_doc_link(file, ty.name)?;
    writeln!(file, "#[repr(C)]")?;
    if type_info.trivial_debug {
        writeln!(file, "#[cfg_attr(feature = \"debug\", derive(Debug))]")?;
    }
    writeln!(
        file,
        "#[derive({derives_str})]
        #[must_use]
        pub struct {}{} {{",
        normalize_ty_name(ty.name),
        lifetime_spec
    )?;

    // Build a set of XML member indices that belong to bitfield groups,
    // and track which indices start a group.
    let mut bitfield_skip: std::collections::HashSet<usize> = std::collections::HashSet::new();
    let mut bitfield_start: std::collections::HashMap<usize, usize> =
        std::collections::HashMap::new();
    for (group_idx, group) in bitfield_groups.iter().enumerate() {
        for field_idx in 0..group.fields.len() {
            let xml_idx = group.first_xml_index + field_idx;
            if field_idx == 0 {
                bitfield_start.insert(xml_idx, group_idx);
            } else {
                bitfield_skip.insert(xml_idx);
            }
        }
    }

    for (i, member) in ty.members.iter().enumerate() {
        if bitfield_skip.contains(&i) {
            continue;
        }

        if let Some(&group_idx) = bitfield_start.get(&i) {
            let group = &bitfield_groups[group_idx];
            writeln!(file, "pub {}: {},", group.backing_name, group.backing_ty)?;
            continue;
        }

        let name = normalize_name(member.c_decl.name);

        let field_ty =
            if let Some(override_ty) = overrides::member_type_override(member.c_decl.name) {
                override_ty.to_string()
            } else {
                let field_ty = ctype_to_rust_type(analysis, &member.c_decl.ty, Some("a"));
                let category = CTypeCategory::from_ctype(&member.c_decl.ty, analysis);
                match category {
                    CTypeCategory::FuncPointer => format!("Option<{field_ty}>"),
                    _ => field_ty,
                }
            };

        writeln!(file, "pub {name}: {field_ty},")?;
    }
    if type_info.lifetime_param {
        writeln!(file, "pub _marker: PhantomData<&'a ()>,")?;
    }
    writeln!(file, "}}\n")?;
    Ok(())
}

fn write_trait_impls(
    file: &mut impl std::io::Write,
    analysis: &Analysis,
    ty: &xml::Structure,
    info: &StructInfo<'_>,
) -> Result<()> {
    if let Some(tag) = info.stype_suffix {
        writeln!(
            file,
            "unsafe impl<'a> TaggedStructure<'a> for {}<'a> {{
                const STRUCTURE_TYPE: StructureType = StructureType::{};
            }}\n",
            info.name, tag
        )?;
    }

    for extends in &ty.structextends {
        let rust_ty = base_ctype_to_rust_str(extends);
        if analysis.is_provisional_type(extends) {
            writeln!(file, "#[cfg(feature = \"provisional\")]")?;
        }
        writeln!(
            file,
            "unsafe impl Extends<{}<'_>> for {}<'_> {{}}",
            rust_ty, info.name
        )?;
    }
    if !ty.structextends.is_empty() {
        writeln!(file)?;
    }
    Ok(())
}

fn write_default_impl(
    file: &mut impl std::io::Write,
    analysis: &Analysis,
    info: &StructInfo<'_>,
    type_info: crate::analysis::TypeInfo,
    lifetime_spec_anon: &str,
) -> Result<()> {
    writeln!(
        file,
        "impl Default for {}{} {{
        fn default() -> Self {{
        Self {{",
        info.name, lifetime_spec_anon
    )?;
    for member in &info.members {
        write!(file, "{}: ", member.name)?;
        if member.member.c_decl.name == "sType" {
            writeln!(file, "Self::STRUCTURE_TYPE")?
        } else {
            write!(
                file,
                "{}",
                default_value(analysis, &member.member.c_decl.ty)
            )?;
        }
        writeln!(file, ",")?;
    }
    if type_info.lifetime_param {
        writeln!(file, "_marker: PhantomData")?;
    }
    writeln!(file, "}} }} }}\n")?;
    Ok(())
}

fn write_value_setter_body(
    file: &mut impl std::io::Write,
    param: &SetterParamInfo,
    member: &MemberInfo<'_>,
) -> Result<()> {
    match param.assignment {
        SetterAssignmentKind::CStrToPtr => {
            writeln!(file, "self.{} = {}.as_ptr();", member.name, param.name)?;
        }
        SetterAssignmentKind::CStrToArray => {
            writeln!(
                file,
                "write_c_str_slice_with_nul(&mut self.{}, {})?;",
                member.name, param.name
            )?;
        }
        _ => {
            if member.ty.starts_with("PFN_") {
                writeln!(file, "self.{} = Some({});", member.name, param.name)?;
            } else if is_bool32(&member.member.c_decl.ty) {
                writeln!(file, "self.{} = {}.into();", member.name, param.name)?;
            } else {
                writeln!(file, "self.{} = {};", member.name, param.name)?;
            }
        }
    }
    Ok(())
}

fn write_bitfield_setter_body(
    file: &mut impl Write,
    backing_field_name: &str,
    backing_ty: &str,
    param_name: &str,
    offset: u8,
    width: u8,
    is_flags_type: bool,
) -> Result<()> {
    assert!(
        backing_ty == "u32",
        "TODO: handle bitfield backing type `{backing_ty}` (only u32 is supported)",
    );
    if width == 1 {
        writeln!(
            file,
            "set_bitfield_bool::<{offset}>(&mut self.{backing_field_name}, {param_name});",
        )?;
    } else {
        let raw_val = if is_flags_type {
            format!("{param_name}.as_raw()")
        } else {
            param_name.to_string()
        };
        writeln!(
            file,
            "set_bitfield::<{offset}, {width}>(&mut self.{backing_field_name}, {raw_val});",
        )?;
    }
    Ok(())
}

fn write_array_setter_body(
    file: &mut impl std::io::Write,
    info: &StructInfo<'_>,
    len_member_index: usize,
    params: &[SetterParamInfo],
) -> Result<()> {
    // Find the first required (non-optional) param to derive length from.
    let first_required = params.iter().find(|p| !p.optional);
    let len_source = first_required.unwrap_or(&params[0]);

    let len_member = &info.members[len_member_index];

    if first_required.is_none() {
        // All params are optional — derive length from first that is Some.
        write!(file, "self.{} = None", len_member.name)?;
        for param in params {
            write!(
                file,
                ".or_else(|| {}.as_deref().map(|s| s.len()))",
                param.name
            )?;
        }
        writeln!(file, ".unwrap_or(0).try_into().unwrap();")?;
    } else {
        writeln!(
            file,
            "self.{} = {}.len().try_into().unwrap();",
            len_member.name, len_source.name
        )?;
    }

    // Assert matching lengths for all other params.
    for param in params {
        if std::ptr::eq(param, len_source) {
            continue;
        }
        if param.optional {
            writeln!(
                file,
                "if let Some(s) = &{name} {{ assert_eq!(s.len(), self.{len} as usize); }}",
                name = param.name,
                len = len_member.name,
            )?;
        } else {
            writeln!(
                file,
                "assert_eq!({}.len(), self.{} as usize);",
                param.name, len_member.name
            )?;
        }
    }

    // Emit assignments.
    for param in params {
        let member = &info.members[param.member_index];
        if param.optional {
            emit_optional_setter_assignment(file, &member.name, &param.name, param.assignment)?;
        } else {
            emit_setter_assignment(file, &member.name, &param.name, param.assignment)?;
        }
    }
    Ok(())
}

fn write_setters(
    file: &mut impl std::io::Write,
    info: &StructInfo<'_>,
    lifetime_spec: &str,
) -> Result<()> {
    writeln!(
        file,
        "impl{} {}{}{{",
        lifetime_spec, info.name, lifetime_spec
    )?;
    for setter in &info.setters {
        writeln!(
            file,
            "#[inline]
            pub fn {}(mut self,",
            setter.name
        )?;

        match &setter.kind {
            SetterKind::Value(param) => {
                writeln!(file, "{}: {},", param.name, param.ty)?;
            }
            SetterKind::Array { params, .. } => {
                for param in params {
                    if param.optional {
                        writeln!(file, "{}: Option<{}>,", param.name, param.ty)?;
                    } else {
                        writeln!(file, "{}: {},", param.name, param.ty)?;
                    }
                }
            }
            SetterKind::Bitfield { param_ty, .. } => {
                writeln!(file, "{}: {},", setter.name, param_ty)?;
            }
        }

        let is_cstr_array = matches!(&setter.kind, SetterKind::Value(p) if p.assignment == SetterAssignmentKind::CStrToArray);
        if is_cstr_array {
            writeln!(
                file,
                ") -> core::result::Result<Self, CStrTooLargeForStaticArray> {{"
            )?;
        } else {
            writeln!(file, ") -> Self {{")?;
        }

        match &setter.kind {
            SetterKind::Value(param) => {
                write_value_setter_body(file, param, &info.members[param.member_index])?;
            }
            SetterKind::Array {
                len_member_index,
                params,
            } => {
                write_array_setter_body(file, info, *len_member_index, params)?;
            }
            SetterKind::Bitfield {
                backing_member_index,
                offset,
                width,
                is_flags_type,
                ..
            } => {
                write_bitfield_setter_body(
                    file,
                    &info.members[*backing_member_index].name,
                    &info.members[*backing_member_index].ty,
                    &setter.name,
                    *offset,
                    *width,
                    *is_flags_type,
                )?;
            }
        }

        if is_cstr_array {
            writeln!(file, "Ok(self) }}\n")?;
        } else {
            writeln!(file, "self }}\n")?;
        }
    }
    writeln!(file, "}}\n")?;
    Ok(())
}

pub fn write_struct(
    file: &mut impl std::io::Write,
    analysis: &Analysis,
    ty: &xml::Structure,
) -> Result<()> {
    let info = analyze_struct(analysis, ty);
    let type_info = analysis.get_base_type_info(ty.name).unwrap();

    let lifetime_spec = if type_info.lifetime_param { "<'a>" } else { "" };
    let lifetime_spec_anon = if type_info.lifetime_param { "<'_>" } else { "" };

    // True when zeroed memory is a valid default — use #[derive(Default)] instead of manual impl.
    let has_derived_default = info.has_stype_default && type_info.default;

    write_struct_definition(
        file,
        analysis,
        ty,
        type_info,
        has_derived_default,
        lifetime_spec,
        &info.bitfield_groups,
    )?;

    if !type_info.trivial_debug {
        writeln!(file, "#[cfg(feature = \"debug\")]")?;
        write_debug_impl(file, analysis, ty, &info, type_info)?;
    }

    write_trait_impls(file, analysis, ty, &info)?;

    // Manual Default impl needed: sType must be set, and zeroed memory isn't a valid default.
    if info.has_stype_default && !type_info.default {
        write_default_impl(file, analysis, &info, type_info, lifetime_spec_anon)?;
    }

    write_setters(file, &info, lifetime_spec)?;
    Ok(())
}

/// Classifies how a struct member should be formatted in a Debug impl.
enum DebugFieldKind {
    /// Normal `Debug::fmt` formatting (also used for raw pointers).
    Normal,
    /// `*const c_char` with null-terminated semantics → display via `as_c_str`.
    CStrPtr,
    /// `[c_char; N]` → display via `wrap_c_str_slice_until_nul`.
    CStrArray,
    /// `Option<PFN_*>` function pointer → display with `.map(|f| f as *const ())`.
    FuncPointer,
}

fn debug_field_kind(analysis: &Analysis, member: &xml::StructureMember) -> DebugFieldKind {
    let ty = &member.c_decl.ty;
    let category = CTypeCategory::from_ctype(ty, analysis);

    match &category {
        // *const c_char with null-terminated length → CStr display
        CTypeCategory::CharPointer { .. } => {
            if member.len.contains(&"null-terminated") {
                DebugFieldKind::CStrPtr
            } else {
                DebugFieldKind::Normal
            }
        }
        // [c_char; N] → CStr display
        CTypeCategory::Array { element, .. } => {
            if matches!(element, CType::Base(b) if b.name == "char") {
                DebugFieldKind::CStrArray
            } else {
                DebugFieldKind::Normal
            }
        }
        // Any pointer → normal Debug (prints address)
        CTypeCategory::OpaquePointer { .. } | CTypeCategory::TypedPointer { .. } => {
            DebugFieldKind::Normal
        }
        // Function pointer (wrapped in Option) → show as pointer
        CTypeCategory::FuncPointer => DebugFieldKind::FuncPointer,
        // Everything else
        _ => DebugFieldKind::Normal,
    }
}

fn write_debug_impl(
    file: &mut impl std::io::Write,
    analysis: &Analysis,
    ty: &xml::Structure,
    info: &StructInfo<'_>,
    type_info: crate::analysis::TypeInfo,
) -> Result<()> {
    let name = &info.name;
    let lifetime_spec_anon = if type_info.lifetime_param { "<'_>" } else { "" };

    writeln!(
        file,
        "impl fmt::Debug for {name}{lifetime_spec_anon} {{
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {{
            f.debug_struct(\"{name}\")"
    )?;

    // Build bitfield lookup (same as write_struct_definition).
    let mut bitfield_skip: std::collections::HashSet<usize> = std::collections::HashSet::new();
    let mut bitfield_start: std::collections::HashMap<usize, usize> =
        std::collections::HashMap::new();
    for (group_idx, group) in info.bitfield_groups.iter().enumerate() {
        for field_idx in 0..group.fields.len() {
            let xml_idx = group.first_xml_index + field_idx;
            if field_idx == 0 {
                bitfield_start.insert(xml_idx, group_idx);
            } else {
                bitfield_skip.insert(xml_idx);
            }
        }
    }

    for (i, member) in ty.members.iter().enumerate() {
        if bitfield_skip.contains(&i) {
            continue;
        }

        if let Some(&group_idx) = bitfield_start.get(&i) {
            let group = &info.bitfield_groups[group_idx];
            // Show each logical bitfield member by extracting bits.
            for field in &group.fields {
                let mask = (1u64 << field.width) - 1;
                let backing = &group.backing_name;
                if field.offset == 0 {
                    writeln!(
                        file,
                        ".field(\"{}\", &(self.{} & {:#x}))",
                        field.name, backing, mask
                    )?;
                } else {
                    writeln!(
                        file,
                        ".field(\"{}\", &((self.{} >> {}) & {:#x}))",
                        field.name, backing, field.offset, mask
                    )?;
                }
            }
            continue;
        }

        let field_name = normalize_name(member.c_decl.name);
        let kind = debug_field_kind(analysis, member);

        match kind {
            DebugFieldKind::Normal => {
                writeln!(file, ".field(\"{field_name}\", &self.{field_name})")?;
            }
            DebugFieldKind::CStrPtr => {
                writeln!(
                    file,
                    ".field(\"{field_name}\", &unsafe {{ as_c_str(self.{field_name}) }})"
                )?;
            }
            DebugFieldKind::CStrArray => {
                writeln!(
                    file,
                    ".field(\"{field_name}\", &wrap_c_str_slice_until_nul(&self.{field_name}))"
                )?;
            }
            DebugFieldKind::FuncPointer => {
                writeln!(
                    file,
                    ".field(\"{field_name}\", &self.{field_name}.map(|f| f as *const ()))"
                )?;
            }
        }
    }

    writeln!(file, ".finish()\n}} }}\n")?;
    Ok(())
}

fn default_value(analysis: &Analysis, ty: &CType) -> std::borrow::Cow<'static, str> {
    let category = CTypeCategory::from_ctype(ty, analysis);
    match category {
        CTypeCategory::Array { element, .. } => {
            format!("[{}; _]", default_value(analysis, element)).into()
        }
        CTypeCategory::OpaquePointer { is_const, .. }
        | CTypeCategory::CharPointer { is_const }
        | CTypeCategory::TypedPointer { is_const, .. } => {
            if is_const {
                "ptr::null()".into()
            } else {
                "ptr::null_mut()".into()
            }
        }
        _ => "Default::default()".into(),
    }
}

fn setter_assignment_kind(analysis: &Analysis, ty: &CType) -> SetterAssignmentKind {
    use CTypeCategory;
    let category = CTypeCategory::from_ctype(ty, analysis);
    match category {
        CTypeCategory::Array { .. } => SetterAssignmentKind::CopyFromSlice,
        CTypeCategory::OpaquePointer { is_const, .. } | CTypeCategory::CharPointer { is_const } => {
            SetterAssignmentKind::PtrFromSlice { is_const }
        }
        CTypeCategory::TypedPointer { is_const, pointee } => {
            if matches!(pointee, CType::Ptr { .. }) {
                SetterAssignmentKind::PtrFromRefNested { is_const }
            } else {
                SetterAssignmentKind::PtrFromRef { is_const }
            }
        }
        _ => SetterAssignmentKind::CopyFromSlice, // fallback
    }
}

fn emit_setter_assignment(
    file: &mut impl std::io::Write,
    member_name: &str,
    param_name: &str,
    kind: SetterAssignmentKind,
) -> Result<()> {
    match kind {
        SetterAssignmentKind::CopyFromSlice => {
            writeln!(
                file,
                "self.{member_name}[..{param_name}.len()].copy_from_slice({param_name});"
            )?;
        }
        SetterAssignmentKind::PtrFromSlice { is_const } => {
            if is_const {
                writeln!(file, "self.{member_name} = {param_name}.as_ptr() as _;")?;
            } else {
                writeln!(file, "self.{member_name} = {param_name}.as_mut_ptr() as _;")?;
            }
        }
        SetterAssignmentKind::PtrFromRefNested { is_const } => {
            if is_const {
                writeln!(file, "self.{member_name} = {param_name}.as_ptr() as _;")?;
            } else {
                writeln!(file, "self.{member_name} = {param_name}.as_mut_ptr() as _;")?;
            }
        }
        SetterAssignmentKind::PtrFromRef { is_const } => {
            if is_const {
                writeln!(file, "self.{member_name} = {param_name}.as_ptr() as _;")?;
            } else {
                writeln!(file, "self.{member_name} = {param_name}.as_mut_ptr() as _;")?;
            }
        }
        SetterAssignmentKind::CStrToPtr | SetterAssignmentKind::CStrToArray => {
            unreachable!("CStr assignments only used for Value setters")
        }
    }
    Ok(())
}

/// Emit assignment for an optional array param (`Option<&[T]>` → pointer or null).
fn emit_optional_setter_assignment(
    file: &mut impl std::io::Write,
    member_name: &str,
    param_name: &str,
    kind: SetterAssignmentKind,
) -> Result<()> {
    // For optional params, emit: if let Some(s) = param { ptr } else { null }
    let (map_expr, null_expr) = match kind {
        SetterAssignmentKind::CopyFromSlice => {
            // Optional fixed-array copy: copy if Some, leave unchanged if None
            writeln!(
                file,
                "if let Some(s) = {param_name} {{ self.{member_name}[..s.len()].copy_from_slice(s); }}"
            )?;
            return Ok(());
        }
        SetterAssignmentKind::PtrFromSlice { is_const }
        | SetterAssignmentKind::PtrFromRefNested { is_const } => {
            if is_const {
                ("|s| s.as_ptr() as _", "ptr::null()")
            } else {
                ("|s| s.as_mut_ptr() as _", "ptr::null_mut()")
            }
        }
        SetterAssignmentKind::PtrFromRef { is_const } => {
            if is_const {
                ("|s| s.as_ptr() as _", "ptr::null()")
            } else {
                ("|s| s.as_mut_ptr() as _", "ptr::null_mut()")
            }
        }
        SetterAssignmentKind::CStrToPtr | SetterAssignmentKind::CStrToArray => {
            unreachable!("CStr assignments only used for Value setters")
        }
    };
    writeln!(
        file,
        "self.{member_name} = {param_name}.map_or({null_expr}, {map_expr});"
    )?;
    Ok(())
}

pub fn convert_setter_param_type(
    analysis: &Analysis,
    ty: &CType,
    lengths: &[LengthKind<'_>],
    optional: &[bool],
    lifetime_param: Option<&str>,
) -> String {
    use CTypeCategory;

    if let Some(len) = lengths.first()
        && !matches!(len, LengthKind::Literal(1))
    {
        let category = CTypeCategory::from_ctype(ty, analysis);
        return match category {
            CTypeCategory::CharPointer { is_const } => {
                if is_const {
                    "&'a CStr".to_string()
                } else {
                    "&'a mut CStr".to_string()
                }
            }
            // Ptr to Ptr to char (e.g. const char* const*) with length: original recursed to *const c_char, so &'a [*const c_char]
            CTypeCategory::OpaquePointer {
                pointee_name: "char",
                is_const,
            } => {
                if is_const {
                    "&'a [*const c_char]".to_string()
                } else {
                    "&'a mut [*mut c_char]".to_string()
                }
            }
            // void* with length becomes &[u8] / &mut [u8]
            CTypeCategory::OpaquePointer {
                pointee_name: "void",
                is_const,
            } => {
                if is_const {
                    "&'a [u8]".to_string()
                } else {
                    "&'a mut [u8]".to_string()
                }
            }
            // Ptr to Ptr to T (e.g. const VkAccelerationStructureGeometryKHR**) with length → slice of raw pointers
            // Exception: Ptr to Ptr to known non-opaque struct (e.g. const VkMicromapUsageEXT* const*) → &'a [&'a T]
            CTypeCategory::OpaquePointer {
                pointee_name,
                is_const,
            } => {
                // Use &[&T] when the C type is T** and T is a known non-opaque struct.
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
                if use_slice_of_refs {
                    let inner = type_name_with_lifetime(analysis, pointee_name, None);
                    if is_const {
                        format!("&'a [&'a {inner}]")
                    } else {
                        format!("&'a mut [&'a mut {inner}]")
                    }
                } else {
                    let inner = type_name_with_lifetime(analysis, pointee_name, None);
                    if is_const {
                        format!("&'a [*const {inner}]")
                    } else {
                        format!("&'a mut [*mut {inner}]")
                    }
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
                let element_ty =
                    convert_setter_param_type(analysis, pointee, rest_lengths, rest_optional, None);
                // Bool conversion must not apply to array elements since
                // bool and Bool32 (u32) differ in memory layout.
                let element_ty = if element_ty == "bool" {
                    "Bool32".to_string()
                } else if element_ty == "c_void" {
                    "u8".to_string()
                } else {
                    element_ty
                };
                if is_const {
                    format!("&'a [{element_ty}]")
                } else {
                    format!("&'a mut [{element_ty}]")
                }
            }
            CTypeCategory::Array { element, .. } => {
                let element_ty = ctype_to_rust_type(analysis, element, None);
                format!("&[{element_ty}]")
            }
            _ => panic!("expected pointer or array type because of length {len:?}, got {ty:?}"),
        };
    }

    let category = CTypeCategory::from_ctype(ty, analysis);
    match category {
        CTypeCategory::OpaquePointer {
            is_const,
            pointee_name,
        } => {
            let ty = type_name_with_lifetime(analysis, pointee_name, lifetime_param);
            if is_const {
                format!("*const {ty}")
            } else {
                format!("*mut {ty}")
            }
        }
        CTypeCategory::CharPointer { is_const } => {
            if is_const {
                "*const c_char".to_string()
            } else {
                "*mut c_char".to_string()
            }
        }
        CTypeCategory::TypedPointer { is_const, pointee } => {
            let ty = ctype_to_rust_type(analysis, pointee, lifetime_param);
            let is_opaque = analysis.is_opaque_type(pointee);
            if is_opaque {
                if is_const {
                    format!("*const {ty}")
                } else {
                    format!("*mut {ty}")
                }
            } else if is_const {
                format!("&'a {ty}")
            } else {
                format!("&'a mut {ty}")
            }
        }
        _ => {
            if is_bool32(ty) {
                return "bool".to_string();
            }
            ctype_to_rust_type(analysis, ty, lifetime_param)
        }
    }
}
