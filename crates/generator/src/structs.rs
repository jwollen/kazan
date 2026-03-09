use std::{collections::HashSet, io::Write};

use crate::{
    LengthKind, analysis::Analysis, cdecl::CType, ctype_rust, ctype_to_rust_type,
    ctype_to_rust_type_str, get_len_kind, normalize_name, normalize_setter_param_name,
    normalize_ty_name, overrides, write_doc_link, xml,
};

pub fn generate_structs(file: &mut impl Write, analysis: &Analysis, owned: &HashSet<&str>) {
    let new_structs = analysis
        .registry()
        .structs
        .iter()
        .filter(|ty| owned.contains(ty.name));

    for ty in new_structs {
        write_struct(file, analysis, ty);
    }
}

pub fn generate_unions(file: &mut impl Write, analysis: &Analysis, owned: &HashSet<&str>) {
    let unions = analysis
        .registry()
        .unions
        .iter()
        .filter(|ty| owned.contains(ty.name));
    for ty in unions {
        let name = normalize_ty_name(ty.name);
        let type_info = analysis.get_base_type_info(ty.name).unwrap();
        write_doc_link(file, ty.name);
        writeln!(
            file,
            "#[repr(C)]
            #[derive(Copy, Clone)]
            pub union {}{} {{",
            name,
            if type_info.lifetime_param { "<'a>" } else { "" }
        )
        .unwrap();
        for member in &ty.members {
            let field_ty = ctype_to_rust_type(analysis, &member.c_decl.ty, Some("a"));
            writeln!(
                file,
                "pub {}: {},",
                normalize_name(member.c_decl.name),
                field_ty
            )
            .unwrap();
        }
        if type_info.lifetime_param {
            writeln!(file, "pub _marker: PhantomData<&'a ()>,",).unwrap();
        }
        writeln!(file, "}}\n").unwrap();
        let anon = if type_info.lifetime_param { "<'_>" } else { "" };
        writeln!(
            file,
            "#[cfg(feature = \"debug\")]
            impl fmt::Debug for {name}{anon} {{
                fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {{
                    f.debug_struct(\"{name}\").finish()
                }}
            }}\n"
        )
        .unwrap();
        writeln!(
            file,
            "impl Default for {name}{anon} {{
                fn default() -> Self {{
                    unsafe {{ core::mem::zeroed() }}
                }}
            }}\n"
        )
        .unwrap();
    }
}

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

        // Null-terminated string fields get a setter taking &CStr.
        if len.len() == 1
            && matches!(len.first(), Some(LengthKind::NullTerminated))
            && !is_special_member
        {
            let category = ctype_rust::CTypeCategory::from_ctype(&member.c_decl.ty, analysis);
            let is_char_ptr = matches!(
                category,
                ctype_rust::CTypeCategory::CharPointer { .. }
                    | ctype_rust::CTypeCategory::OpaquePointer {
                        pointee_name: "char",
                        ..
                    }
            );
            let is_char_array = matches!(
                &category,
                ctype_rust::CTypeCategory::Array { element, .. }
                    if matches!(element, CType::Base(b) if b.name == "char")
            );
            if is_char_ptr {
                let param_name = normalize_setter_param_name(member.c_decl.name);
                setters.push(MemberSetterInfo {
                    name: param_name.clone(),
                    kind: SetterKind::Value(SetterParamInfo {
                        name: param_name,
                        member_index,
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
                        member_index,
                        ty: "&CStr".to_string(),
                        assignment: SetterAssignmentKind::CStrToArray,
                        optional: false,
                    }),
                });

                let ty = ctype_to_rust_type(analysis, &member.c_decl.ty, lifetime_param);
                members.push(MemberInfo {
                    member,
                    name,
                    ty,
                    len,
                });
                continue;
            }
        }

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

                    let mut name = normalize_setter_param_name(array_member.c_decl.name);
                    let category =
                        ctype_rust::CTypeCategory::from_ctype(&array_member.c_decl.ty, analysis);
                    if matches!(
                        category,
                        ctype_rust::CTypeCategory::OpaquePointer {
                            pointee_name: "char",
                            ..
                        }
                    ) {
                        if let Some(stripped) = name.strip_suffix("_ptrs") {
                            name = stripped.to_string();
                        }
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

            let array_param_groups = if array_params.is_empty() {
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
            };

            for array_params in array_param_groups {
                let setter_name = if array_params.len() == 1 {
                    array_params[0].name.clone()
                } else if !array_params.is_empty() {
                    let base = if let Some(name) = param_name.strip_suffix("_count") {
                        name
                    } else {
                        println!(
                            "length member that doesn't end in 'count': {}.{}",
                            struct_ty.name, member.c_decl.name
                        );
                        &param_name
                    };
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

    let mut derives = vec!["Copy", "Clone"];
    if info.has_default && type_info.default {
        derives.push("Default");
    }
    let derives_str = derives.join(", ");

    crate::write_doc_link(file, ty.name);
    writeln!(file, "#[repr(C)]").unwrap();
    if type_info.trivial_debug {
        writeln!(file, "#[cfg_attr(feature = \"debug\", derive(Debug))]").unwrap();
    }
    writeln!(
        file,
        "#[derive({derives_str})]
        #[must_use]
        pub struct {}{} {{",
        normalize_ty_name(ty.name),
        lifetime_spec
    )
    .unwrap();
    for member in &ty.members {
        let name = normalize_name(member.c_decl.name);

        let field_ty = ctype_to_rust_type(analysis, &member.c_decl.ty, Some("a"));
        let field_ty = {
            let category = ctype_rust::CTypeCategory::from_ctype(&member.c_decl.ty, analysis);
            match category {
                ctype_rust::CTypeCategory::FuncPointer => format!("Option<{}>", field_ty),
                _ => field_ty,
            }
        };

        writeln!(file, "pub {}: {},", name, field_ty).unwrap();
    }
    if type_info.lifetime_param {
        writeln!(file, "pub _marker: PhantomData<&'a ()>,",).unwrap();
    }
    writeln!(file, "}}\n").unwrap();

    if !type_info.trivial_debug {
        writeln!(file, "#[cfg(feature = \"debug\")]").unwrap();
        write_debug_impl(file, analysis, ty, &info, type_info);
    }

    if let Some(tag) = info.tag {
        writeln!(
            file,
            "unsafe impl<'a> TaggedStructure<'a> for {}<'a> {{
                const STRUCTURE_TYPE: StructureType = StructureType::{};
            }}\n",
            info.name, tag
        )
        .unwrap();
    }

    for extends in &ty.structextends {
        let rust_ty = ctype_to_rust_type_str(extends);
        if analysis.is_provisional_type(extends) {
            writeln!(file, "#[cfg(feature = \"provisional\")]").unwrap();
        }
        writeln!(
            file,
            "unsafe impl<'a> Extends<{}<'a>> for {}<'a> {{}}",
            rust_ty, info.name
        )
        .unwrap();
    }
    if !ty.structextends.is_empty() {
        writeln!(file).unwrap();
    }

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
                writeln!(file, "Self::STRUCTURE_TYPE").unwrap()
            } else {
                write!(
                    file,
                    "{}",
                    default_value(analysis, &member.member.c_decl.ty)
                )
                .unwrap();
            }
            writeln!(file, ",").unwrap();
        }
        if type_info.lifetime_param {
            writeln!(file, "_marker: PhantomData",).unwrap();
        }
        writeln!(file, "}} }} }}\n").unwrap();
    }

    writeln!(
        file,
        "impl{} {}{}{{",
        lifetime_spec, info.name, lifetime_spec
    )
    .unwrap();
    for setter in &info.setters {
        writeln!(
            file,
            "#[inline]
            pub fn {}(mut self,",
            setter.name
        )
        .unwrap();

        match &setter.kind {
            SetterKind::Value(param) => {
                writeln!(file, "{}: {},", param.name, param.ty).unwrap();
            }
            SetterKind::Array { params, .. } => {
                for param in params {
                    if param.optional {
                        writeln!(file, "{}: Option<{}>,", param.name, param.ty).unwrap();
                    } else {
                        writeln!(file, "{}: {},", param.name, param.ty).unwrap();
                    }
                }
            }
        }

        let is_cstr_array = matches!(&setter.kind, SetterKind::Value(p) if p.assignment == SetterAssignmentKind::CStrToArray);
        if is_cstr_array {
            writeln!(
                file,
                ") -> core::result::Result<Self, CStrTooLargeForStaticArray> {{"
            )
            .unwrap();
        } else {
            writeln!(file, ") -> Self {{").unwrap();
        }

        match &setter.kind {
            SetterKind::Value(param) => {
                let member = &info.members[param.member_index];
                match param.assignment {
                    SetterAssignmentKind::CStrToPtr => {
                        writeln!(file, "self.{} = {}.as_ptr();", member.name, param.name).unwrap();
                    }
                    SetterAssignmentKind::CStrToArray => {
                        writeln!(
                            file,
                            "write_c_str_slice_with_nul(&mut self.{}, {})?;",
                            member.name, param.name
                        )
                        .unwrap();
                    }
                    _ => {
                        if member.ty.starts_with("PFN_") {
                            writeln!(file, "self.{} = Some({});", member.name, param.name).unwrap();
                        } else if ctype_rust::is_bool32(&member.member.c_decl.ty) {
                            writeln!(file, "self.{} = {}.into();", member.name, param.name)
                                .unwrap();
                        } else {
                            writeln!(file, "self.{} = {};", member.name, param.name).unwrap();
                        }
                    }
                }
            }
            SetterKind::Array {
                len_member_index,
                params,
            } => {
                // Find the first required (non-optional) param to derive length from.
                let first_required = params.iter().find(|p| !p.optional);
                let len_source = first_required.unwrap_or(&params[0]);

                let len_member = &info.members[*len_member_index];

                if first_required.is_none() {
                    // All params are optional — derive length from first that is Some.
                    write!(file, "self.{} = None", len_member.name).unwrap();
                    for param in params {
                        write!(
                            file,
                            ".or_else(|| {}.as_deref().map(|s| s.len()))",
                            param.name
                        )
                        .unwrap();
                    }
                    writeln!(file, ".unwrap_or(0).try_into().unwrap();").unwrap();
                } else {
                    writeln!(
                        file,
                        "self.{} = {}.len().try_into().unwrap();",
                        len_member.name, len_source.name
                    )
                    .unwrap();
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
                        )
                        .unwrap();
                    } else {
                        writeln!(
                            file,
                            "assert_eq!({}.len(), self.{} as usize);",
                            param.name, len_member.name
                        )
                        .unwrap();
                    }
                }

                // Emit assignments.
                for param in params {
                    let member = &info.members[param.member_index];
                    if param.optional {
                        emit_optional_setter_assignment(
                            file,
                            &member.name,
                            &param.name,
                            param.assignment,
                        );
                    } else {
                        emit_setter_assignment(file, &member.name, &param.name, param.assignment);
                    }
                }
            }
        }

        if is_cstr_array {
            writeln!(file, "Ok(self) }}\n").unwrap();
        } else {
            writeln!(file, "self }}\n").unwrap();
        }
    }
    writeln!(file, "}}\n").unwrap();
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
    let category = ctype_rust::CTypeCategory::from_ctype(ty, analysis);

    match &category {
        // *const c_char with null-terminated length → CStr display
        ctype_rust::CTypeCategory::CharPointer { .. } => {
            if member.len.iter().any(|l| *l == "null-terminated") {
                DebugFieldKind::CStrPtr
            } else {
                DebugFieldKind::Normal
            }
        }
        // [c_char; N] → CStr display
        ctype_rust::CTypeCategory::Array { element, .. } => {
            if matches!(element, CType::Base(b) if b.name == "char") {
                DebugFieldKind::CStrArray
            } else {
                DebugFieldKind::Normal
            }
        }
        // Any pointer → normal Debug (prints address)
        ctype_rust::CTypeCategory::OpaquePointer { .. }
        | ctype_rust::CTypeCategory::TypedPointer { .. } => DebugFieldKind::Normal,
        // Function pointer (wrapped in Option) → show as pointer
        ctype_rust::CTypeCategory::FuncPointer => DebugFieldKind::FuncPointer,
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
) {
    let name = &info.name;
    let lifetime_spec_anon = if type_info.lifetime_param { "<'_>" } else { "" };

    writeln!(
        file,
        "impl fmt::Debug for {name}{lifetime_spec_anon} {{
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {{
            f.debug_struct(\"{name}\")"
    )
    .unwrap();

    for member in &ty.members {
        let field_name = normalize_name(member.c_decl.name);
        let kind = debug_field_kind(analysis, member);

        match kind {
            DebugFieldKind::Normal => {
                writeln!(file, ".field(\"{field_name}\", &self.{field_name})").unwrap();
            }
            DebugFieldKind::CStrPtr => {
                writeln!(
                    file,
                    ".field(\"{field_name}\", &unsafe {{ as_c_str(self.{field_name}) }})"
                )
                .unwrap();
            }
            DebugFieldKind::CStrArray => {
                writeln!(
                    file,
                    ".field(\"{field_name}\", &wrap_c_str_slice_until_nul(&self.{field_name}))"
                )
                .unwrap();
            }
            DebugFieldKind::FuncPointer => {
                writeln!(
                    file,
                    ".field(\"{field_name}\", &self.{field_name}.map(|f| f as *const ()))"
                )
                .unwrap();
            }
        }
    }

    writeln!(file, ".finish()\n}} }}\n").unwrap();
}

fn default_value(analysis: &Analysis, ty: &CType) -> std::borrow::Cow<'static, str> {
    let category = ctype_rust::CTypeCategory::from_ctype(ty, analysis);
    match category {
        ctype_rust::CTypeCategory::Array { element, .. } => {
            format!("[{}; _]", default_value(analysis, element)).into()
        }
        ctype_rust::CTypeCategory::OpaquePointer { is_const, .. }
        | ctype_rust::CTypeCategory::CharPointer { is_const }
        | ctype_rust::CTypeCategory::TypedPointer { is_const, .. } => {
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
    use ctype_rust::CTypeCategory;
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
) {
    match kind {
        SetterAssignmentKind::CopyFromSlice => {
            writeln!(
                file,
                "self.{}[..{}.len()].copy_from_slice({});",
                member_name, param_name, param_name
            )
            .unwrap();
        }
        SetterAssignmentKind::PtrFromSlice { is_const } => {
            if is_const {
                writeln!(file, "self.{} = {}.as_ptr() as _;", member_name, param_name).unwrap();
            } else {
                writeln!(
                    file,
                    "self.{} = {}.as_mut_ptr() as _;",
                    member_name, param_name
                )
                .unwrap();
            }
        }
        SetterAssignmentKind::PtrFromRefNested { is_const } => {
            if is_const {
                writeln!(file, "self.{} = {}.as_ptr() as _;", member_name, param_name).unwrap();
            } else {
                writeln!(
                    file,
                    "self.{} = {}.as_mut_ptr() as _;",
                    member_name, param_name
                )
                .unwrap();
            }
        }
        SetterAssignmentKind::PtrFromRef { is_const } => {
            if is_const {
                writeln!(file, "self.{} = {}.as_ptr();", member_name, param_name).unwrap();
            } else {
                writeln!(file, "self.{} = {}.as_mut_ptr();", member_name, param_name).unwrap();
            }
        }
        SetterAssignmentKind::CStrToPtr | SetterAssignmentKind::CStrToArray => {
            unreachable!("CStr assignments only used for Value setters")
        }
    }
}

/// Emit assignment for an optional array param (`Option<&[T]>` → pointer or null).
fn emit_optional_setter_assignment(
    file: &mut impl std::io::Write,
    member_name: &str,
    param_name: &str,
    kind: SetterAssignmentKind,
) {
    // For optional params, emit: if let Some(s) = param { ptr } else { null }
    let (map_expr, null_expr) = match kind {
        SetterAssignmentKind::CopyFromSlice => {
            // Optional fixed-array copy: copy if Some, leave unchanged if None
            writeln!(
                file,
                "if let Some(s) = {} {{ self.{}[..s.len()].copy_from_slice(s); }}",
                param_name, member_name
            )
            .unwrap();
            return;
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
                ("|s| s.as_ptr()", "ptr::null()")
            } else {
                ("|s| s.as_mut_ptr()", "ptr::null_mut()")
            }
        }
        SetterAssignmentKind::CStrToPtr | SetterAssignmentKind::CStrToArray => {
            unreachable!("CStr assignments only used for Value setters")
        }
    };
    writeln!(
        file,
        "self.{} = {}.map_or({}, {});",
        member_name, param_name, null_expr, map_expr
    )
    .unwrap();
}

pub fn convert_setter_param_type(
    analysis: &Analysis,
    ty: &CType,
    lengths: &[LengthKind<'_>],
    optional: &[bool],
    lifetime_param: Option<&str>,
) -> String {
    use ctype_rust::CTypeCategory;

    if let Some(len) = lengths.iter().next() {
        if !matches!(len, LengthKind::Literal(1)) {
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
                    let use_slice_of_refs = matches!(ty, CType::Ptr { pointee: p, .. } if matches!(p.as_ref(), CType::Ptr { pointee: inner, .. } if matches!(inner.as_ref(), CType::Base(b) if !analysis.is_opaque_type_name(b.name) && b.name == pointee_name)));
                    if use_slice_of_refs {
                        let inner = ctype_rust::type_name_with_lifetime(
                            analysis,
                            pointee_name,
                            lifetime_param,
                        );
                        if is_const {
                            format!("&'a [&'a {}]", inner)
                        } else {
                            format!("&'a mut [&'a mut {}]", inner)
                        }
                    } else {
                        let inner = ctype_rust::type_name_with_lifetime(
                            analysis,
                            pointee_name,
                            lifetime_param,
                        );
                        if is_const {
                            format!("&'a [*const {}]", inner)
                        } else {
                            format!("&'a mut [*mut {}]", inner)
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
                    let element_ty = convert_setter_param_type(
                        analysis,
                        pointee,
                        rest_lengths,
                        rest_optional,
                        lifetime_param,
                    );
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
                        format!("&'a [{}]", element_ty)
                    } else {
                        format!("&'a mut [{}]", element_ty)
                    }
                }
                CTypeCategory::Array { element, .. } => {
                    let element_ty = ctype_to_rust_type(analysis, element, lifetime_param);
                    format!("&[{}]", element_ty)
                }
                _ => panic!(
                    "expected pointer or array type because of length {:?}, got {:?}",
                    len, ty
                ),
            };
        }
    }

    let category = CTypeCategory::from_ctype(ty, analysis);
    match category {
        CTypeCategory::OpaquePointer {
            is_const,
            pointee_name,
        } => {
            let ty = ctype_rust::type_name_with_lifetime(analysis, pointee_name, lifetime_param);
            if is_const {
                format!("*const {}", ty)
            } else {
                format!("*mut {}", ty)
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
                    format!("*const {}", ty)
                } else {
                    format!("*mut {}", ty)
                }
            } else {
                if is_const {
                    format!("&'a {}", ty)
                } else {
                    format!("&'a mut {}", ty)
                }
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

