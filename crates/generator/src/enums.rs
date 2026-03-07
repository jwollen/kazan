use std::{
    borrow::Cow,
    collections::{BTreeMap, HashSet},
    io::Write,
    sync::OnceLock,
};

use heck::ToShoutySnakeCase;
use regex::Regex;

use crate::{analysis::Analysis, ctype_to_rust_type_str, module::Module, normalize_ty_name, xml};

pub struct ContribVariant {
    pub name: &'static str,
    pub value: i32,
    pub comment: Option<&'static str>,
}

pub struct ContribValue {
    pub name: &'static str,
    pub value: &'static str,
    pub comment: Option<&'static str>,
}

pub struct ContribAlias {
    pub name: &'static str,
    pub alias: &'static str,
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
                    let ext_number = variant.extnumber.or(ext_number).unwrap() as i32;
                    let value = 1_000_000_000i32 + (ext_number - 1) * 1000 + variant.offset as i32;
                    let value = if variant.negative { -value } else { value };
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
                                alias: alias.alias,
                            });
                    }
                }
                for value in &req.enum_values {
                    data.contributions(value.extends, &module_name)
                        .values
                        .push(ContribValue {
                            name: value.name,
                            value: value.value,
                            comment: value.comment,
                        });
                }
            }
        }

        data
    }
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

fn separate_trailing_number(name: &str) -> Cow<'_, str> {
    let trailing_number = TRAILING_NUMBER.get_or_init(|| regex::Regex::new(r"(\d+)$").unwrap());
    trailing_number.replace(name, "_$1")
}

fn is_useful_comment(comment: &str) -> bool {
    !matches!(comment, "Optional" | "Required")
        && !comment.starts_with("Not promoted to")
        && !comment.starts_with("Note that this defines what was previously")
        && !comment.starts_with("No need to add")
}

fn write_doc_comment(file: &mut impl Write, comment: Option<&str>) {
    if let Some(comment) = comment.filter(|c| is_useful_comment(c)) {
        writeln!(file, "/// {comment}").unwrap();
    }
}

fn format_doc_comment(comment: Option<&str>) -> String {
    match comment.filter(|c| is_useful_comment(c)) {
        Some(comment) => format!("/// {comment}\n"),
        None => String::new(),
    }
}

fn write_module_group(file: &mut impl Write, module_name: &str, entries: &[String]) {
    if entries.is_empty() {
        return;
    }
    writeln!(file, "// {}", module_name).unwrap();
    for entry in entries {
        writeln!(file, "{}", entry).unwrap();
    }
    writeln!(file).unwrap();
}

pub fn write_enum(file: &mut impl Write, analysis: &Analysis, ty: &xml::Enum) {
    let tags = &analysis.registry().tags;
    let req = analysis.req_enum_data();
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

    crate::write_doc_link(file, ty.name);
    writeln!(
        file,
        "#[repr(transparent)]
        #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
        pub struct {}(i32);",
        name
    )
    .unwrap();
    writeln!(file).unwrap();

    let mut debug_variants = Vec::new();
    let mut visited = HashSet::new();

    writeln!(file, "impl {} {{", name).unwrap();

    for bit in &ty.values {
        let vname = normalize_variant_name(bit.name, &value_prefix);
        write_doc_comment(file, bit.comment);
        writeln!(file, "pub const {}: Self = Self({});", vname, bit.value).unwrap();
        visited.insert(vname.clone());
        debug_variants.push(vname);
    }

    if let Some(modules) = req.get(ty.name) {
        for (module_name, contributions) in modules {
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
                    debug_variants.push(vname);
                }
            }
            for v in &contributions.values {
                let vname = normalize_variant_name(v.name, &value_prefix);
                if visited.insert(vname.clone()) {
                    entries.push(format!(
                        "{}pub const {}: Self = Self({});",
                        format_doc_comment(v.comment),
                        vname,
                        v.value
                    ));
                    debug_variants.push(vname);
                }
            }
            for a in &contributions.aliases {
                let aname = normalize_variant_name(a.name, &value_prefix);
                if visited.insert(aname.clone()) {
                    let target = normalize_variant_name(a.alias, &value_prefix);
                    entries.push(format!("pub const {}: Self = Self::{};", aname, target));
                }
            }

            write_module_group(file, module_name, &entries);
        }
    }

    writeln!(file, "}}\n").unwrap();

    writeln!(
        file,
        "impl fmt::Debug for {name} {{
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {{"
    )
    .unwrap();
    writeln!(file, "let name = match *self {{").unwrap();
    for vname in &debug_variants {
        writeln!(file, "Self::{vname} => Some(\"{vname}\"),").unwrap();
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
    )
    .unwrap();
}

/// Writes a bitmask type (bitflags + optional FlagBits struct) to `file`.
pub fn write_bitmask(
    file: &mut impl Write,
    analysis: &Analysis,
    ty: &xml::BitMaskType,
    bitmask: Option<&xml::BitMask>,
    req: &ReqEnumData,
) {
    let name = normalize_ty_name(ty.name);
    let base_type = ctype_to_rust_type_str(ty.ty);

    crate::write_doc_link(file, ty.name);
    writeln!(
        file,
        "#[repr(transparent)]
        #[derive(Copy, Clone, PartialEq, Eq, Hash)]
        pub struct {name}({base_type});
        vk_bitflags_wrapped!({name}, {base_type});
"
    )
    .unwrap();

    let Some(bitmask) = bitmask else {
        // No bits defined — Debug just prints the raw value
        writeln!(
            file,
            "impl fmt::Debug for {name} {{
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {{
                debug_flags(f, &[], self.0)
            }} }}"
        )
        .unwrap();
        writeln!(file).unwrap();
        return;
    };

    let bitmask_name = normalize_ty_name(bitmask.name);
    let vp = {
        let tags = &analysis.registry().tags;
        let prefix = strip_vendor_suffix(bitmask.name, tags)
            .replace("FlagBits", "")
            .to_shouty_snake_case();
        separate_trailing_number(&prefix).to_string()
    };

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

    // Pre-compute normalized base bits (shared between Flags and FlagBits)
    let base_bits: Vec<NormalizedBit> = {
        let mut bits: Vec<_> = bitmask
            .bits
            .iter()
            .map(|bit| NormalizedBit {
                name: normalize_bit_name(bit.name, Some(vp.as_str())),
                bitpos: bit.bitpos,
                comment: bit.comment,
            })
            .collect();
        bits.sort_by_key(|b| b.bitpos);
        bits
    };

    // Pre-compute normalized module data (shared between Flags and FlagBits)
    struct ModuleBitData {
        name: String,
        bits: Vec<NormalizedBit>,
        aliases: Vec<NormalizedAlias>,
        values: Vec<NormalizedValue>,
    }

    let modules = req.get(bitmask.name);
    let module_data: Vec<ModuleBitData> = modules
        .into_iter()
        .flat_map(|m| m.iter())
        .map(|(name, c)| ModuleBitData {
            name: name.clone(),
            bits: {
                let mut bits: Vec<_> = c
                    .bitpositions
                    .iter()
                    .map(|bp| NormalizedBit {
                        name: normalize_bit_name(bp.name, Some(vp.as_str())),
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
                    name: normalize_bit_name(a.name, Some(vp.as_str())),
                    target: normalize_bit_name(a.alias, Some(vp.as_str())),
                })
                .collect(),
            values: c
                .values
                .iter()
                .map(|v| NormalizedValue {
                    name: normalize_bit_name(v.name, Some(vp.as_str())),
                    value: v.value.to_string(),
                    comment: v.comment,
                })
                .collect(),
        })
        .collect();

    // All bit names for alias target validation in FlagBits
    let all_bit_names: HashSet<_> = base_bits
        .iter()
        .map(|b| b.name.clone())
        .chain(
            module_data
                .iter()
                .flat_map(|md| md.bits.iter().map(|b| b.name.clone())),
        )
        .collect();

    // Flags wrapper
    {
        let mut visited = HashSet::new();
        writeln!(file, "impl {name} {{").unwrap();

        for b in &base_bits {
            if visited.insert(b.name.clone()) {
                write_doc_comment(file, b.comment);
                writeln!(
                    file,
                    "pub const {}: Self = Self({}::{}.0);",
                    b.name, bitmask_name, b.name
                )
                .unwrap();
            }
        }
        for alias in &bitmask.aliases {
            let aname = normalize_bit_name(alias.name, Some(vp.as_str()));
            let target = normalize_bit_name(alias.alias, Some(vp.as_str()));
            if visited.insert(aname.clone()) {
                writeln!(file, "pub const {}: Self = Self::{};", aname, target).unwrap();
            }
        }
        for value in &bitmask.values {
            let vname = value
                .name
                .strip_prefix(&vp)
                .unwrap()
                .strip_prefix('_')
                .unwrap();
            let vname = strip_vendor_suffix(vname, &analysis.registry().tags);
            write_doc_comment(file, value.comment);
            writeln!(file, "pub const {}: Self = Self({});", vname, value.value).unwrap();
        }

        for md in &module_data {
            let mut entries = Vec::new();
            for b in &md.bits {
                if visited.insert(b.name.clone()) {
                    entries.push(format!(
                        "{}pub const {}: Self = Self({}::{}.0);",
                        format_doc_comment(b.comment),
                        b.name,
                        bitmask_name,
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
            write_module_group(file, &md.name, &entries);
        }

        writeln!(file, "}}\n").unwrap();
    }

    // Debug impl for Flags type
    {
        // Collect only single-bit constants (not composites, not zero)
        let mut debug_bits: Vec<(String, u8)> = Vec::new();
        let mut visited_bits = HashSet::new();

        for b in &base_bits {
            if visited_bits.insert(b.bitpos) {
                debug_bits.push((b.name.clone(), b.bitpos));
            }
        }
        for md in &module_data {
            for b in &md.bits {
                if visited_bits.insert(b.bitpos) {
                    debug_bits.push((b.name.clone(), b.bitpos));
                }
            }
        }

        writeln!(
            file,
            "impl fmt::Debug for {name} {{
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {{
                const KNOWN: &[({base_type}, &str)] = &["
        )
        .unwrap();
        for (bit_name, _) in &debug_bits {
            writeln!(file, "                ({name}::{bit_name}.0, \"{bit_name}\"),").unwrap();
        }
        writeln!(
            file,
            "            ];
            debug_flags(f, KNOWN, self.0)
            }} }}\n"
        )
        .unwrap();
    }

    // === FlagBits ===
    crate::write_doc_link(file, bitmask.name);
    writeln!(
        file,
        "#[repr(transparent)]
        #[derive(Copy, Clone, Default, Debug, PartialEq, Eq, Hash)]
        pub struct {}(u{});\n",
        bitmask_name,
        bitmask.bitwidth.unwrap_or(32),
    )
    .unwrap();

    {
        let mut visited = HashSet::new();
        writeln!(file, "impl {} {{", bitmask_name).unwrap();

        for b in &base_bits {
            if visited.insert(b.name.clone()) {
                write_doc_comment(file, b.comment);
                writeln!(
                    file,
                    "pub const {}: Self = Self(1 << {});",
                    b.name, b.bitpos
                )
                .unwrap();
            }
        }

        for md in &module_data {
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
                if !all_bit_names.contains(a.target.as_str()) {
                    continue;
                }
                if visited.insert(a.name.clone()) {
                    entries.push(format!("pub const {}: Self = Self::{};", a.name, a.target));
                }
            }
            write_module_group(file, &md.name, &entries);
        }

        writeln!(file, "}}\n").unwrap();
    }
}
