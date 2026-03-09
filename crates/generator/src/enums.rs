use std::{
    borrow::Cow,
    collections::{BTreeMap, HashSet},
    io::Write,
    sync::OnceLock,
};

use anyhow::Result;
use heck::ToShoutySnakeCase;
use regex::Regex;

use crate::{
    analysis::Analysis, ctype_rust::base_ctype_to_rust_str, module::Module, normalize_ty_name, xml,
};

pub fn generate_enum_types(
    file: &mut impl Write,
    analysis: &Analysis,
    enums: &[&xml::Enum],
) -> Result<()> {
    for ty in enums {
        write_enum(file, analysis, ty)?;
    }
    Ok(())
}

pub fn generate_bitmask_types(
    file: &mut impl Write,
    analysis: &Analysis,
    bitmask_types: &[&xml::BitMaskType],
) -> Result<()> {
    for ty in bitmask_types {
        // A bitmask type references its FlagBits enum via either `bitvalues` (modern) or `requires` (legacy).
        let bitmask = ty.bitvalues.or(ty.requires).and_then(|b| {
            analysis
                .registry()
                .bitmasks
                .iter()
                .find(|bitmask| bitmask.name == b)
        });

        write_bitmask(file, analysis, ty, bitmask, analysis.req_enum_data())?;
    }
    Ok(())
}

pub struct ContribVariant {
    pub name: &'static str,
    pub value: i32,
    pub comment: Option<&'static str>,
}

pub struct ContribValue {
    pub name: &'static str,
    /// Raw value string from vk.xml (e.g. `"0"`, `"1000"`).
    pub literal_value: &'static str,
    pub comment: Option<&'static str>,
}

pub struct ContribAlias {
    pub name: &'static str,
    /// The existing enum value this alias points to.
    pub target: &'static str,
}

pub struct ContribBitpos {
    pub name: &'static str,
    pub bitpos: u8,
    pub comment: Option<&'static str>,
}

/// Contributions from a single version/extension to an enum or bitmask type.
#[derive(Default)]
pub struct ModuleEnumContributions {
    pub variants: Vec<ContribVariant>,
    pub values: Vec<ContribValue>,
    pub aliases: Vec<ContribAlias>,
    pub bitpositions: Vec<ContribBitpos>,
}

/// Enum extension data grouped by enum type name, then by module name.
#[derive(Default)]
pub struct ReqEnumData {
    map: BTreeMap<&'static str, BTreeMap<String, ModuleEnumContributions>>,
}

impl ReqEnumData {
    fn contributions(
        &mut self,
        extends: &'static str,
        module: &str,
    ) -> &mut ModuleEnumContributions {
        self.map
            .entry(extends)
            .or_default()
            .entry(module.to_string())
            .or_default()
    }

    pub fn get(&self, enum_name: &str) -> Option<&BTreeMap<String, ModuleEnumContributions>> {
        self.map.get(enum_name)
    }

    pub fn from_registry(registry: &xml::Registry) -> Self {
        let mut data = Self::default();

        for module in Module::from_registry(registry) {
            let ext_number = module.ext_number();
            let module_name = module.display_name();
            for req in module.requires() {
                for variant in &req.enum_variants {
                    let value = get_enum_value(ext_number, variant);
                    data.contributions(variant.extends, &module_name)
                        .variants
                        .push(ContribVariant {
                            name: variant.name,
                            value,
                            comment: variant.comment,
                        });
                }
                for bitpos in &req.bitpositions {
                    data.contributions(bitpos.extends, &module_name)
                        .bitpositions
                        .push(ContribBitpos {
                            name: bitpos.name,
                            bitpos: bitpos.bitpos,
                            comment: bitpos.comment,
                        });
                }
                for alias in &req.enum_aliases {
                    if let Some(extends) = alias.extends {
                        data.contributions(extends, &module_name)
                            .aliases
                            .push(ContribAlias {
                                name: alias.name,
                                target: alias.alias,
                            });
                    }
                }
                for value in &req.enum_values {
                    data.contributions(value.extends, &module_name)
                        .values
                        .push(ContribValue {
                            name: value.name,
                            literal_value: value.value,
                            comment: value.comment,
                        });
                }
            }
        }

        data
    }
}

fn get_enum_value(ext_number: Option<u32>, variant: &xml::RequireEnumVariant) -> i32 {
    // Vulkan enum value formula: base (1e9) + (ext_number - 1) * 1000 + offset
    let ext_number = variant.extnumber.or(ext_number).unwrap() as i32;
    let value = 1_000_000_000i32 + (ext_number - 1) * 1000 + variant.offset as i32;
    let value = if variant.negative { -value } else { value };
    value
}

fn strip_vendor_suffix<'a>(name: &'a str, tags: &[&str]) -> &'a str {
    let vendor = tags
        .iter()
        .filter(|&&tag| name.ends_with(tag))
        .max_by_key(|tag| tag.len())
        .copied();
    if let Some(vendor) = vendor {
        let name = name.strip_suffix(vendor).unwrap();
        name.strip_suffix('_').unwrap_or(name)
    } else {
        name
    }
}

/// Strip the enum type prefix from a Vulkan enum value name (e.g. `VK_STRUCTURE_TYPE_FOO` → `FOO`).
fn normalize_variant_name(name: &str, value_prefix: &str) -> String {
    let name = name
        .strip_prefix(value_prefix)
        .unwrap()
        .strip_prefix('_')
        .unwrap()
        .to_uppercase();
    if name.starts_with(|c: char| c.is_ascii_digit()) {
        format!("_{}", name)
    } else {
        name
    }
}

/// Strip the bitmask type prefix and `_BIT` suffix from a Vulkan flag bit name.
fn normalize_bit_name(name: &str, value_prefix: Option<&str>) -> String {
    let name = match value_prefix {
        Some(prefix) => name.strip_prefix(prefix).unwrap_or(name),
        None => name,
    };
    let name = name
        .strip_prefix("VK")
        .unwrap_or(name)
        .strip_prefix('_')
        .unwrap();
    let name = name.replace("_BIT", "");
    if name.starts_with(|c: char| c.is_ascii_digit()) {
        format!("_{}", name)
    } else {
        name
    }
}

static TRAILING_NUMBER: OnceLock<Regex> = OnceLock::new();

/// Insert underscore before trailing digits so case conversion keeps them separate
/// (e.g. `DEVICE_GROUP2` → `DEVICE_GROUP_2`).
fn separate_trailing_number(name: &str) -> Cow<'_, str> {
    let trailing_number = TRAILING_NUMBER.get_or_init(|| regex::Regex::new(r"(\d+)$").unwrap());
    trailing_number.replace(name, "_$1")
}

/// Filter out vk.xml comments that are meta-notes rather than meaningful documentation.
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
    writeln!(file, "// {}", module_name)?;
    for entry in entries {
        if provisional {
            writeln!(file, "#[cfg(feature = \"provisional\")]")?;
        }
        writeln!(file, "{}", entry)?;
    }
    writeln!(file)?;
    Ok(())
}

pub fn write_enum(file: &mut impl Write, analysis: &Analysis, ty: &xml::Enum) -> Result<()> {
    let tags = &analysis.registry().tags;
    let req = analysis.req_enum_data();
    // VkResult values use "VK_" prefix (e.g. VK_SUCCESS) instead of the type-derived "VK_RESULT_".
    let value_prefix = if ty.name == "VkResult" {
        "VK".to_string()
    } else {
        let prefix = strip_vendor_suffix(ty.name, tags).to_shouty_snake_case();
        separate_trailing_number(&prefix).to_string()
    };

    let name = if ty.name == "VkResult" {
        "Result"
    } else {
        normalize_ty_name(ty.name)
    };

    crate::write_doc_link(file, ty.name)?;
    writeln!(
        file,
        "#[repr(transparent)]
        #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
        pub struct {}(i32);",
        name
    )?;
    writeln!(file)?;

    let mut debug_variants: Vec<(String, bool)> = Vec::new(); // (variant_name, is_provisional)
    let mut visited = HashSet::new();

    writeln!(file, "impl {} {{", name)?;

    for bit in &ty.values {
        let vname = normalize_variant_name(bit.name, &value_prefix);
        write_doc_comment(file, bit.comment)?;
        writeln!(file, "pub const {}: Self = Self({});", vname, bit.value)?;
        visited.insert(vname.clone());
        debug_variants.push((vname, false));
    }
    writeln!(file)?;

    if let Some(modules) = req.get(ty.name) {
        for (module_name, contributions) in modules {
            let provisional = analysis.is_provisional_extension(module_name);
            let mut entries = Vec::new();

            for v in &contributions.variants {
                let vname = normalize_variant_name(v.name, &value_prefix);
                if visited.insert(vname.clone()) {
                    entries.push(format!(
                        "{}pub const {}: Self = Self({});",
                        format_doc_comment(v.comment),
                        vname,
                        v.value
                    ));
                    debug_variants.push((vname, provisional));
                }
            }
            for v in &contributions.values {
                let vname = normalize_variant_name(v.name, &value_prefix);
                if visited.insert(vname.clone()) {
                    entries.push(format!(
                        "{}pub const {}: Self = Self({});",
                        format_doc_comment(v.comment),
                        vname,
                        v.literal_value
                    ));
                    debug_variants.push((vname, provisional));
                }
            }
            for a in &contributions.aliases {
                let aname = normalize_variant_name(a.name, &value_prefix);
                if visited.insert(aname.clone()) {
                    let target = normalize_variant_name(a.target, &value_prefix);
                    entries.push(format!("pub const {}: Self = Self::{};", aname, target));
                }
            }

            write_module_group(file, module_name, &entries, provisional)?;
        }
    }

    writeln!(file, "}}\n")?;

    writeln!(
        file,
        "impl fmt::Debug for {name} {{
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

struct NormalizedBit {
    name: String,
    bitpos: u8,
    comment: Option<&'static str>,
}

struct NormalizedAlias {
    name: String,
    target: String,
}

struct NormalizedValue {
    name: String,
    value: String,
    comment: Option<&'static str>,
}

struct ModuleBitData {
    name: String,
    provisional: bool,
    bits: Vec<NormalizedBit>,
    aliases: Vec<NormalizedAlias>,
    values: Vec<NormalizedValue>,
}

struct NormalizedBitmaskData {
    bitmask_name: String,
    value_prefix: String,
    base_bits: Vec<NormalizedBit>,
    module_data: Vec<ModuleBitData>,
    /// All known bit names (for alias target validation in FlagBits).
    all_bit_names: HashSet<String>,
}

fn normalize_bitmask_data(
    analysis: &Analysis,
    bitmask: &xml::BitMask,
    req: &ReqEnumData,
) -> NormalizedBitmaskData {
    let bitmask_name = normalize_ty_name(bitmask.name).to_string();
    let tags = &analysis.registry().tags;
    // Derive constant name prefix: remove "FlagBits" from type name,
    // e.g. "VkAccessFlagBits2" → "ACCESS_2_".
    let value_prefix = {
        let prefix = strip_vendor_suffix(bitmask.name, tags)
            .replace("FlagBits", "")
            .to_shouty_snake_case();
        separate_trailing_number(&prefix).to_string()
    };

    let mut base_bits: Vec<_> = bitmask
        .bits
        .iter()
        .map(|bit| NormalizedBit {
            name: normalize_bit_name(bit.name, Some(value_prefix.as_str())),
            bitpos: bit.bitpos,
            comment: bit.comment,
        })
        .collect();
    base_bits.sort_by_key(|b| b.bitpos);

    let modules = req.get(bitmask.name);
    let module_data: Vec<ModuleBitData> = modules
        .into_iter()
        .flat_map(|m| m.iter())
        .map(|(name, c)| ModuleBitData {
            name: name.clone(),
            provisional: analysis.is_provisional_extension(name),
            bits: {
                let mut bits: Vec<_> = c
                    .bitpositions
                    .iter()
                    .map(|bp| NormalizedBit {
                        name: normalize_bit_name(bp.name, Some(value_prefix.as_str())),
                        bitpos: bp.bitpos,
                        comment: bp.comment,
                    })
                    .collect();
                bits.sort_by_key(|b| b.bitpos);
                bits
            },
            aliases: c
                .aliases
                .iter()
                .map(|a| NormalizedAlias {
                    name: normalize_bit_name(a.name, Some(value_prefix.as_str())),
                    target: normalize_bit_name(a.target, Some(value_prefix.as_str())),
                })
                .collect(),
            values: c
                .values
                .iter()
                .map(|v| NormalizedValue {
                    name: normalize_bit_name(v.name, Some(value_prefix.as_str())),
                    value: v.literal_value.to_string(),
                    comment: v.comment,
                })
                .collect(),
        })
        .collect();

    let all_bit_names: HashSet<_> = base_bits
        .iter()
        .map(|b| b.name.clone())
        .chain(
            module_data
                .iter()
                .flat_map(|md| md.bits.iter().map(|b| b.name.clone())),
        )
        .collect();

    NormalizedBitmaskData {
        bitmask_name,
        value_prefix,
        base_bits,
        module_data,
        all_bit_names,
    }
}

fn write_flags_constants(
    file: &mut impl Write,
    analysis: &Analysis,
    flags_name: &str,
    bitmask: &xml::BitMask,
    data: &NormalizedBitmaskData,
) -> Result<()> {
    let mut visited = HashSet::new();
    writeln!(file, "impl {flags_name} {{")?;

    for b in &data.base_bits {
        if visited.insert(b.name.clone()) {
            write_doc_comment(file, b.comment)?;
            writeln!(
                file,
                "pub const {}: Self = Self({}::{}.0);",
                b.name, data.bitmask_name, b.name
            )?;
        }
    }
    for alias in &bitmask.aliases {
        let aname = normalize_bit_name(alias.name, Some(data.value_prefix.as_str()));
        let target = normalize_bit_name(alias.alias, Some(data.value_prefix.as_str()));
        if visited.insert(aname.clone()) {
            writeln!(file, "pub const {}: Self = Self::{};", aname, target)?;
        }
    }
    for value in &bitmask.values {
        let vname = value
            .name
            .strip_prefix(&data.value_prefix)
            .unwrap()
            .strip_prefix('_')
            .unwrap();
        let vname = strip_vendor_suffix(vname, &analysis.registry().tags);
        write_doc_comment(file, value.comment)?;
        writeln!(file, "pub const {}: Self = Self({});", vname, value.value)?;
    }
    writeln!(file)?;

    for md in &data.module_data {
        let mut entries = Vec::new();
        for b in &md.bits {
            if visited.insert(b.name.clone()) {
                entries.push(format!(
                    "{}pub const {}: Self = Self({}::{}.0);",
                    format_doc_comment(b.comment),
                    b.name,
                    data.bitmask_name,
                    b.name
                ));
            }
        }
        for a in &md.aliases {
            if visited.insert(a.name.clone()) {
                entries.push(format!("pub const {}: Self = Self::{};", a.name, a.target));
            }
        }
        for v in &md.values {
            entries.push(format!(
                "{}pub const {}: Self = Self({});",
                format_doc_comment(v.comment),
                v.name,
                v.value
            ));
        }
        write_module_group(file, &md.name, &entries, md.provisional)?;
    }

    writeln!(file, "}}\n")?;
    Ok(())
}

fn write_flags_debug(
    file: &mut impl Write,
    flags_name: &str,
    base_type: &str,
    data: &NormalizedBitmaskData,
) -> Result<()> {
    let mut debug_bits: Vec<(String, u8, bool)> = Vec::new();
    let mut visited_bits = HashSet::new();

    for b in &data.base_bits {
        if visited_bits.insert(b.bitpos) {
            debug_bits.push((b.name.clone(), b.bitpos, false));
        }
    }
    for md in &data.module_data {
        for b in &md.bits {
            if visited_bits.insert(b.bitpos) {
                debug_bits.push((b.name.clone(), b.bitpos, md.provisional));
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
            "                ({flags_name}::{bit_name}.0, \"{bit_name}\"),"
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

fn write_flagbits_constants(
    file: &mut impl Write,
    bitmask: &xml::BitMask,
    data: &NormalizedBitmaskData,
) -> Result<()> {
    let bitmask_name = &data.bitmask_name;

    crate::write_doc_link(file, bitmask.name)?;
    writeln!(
        file,
        "#[repr(transparent)]
        #[derive(Copy, Clone, Default, PartialEq, Eq, Hash)]
        pub struct {}(u{});\n",
        bitmask_name,
        bitmask.bitwidth.unwrap_or(32),
    )?;

    let mut visited = HashSet::new();
    writeln!(file, "impl {} {{", bitmask_name)?;

    for b in &data.base_bits {
        if visited.insert(b.name.clone()) {
            write_doc_comment(file, b.comment)?;
            writeln!(
                file,
                "pub const {}: Self = Self(1 << {});",
                b.name, b.bitpos
            )?;
        }
    }

    for md in &data.module_data {
        let mut entries = Vec::new();
        for b in &md.bits {
            if visited.insert(b.name.clone()) {
                entries.push(format!(
                    "{}pub const {}: Self = Self(1 << {});",
                    format_doc_comment(b.comment),
                    b.name,
                    b.bitpos
                ));
            }
        }
        for a in &md.aliases {
            // Skip aliases whose target bit was not emitted (defined in a different/disabled extension).
            if !data.all_bit_names.contains(a.target.as_str()) {
                continue;
            }
            if visited.insert(a.name.clone()) {
                entries.push(format!("pub const {}: Self = Self::{};", a.name, a.target));
            }
        }
        write_module_group(file, &md.name, &entries, md.provisional)?;
    }

    writeln!(file, "}}\n")?;
    Ok(())
}

fn write_flagbits_debug(file: &mut impl Write, data: &NormalizedBitmaskData) -> Result<()> {
    let bitmask_name = &data.bitmask_name;
    let mut debug_variants: Vec<(String, bool)> = Vec::new(); // (variant_name, is_provisional)
    let mut debug_visited = HashSet::new();

    for b in &data.base_bits {
        if debug_visited.insert(b.name.clone()) {
            debug_variants.push((b.name.clone(), false));
        }
    }
    for md in &data.module_data {
        for b in &md.bits {
            if debug_visited.insert(b.name.clone()) {
                debug_variants.push((b.name.clone(), md.provisional));
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

/// Writes a bitmask type (bitflags + optional FlagBits struct) to `file`.
pub fn write_bitmask(
    file: &mut impl Write,
    analysis: &Analysis,
    ty: &xml::BitMaskType,
    bitmask: Option<&xml::BitMask>,
    req: &ReqEnumData,
) -> Result<()> {
    let name = normalize_ty_name(ty.name);
    let base_type = base_ctype_to_rust_str(ty.ty);

    crate::write_doc_link(file, ty.name)?;
    writeln!(
        file,
        "#[repr(transparent)]
        #[derive(Copy, Clone, PartialEq, Eq, Hash)]
        pub struct {name}({base_type});
        vk_bitflags_wrapped!({name}, {base_type});
"
    )?;

    let Some(bitmask) = bitmask else {
        writeln!(
            file,
            "impl fmt::Debug for {name} {{
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {{
                debug_flags(f, &[], self.0)
            }} }}"
        )?;
        writeln!(file)?;
        return Ok(());
    };

    let data = normalize_bitmask_data(analysis, bitmask, req);

    write_flags_constants(file, analysis, name, bitmask, &data)?;
    write_flags_debug(file, name, base_type, &data)?;
    write_flagbits_constants(file, bitmask, &data)?;
    write_flagbits_debug(file, &data)?;
    Ok(())
}
