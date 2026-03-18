//! Emit layer: model → Rust source for enums and bitmasks.

use std::{collections::HashSet, io::Write};

use anyhow::Result;

use crate::{analysis::Analysis, model::enumeration::*, write_doc_link, xml};

pub fn generate_enums(
    file: &mut impl Write,
    analysis: &Analysis,
    enums: &[&xml::Enum],
) -> Result<()> {
    for ty in enums {
        let (def, debug_variants) = crate::model::enumeration::build_enum(analysis, ty);
        emit_enum(file, &def, &debug_variants)?;
    }
    Ok(())
}

pub fn generate_bitmasks(
    file: &mut impl Write,
    analysis: &Analysis,
    bitmask_types: &[&xml::BitMaskType],
) -> Result<()> {
    for ty in bitmask_types {
        let bitmask = ty.bitvalues.or(ty.requires).and_then(|b| {
            analysis
                .registry()
                .bitmasks
                .iter()
                .find(|bitmask| bitmask.name == b)
        });
        let def = crate::model::enumeration::build_bitmask(
            analysis,
            ty,
            bitmask,
            analysis.req_enum_data(),
        );
        emit_bitmask(file, &def)?;
    }
    Ok(())
}

// ── Enum emission ────────────────────────────────────────────────────────────

pub fn emit_enum(
    file: &mut impl Write,
    def: &EnumDef,
    debug_variants: &[EnumDebugVariant],
) -> Result<()> {
    let name = &def.name;

    write_doc_link(file, def.c_name)?;
    writeln!(
        file,
        "#[repr(transparent)]
        #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
        pub struct {name}(i32);"
    )?;
    writeln!(file)?;

    writeln!(file, "impl {name} {{")?;

    for v in &def.base_variants {
        write_doc_comment(file, v.comment)?;
        writeln!(file, "pub const {}: Self = Self({});", v.name, v.value)?;
    }
    writeln!(file)?;

    for group in &def.module_groups {
        let mut entries = Vec::new();
        for v in &group.variants {
            entries.push(format!(
                "{}pub const {}: Self = Self({});",
                format_doc_comment(v.comment),
                v.name,
                v.value
            ));
        }
        for a in &group.aliases {
            entries.push(format!("pub const {}: Self = Self::{};", a.name, a.target));
        }
        write_module_group(file, &group.module_name, &entries, group.provisional)?;
    }

    writeln!(file, "}}\n")?;

    // Debug impl
    writeln!(
        file,
        "impl fmt::Debug for {name} {{
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {{"
    )?;
    writeln!(file, "let name = match *self {{")?;
    for dv in debug_variants {
        if dv.provisional {
            writeln!(file, "#[cfg(feature = \"provisional\")]")?;
        }
        writeln!(file, "Self::{} => Some(\"{}\"),", dv.name, dv.name)?;
    }
    writeln!(
        file,
        "_ => None
        }};
        if let Some(name) = name {{
            f.write_str(name)
        }} else {{
            self.0.fmt(f)
        }} }} }}\n"
    )?;
    Ok(())
}

// ── Bitmask emission ─────────────────────────────────────────────────────────

pub fn emit_bitmask(file: &mut impl Write, def: &BitmaskDef) -> Result<()> {
    let flags_name = &def.flags_name;
    let base_type = &def.base_type;

    write_doc_link(file, def.flags_c_name)?;
    writeln!(
        file,
        "#[repr(transparent)]
        #[derive(Copy, Clone, PartialEq, Eq, Hash)]
        pub struct {flags_name}({base_type});"
    )?;

    let Some(ref fb) = def.flagbits else {
        writeln!(file, "vk_bitflags_wrapped!({flags_name}, {base_type});")?;
        writeln!(file)?;
        writeln!(
            file,
            "impl fmt::Debug for {flags_name} {{
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {{
                debug_flags(f, &[], self.0)
            }} }}"
        )?;
        writeln!(file)?;
        return Ok(());
    };

    let bitmask_name = &fb.name;
    writeln!(
        file,
        "vk_bitflags_wrapped!({flags_name}, {base_type}, {bitmask_name});\n"
    )?;

    emit_flags_constants(file, flags_name, fb)?;
    emit_flags_debug(file, flags_name, base_type, fb)?;
    emit_flagbits_constants(file, fb)?;
    emit_flagbits_debug(file, fb)?;

    Ok(())
}

fn emit_flags_constants(file: &mut impl Write, flags_name: &str, fb: &FlagBitsDef) -> Result<()> {
    let mut has_values = false;

    for v in &fb.flags_values {
        if !has_values {
            writeln!(file, "impl {flags_name} {{")?;
            has_values = true;
        }
        write_doc_comment(file, v.comment)?;
        writeln!(file, "pub const {}: Self = Self({});", v.name, v.value)?;
    }

    for mv in &fb.flags_module_values {
        let mut entries = Vec::new();
        for v in &mv.values {
            entries.push(format!(
                "{}pub const {}: Self = Self({});",
                format_doc_comment(v.comment),
                v.name,
                v.value
            ));
        }
        if !entries.is_empty() && !has_values {
            writeln!(file, "impl {flags_name} {{")?;
            has_values = true;
        }
        write_module_group(file, &mv.module_name, &entries, mv.provisional)?;
    }

    if has_values {
        writeln!(file, "}}\n")?;
    }
    Ok(())
}

fn emit_flags_debug(
    file: &mut impl Write,
    flags_name: &str,
    base_type: &str,
    fb: &FlagBitsDef,
) -> Result<()> {
    let bitmask_name = &fb.name;
    let mut debug_bits: Vec<(String, u8, bool)> = Vec::new();
    let mut visited_bits = HashSet::new();

    for b in &fb.base_bits {
        if visited_bits.insert(b.bitpos) {
            debug_bits.push((b.name.clone(), b.bitpos, false));
        }
    }
    for mg in &fb.module_groups {
        for b in &mg.bits {
            if visited_bits.insert(b.bitpos) {
                debug_bits.push((b.name.clone(), b.bitpos, mg.provisional));
            }
        }
    }

    writeln!(
        file,
        "impl fmt::Debug for {flags_name} {{
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {{
            const KNOWN: &[({base_type}, &str)] = &["
    )?;
    for (bit_name, _, provisional) in &debug_bits {
        if *provisional {
            writeln!(file, "#[cfg(feature = \"provisional\")]")?;
        }
        writeln!(
            file,
            "                ({bitmask_name}::{bit_name}.0, \"{bit_name}\"),"
        )?;
    }
    writeln!(
        file,
        "            ];
        debug_flags(f, KNOWN, self.0)
        }} }}\n"
    )?;
    Ok(())
}

fn emit_flagbits_constants(file: &mut impl Write, fb: &FlagBitsDef) -> Result<()> {
    let bitmask_name = &fb.name;

    write_doc_link(file, fb.c_name)?;
    writeln!(
        file,
        "#[repr(transparent)]
        #[derive(Copy, Clone, Default, PartialEq, Eq, Hash)]
        pub struct {}(u{});\n",
        bitmask_name, fb.bitwidth,
    )?;

    let all_bit_names: HashSet<&str> = fb.all_bit_names.iter().map(|s| s.as_str()).collect();
    let mut visited = HashSet::new();
    writeln!(file, "impl {bitmask_name} {{")?;

    for b in &fb.base_bits {
        if visited.insert(b.name.clone()) {
            write_doc_comment(file, b.comment)?;
            writeln!(
                file,
                "pub const {}: Self = Self(1 << {});",
                b.name, b.bitpos
            )?;
        }
    }

    for a in &fb.base_aliases {
        if all_bit_names.contains(a.target.as_str()) && visited.insert(a.name.clone()) {
            writeln!(file, "pub const {}: Self = Self::{};", a.name, a.target)?;
        }
    }

    for mg in &fb.module_groups {
        let mut entries = Vec::new();
        for b in &mg.bits {
            if visited.insert(b.name.clone()) {
                entries.push(format!(
                    "{}pub const {}: Self = Self(1 << {});",
                    format_doc_comment(b.comment),
                    b.name,
                    b.bitpos
                ));
            }
        }
        for a in &mg.aliases {
            if !all_bit_names.contains(a.target.as_str()) {
                continue;
            }
            if visited.insert(a.name.clone()) {
                entries.push(format!("pub const {}: Self = Self::{};", a.name, a.target));
            }
        }
        write_module_group(file, &mg.module_name, &entries, mg.provisional)?;
    }

    writeln!(file, "}}\n")?;
    Ok(())
}

fn emit_flagbits_debug(file: &mut impl Write, fb: &FlagBitsDef) -> Result<()> {
    let bitmask_name = &fb.name;
    let mut debug_variants: Vec<(String, bool)> = Vec::new();
    let mut debug_visited = HashSet::new();

    for b in &fb.base_bits {
        if debug_visited.insert(b.name.clone()) {
            debug_variants.push((b.name.clone(), false));
        }
    }
    for mg in &fb.module_groups {
        for b in &mg.bits {
            if debug_visited.insert(b.name.clone()) {
                debug_variants.push((b.name.clone(), mg.provisional));
            }
        }
    }

    writeln!(
        file,
        "impl fmt::Debug for {bitmask_name} {{
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {{"
    )?;
    writeln!(file, "let name = match *self {{")?;
    for (vname, provisional) in &debug_variants {
        if *provisional {
            writeln!(file, "#[cfg(feature = \"provisional\")]")?;
        }
        writeln!(file, "Self::{vname} => Some(\"{vname}\"),")?;
    }
    writeln!(
        file,
        "_ => None
        }};
        if let Some(name) = name {{
            f.write_str(name)
        }} else {{
            self.0.fmt(f)
        }} }} }}\n"
    )?;
    Ok(())
}

// ── Shared helpers ───────────────────────────────────────────────────────────

fn is_useful_comment(comment: &str) -> bool {
    !matches!(comment, "Optional" | "Required")
        && !comment.starts_with("Not promoted to")
        && !comment.starts_with("Note that this defines what was previously")
        && !comment.starts_with("No need to add")
}

fn write_doc_comment(file: &mut impl Write, comment: Option<&str>) -> Result<()> {
    if let Some(comment) = comment.filter(|c| is_useful_comment(c)) {
        writeln!(file, "/// {comment}")?;
    }
    Ok(())
}

fn format_doc_comment(comment: Option<&str>) -> String {
    match comment.filter(|c| is_useful_comment(c)) {
        Some(comment) => format!("/// {comment}\n"),
        None => String::new(),
    }
}

fn write_module_group(
    file: &mut impl Write,
    module_name: &str,
    entries: &[String],
    provisional: bool,
) -> Result<()> {
    if entries.is_empty() {
        return Ok(());
    }
    writeln!(file, "// {module_name}")?;
    for entry in entries {
        if provisional {
            writeln!(file, "#[cfg(feature = \"provisional\")]")?;
        }
        writeln!(file, "{entry}")?;
    }
    writeln!(file)?;
    Ok(())
}
