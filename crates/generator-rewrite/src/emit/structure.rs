use std::io::Write;

use anyhow::Result;

use crate::{analysis::Analysis, write_doc_link, xml};

use crate::model::structure as ms;
pub fn generate_structs(
    file: &mut impl Write,
    analysis: &Analysis,
    structs: &[&xml::Structure],
) -> Result<()> {
    for ty in structs {
        let model = crate::build::structure::build_struct(analysis, ty);
        emit_struct(file, &model)?;
    }
    Ok(())
}

// ---- New model-based emit functions ----

/// Emit a complete struct definition from the model.
pub fn emit_struct(file: &mut impl Write, s: &ms::StructDef) -> Result<()> {
    let lifetime_spec = if s.has_lifetime { "<'a>" } else { "" };
    let lifetime_spec_anon = if s.has_lifetime { "<'_>" } else { "" };
    let trivial_debug = s.debug_impl.is_none();

    emit_struct_definition(file, s, trivial_debug, lifetime_spec)?;

    if let Some(ref debug_impl) = s.debug_impl {
        writeln!(file, "#[cfg(feature = \"debug\")]")?;
        emit_debug_impl(file, s, debug_impl, lifetime_spec_anon)?;
    }

    emit_trait_impls(file, s)?;

    if let Some(ref default_impl) = s.default_impl {
        emit_default_impl(file, s, default_impl, lifetime_spec_anon)?;
    }

    emit_setters(file, s, lifetime_spec)?;

    Ok(())
}

fn emit_struct_definition(
    file: &mut impl Write,
    s: &ms::StructDef,
    trivial_debug: bool,
    lifetime_spec: &str,
) -> Result<()> {
    let derives_str = s.derives.join(", ");

    write_doc_link(file, &s.c_name)?;
    writeln!(file, "#[repr(C)]")?;
    if trivial_debug {
        writeln!(file, "#[cfg_attr(feature = \"debug\", derive(Debug))]")?;
    }
    writeln!(
        file,
        "#[derive({derives_str})]
        #[must_use]
        pub struct {}{} {{",
        s.name, lifetime_spec
    )?;

    for field in &s.fields {
        writeln!(file, "pub {}: {},", field.name, field.ty.to_tokens())?;
    }
    if s.has_lifetime {
        writeln!(file, "pub _marker: PhantomData<&'a ()>,")?;
    }
    writeln!(file, "}}\n")?;
    Ok(())
}

fn emit_debug_impl(
    file: &mut impl Write,
    s: &ms::StructDef,
    debug_impl: &ms::DebugImpl,
    lifetime_spec_anon: &str,
) -> Result<()> {
    let name = &s.name;
    writeln!(
        file,
        "impl fmt::Debug for {name}{lifetime_spec_anon} {{
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {{
            f.debug_struct(\"{name}\")"
    )?;

    for field in &debug_impl.fields {
        match field {
            ms::DebugField::Normal(field_name) => {
                writeln!(file, ".field(\"{field_name}\", &self.{field_name})")?;
            }
            ms::DebugField::CStrPtr(field_name) => {
                writeln!(
                    file,
                    ".field(\"{field_name}\", &unsafe {{ as_c_str(self.{field_name}) }})"
                )?;
            }
            ms::DebugField::FuncPointer(field_name) => {
                writeln!(
                    file,
                    ".field(\"{field_name}\", &self.{field_name}.map(|f| f as *const ()))"
                )?;
            }
            ms::DebugField::Bitfield {
                name: field_name,
                backing_field,
                offset,
                width,
            } => {
                let mask = (1u64 << width) - 1;
                if *offset == 0 {
                    writeln!(
                        file,
                        ".field(\"{field_name}\", &(self.{backing_field} & {mask:#x}))"
                    )?;
                } else {
                    writeln!(
                        file,
                        ".field(\"{field_name}\", &((self.{backing_field} >> {offset}) & {mask:#x}))"
                    )?;
                }
            }
        }
    }

    writeln!(file, ".finish()\n}} }}\n")?;
    Ok(())
}

fn emit_trait_impls(file: &mut impl Write, s: &ms::StructDef) -> Result<()> {
    for trait_impl in &s.trait_impls {
        match trait_impl {
            ms::TraitImpl::TaggedStructure { stype_suffix } => {
                writeln!(
                    file,
                    "unsafe impl<'a> TaggedStructure<'a> for {}<'a> {{
                        const STRUCTURE_TYPE: StructureType = StructureType::{};
                    }}\n",
                    s.name, stype_suffix
                )?;
            }
            ms::TraitImpl::Extends {
                target,
                provisional,
            } => {
                if *provisional {
                    writeln!(file, "#[cfg(feature = \"provisional\")]")?;
                }
                writeln!(
                    file,
                    "unsafe impl Extends<{}<'_>> for {}<'_> {{}}",
                    target, s.name
                )?;
            }
        }
    }
    if s.trait_impls
        .iter()
        .any(|t| matches!(t, ms::TraitImpl::Extends { .. }))
    {
        writeln!(file)?;
    }
    Ok(())
}

fn emit_default_impl(
    file: &mut impl Write,
    s: &ms::StructDef,
    default_impl: &ms::DefaultImpl,
    lifetime_spec_anon: &str,
) -> Result<()> {
    writeln!(
        file,
        "impl Default for {}{} {{
        fn default() -> Self {{
        Self {{",
        s.name, lifetime_spec_anon
    )?;
    for (i, field_default) in default_impl.field_defaults.iter().enumerate() {
        write!(file, "{}: ", s.fields[i].name)?;
        if matches!(field_default, ms::FieldDefault::StructureType) {
            writeln!(file, "Self::STRUCTURE_TYPE")?;
        } else {
            write!(file, "{}", emit_field_default(field_default))?;
        }
        writeln!(file, ",")?;
    }
    if default_impl.has_phantom {
        writeln!(file, "_marker: PhantomData")?;
    }
    writeln!(file, "}} }} }}\n")?;
    Ok(())
}

fn emit_field_default(default: &ms::FieldDefault) -> std::borrow::Cow<'static, str> {
    match default {
        ms::FieldDefault::StructureType => "Self::STRUCTURE_TYPE".into(),
        ms::FieldDefault::Null { mutable } => {
            if *mutable {
                "ptr::null_mut()".into()
            } else {
                "ptr::null()".into()
            }
        }
        ms::FieldDefault::ArrayFill(inner) => format!("[{}; _]", emit_field_default(inner)).into(),
        ms::FieldDefault::Zero => "Default::default()".into(),
    }
}

fn emit_setters(file: &mut impl Write, s: &ms::StructDef, lifetime_spec: &str) -> Result<()> {
    writeln!(file, "impl{} {}{}{{", lifetime_spec, s.name, lifetime_spec)?;

    for setter in &s.setters {
        writeln!(
            file,
            "#[inline]
            pub fn {}(mut self,",
            setter.name
        )?;

        match &setter.kind {
            ms::SetterKind::Value { param, .. } => {
                writeln!(file, "{}: {},", param.name, param.ty.to_tokens())?;
            }
            ms::SetterKind::Array { params, .. } => {
                for param in params {
                    if param.optional {
                        writeln!(file, "{}: Option<{}>,", param.name, param.ty.to_tokens())?;
                    } else {
                        writeln!(file, "{}: {},", param.name, param.ty.to_tokens())?;
                    }
                }
            }
            ms::SetterKind::Bitfield { param_ty, .. } => {
                writeln!(file, "{}: {},", setter.name, param_ty.to_tokens())?;
            }
            ms::SetterKind::CStrToPtr { param, .. } => {
                writeln!(file, "{}: {},", param.name, param.ty.to_tokens())?;
            }
            ms::SetterKind::CStrToArray { param, .. } => {
                writeln!(file, "{}: {},", param.name, param.ty.to_tokens())?;
            }
        }

        let is_cstr_array = matches!(&setter.kind, ms::SetterKind::CStrToArray { .. });
        if is_cstr_array {
            writeln!(
                file,
                ") -> core::result::Result<Self, CStrTooLargeForStaticArray> {{"
            )?;
        } else {
            writeln!(file, ") -> Self {{")?;
        }

        match &setter.kind {
            ms::SetterKind::Value { param, assignment } => {
                emit_value_setter_body(file, s, param, *assignment)?;
            }
            ms::SetterKind::Array {
                len_field, params, ..
            } => {
                emit_array_setter_body(file, s, *len_field, params)?;
            }
            ms::SetterKind::Bitfield {
                backing_field,
                offset,
                width,
                extract,
                ..
            } => {
                emit_bitfield_setter_body(
                    file,
                    &s.fields[*backing_field].name,
                    &s.fields[*backing_field].ty.to_tokens(),
                    &setter.name,
                    *offset,
                    *width,
                    *extract,
                )?;
            }
            ms::SetterKind::CStrToPtr { param, field } => {
                writeln!(
                    file,
                    "self.{} = {}.as_ptr();",
                    s.fields[*field].name, param.name
                )?;
            }
            ms::SetterKind::CStrToArray { param, field } => {
                writeln!(
                    file,
                    "self.{}.write_c_str({})?;",
                    s.fields[*field].name, param.name
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

fn emit_value_setter_body(
    file: &mut impl Write,
    s: &ms::StructDef,
    param: &ms::SetterParam,
    assignment: ms::ValueAssignment,
) -> Result<()> {
    let member_name = &s.fields[param.field_index].name;
    match assignment {
        ms::ValueAssignment::WrapInSome => {
            writeln!(file, "self.{} = Some({});", member_name, param.name)?;
        }
        ms::ValueAssignment::BoolInto => {
            writeln!(file, "self.{} = {}.into();", member_name, param.name)?;
        }
        ms::ValueAssignment::Direct => {
            writeln!(file, "self.{} = {};", member_name, param.name)?;
        }
    }
    Ok(())
}

fn emit_bitfield_setter_body(
    file: &mut impl Write,
    backing_field_name: &str,
    backing_ty: &str,
    param_name: &str,
    offset: u8,
    width: u8,
    extract: ms::BitfieldExtract,
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
        let raw_val = match extract {
            ms::BitfieldExtract::AsRaw => format!("{param_name}.as_raw()"),
            ms::BitfieldExtract::Bool => format!("{param_name} as u32"),
            ms::BitfieldExtract::Direct => param_name.to_string(),
        };
        writeln!(
            file,
            "set_bitfield::<{offset}, {width}>(&mut self.{backing_field_name}, {raw_val});",
        )?;
    }
    Ok(())
}

fn emit_array_setter_body(
    file: &mut impl Write,
    s: &ms::StructDef,
    len_field: usize,
    params: &[ms::SetterParam],
) -> Result<()> {
    let first_required = params.iter().find(|p| !p.optional);
    let len_source = first_required.unwrap_or(&params[0]);
    let len_member_name = &s.fields[len_field].name;

    if first_required.is_none() {
        write!(file, "self.{len_member_name} = None")?;
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
            len_member_name, len_source.name
        )?;
    }

    for param in params {
        if std::ptr::eq(param, len_source) {
            continue;
        }
        if param.optional {
            writeln!(
                file,
                "if let Some(s) = &{name} {{ assert_eq!(s.len(), self.{len} as usize); }}",
                name = param.name,
                len = len_member_name,
            )?;
        } else {
            writeln!(
                file,
                "assert_eq!({}.len(), self.{} as usize);",
                param.name, len_member_name
            )?;
        }
    }

    for param in params {
        let member_name = &s.fields[param.field_index].name;
        if param.optional {
            emit_optional_array_assignment(file, member_name, &param.name, param.array_assign)?;
        } else {
            emit_array_assignment(file, member_name, &param.name, param.array_assign)?;
        }
    }
    Ok(())
}

fn emit_array_assignment(
    file: &mut impl Write,
    member_name: &str,
    param_name: &str,
    kind: Option<ms::ArrayAssignment>,
) -> Result<()> {
    match kind {
        Some(ms::ArrayAssignment::CopyFromSlice) | None => {
            writeln!(
                file,
                "self.{member_name}[..{param_name}.len()].copy_from_slice({param_name});"
            )?;
        }
        Some(ms::ArrayAssignment::PtrFromSlice { is_const }) => {
            if is_const {
                writeln!(file, "self.{member_name} = {param_name}.as_ptr() as _;")?;
            } else {
                writeln!(file, "self.{member_name} = {param_name}.as_mut_ptr() as _;")?;
            }
        }
        Some(ms::ArrayAssignment::PtrFromRef { is_const }) => {
            if is_const {
                writeln!(file, "self.{member_name} = {param_name}.as_ptr() as _;")?;
            } else {
                writeln!(file, "self.{member_name} = {param_name}.as_mut_ptr() as _;")?;
            }
        }
    }
    Ok(())
}

fn emit_optional_array_assignment(
    file: &mut impl Write,
    member_name: &str,
    param_name: &str,
    kind: Option<ms::ArrayAssignment>,
) -> Result<()> {
    match kind {
        Some(ms::ArrayAssignment::CopyFromSlice) | None => {
            writeln!(
                file,
                "if let Some(s) = {param_name} {{ self.{member_name}[..s.len()].copy_from_slice(s); }}"
            )?;
        }
        Some(ms::ArrayAssignment::PtrFromSlice { is_const })
        | Some(ms::ArrayAssignment::PtrFromRef { is_const }) => {
            let (map_expr, null_expr) = if is_const {
                ("|s| s.as_ptr() as _", "ptr::null()")
            } else {
                ("|s| s.as_mut_ptr() as _", "ptr::null_mut()")
            };
            writeln!(
                file,
                "self.{member_name} = {param_name}.map_or({null_expr}, {map_expr});"
            )?;
        }
    }
    Ok(())
}
