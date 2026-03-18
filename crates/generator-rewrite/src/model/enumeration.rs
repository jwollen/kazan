//! Model and builders for enum and bitmask types.

use std::{borrow::Cow, collections::HashSet, sync::OnceLock};

use heck::ToShoutySnakeCase;
use regex::Regex;

use crate::{
    analysis::{Analysis, ReqEnumData},
    ctype::base_ctype_to_rust_str,
    normalize_ty_name, xml,
};

/// Model for a generated Vulkan enum type.
#[derive(Debug, Clone)]
pub struct EnumDef {
    pub name: String,
    pub c_name: &'static str,
    /// Base variants defined in the enum itself.
    pub base_variants: Vec<EnumVariant>,
    /// Variants and aliases contributed by extensions/versions.
    pub module_groups: Vec<EnumModuleGroup>,
}

/// A single enum variant (constant).
#[derive(Debug, Clone)]
pub struct EnumVariant {
    pub name: String,
    pub value: String,
    pub comment: Option<&'static str>,
}

/// An alias for an enum variant.
#[derive(Debug, Clone)]
pub struct EnumAlias {
    pub name: String,
    pub target: String,
}

/// Variants and aliases contributed by a single module (version or extension).
#[derive(Debug, Clone)]
pub struct EnumModuleGroup {
    pub module_name: String,
    pub provisional: bool,
    pub variants: Vec<EnumVariant>,
    pub aliases: Vec<EnumAlias>,
}

/// Debug variant: tracks which variants need debug output and whether they're provisional.
#[derive(Debug, Clone)]
pub struct EnumDebugVariant {
    pub name: String,
    pub provisional: bool,
}

// ── Bitmask model ────────────────────────────────────────────────────────────

/// Model for a bitmask type (VkFooFlags + optional VkFooFlagBits).
#[derive(Debug, Clone)]
pub struct BitmaskDef {
    pub flags_name: &'static str,
    pub flags_c_name: &'static str,
    pub base_type: &'static str,
    /// None if no FlagBits enum is associated.
    pub flagbits: Option<FlagBitsDef>,
}

/// Model for the FlagBits portion of a bitmask.
#[derive(Debug, Clone)]
pub struct FlagBitsDef {
    pub name: &'static str,
    pub c_name: &'static str,
    pub bitwidth: u8,
    /// Base bits defined in the bitmask itself.
    pub base_bits: Vec<BitmaskBit>,
    /// Base-level aliases (from the bitmask definition).
    pub base_aliases: Vec<BitmaskAlias>,
    /// Module contributions (bits, aliases, values).
    pub module_groups: Vec<BitmaskModuleGroup>,
    /// All known bit names (for alias target validation).
    pub all_bit_names: Vec<String>,
    /// Non-bit values on the Flags type (e.g. NONE = 0, FRONT_AND_BACK = 0x3).
    pub flags_values: Vec<BitmaskValue>,
    /// Non-bit values contributed by modules.
    pub flags_module_values: Vec<BitmaskModuleValues>,
}

/// A single bit in a bitmask.
#[derive(Debug, Clone)]
pub struct BitmaskBit {
    pub name: String,
    pub bitpos: u8,
    pub comment: Option<&'static str>,
}

/// An alias for a bitmask bit.
#[derive(Debug, Clone)]
pub struct BitmaskAlias {
    pub name: String,
    pub target: String,
}

/// A literal value in a bitmask (non-single-bit, e.g. FRONT_AND_BACK = 0x3).
#[derive(Debug, Clone)]
pub struct BitmaskValue {
    pub name: String,
    pub value: &'static str,
    pub comment: Option<&'static str>,
}

/// Module contributions to a bitmask type.
#[derive(Debug, Clone)]
pub struct BitmaskModuleGroup {
    pub module_name: String,
    pub provisional: bool,
    pub bits: Vec<BitmaskBit>,
    pub aliases: Vec<BitmaskAlias>,
}

/// Module contributions of literal values to a Flags type.
#[derive(Debug, Clone)]
pub struct BitmaskModuleValues {
    pub module_name: String,
    pub provisional: bool,
    pub values: Vec<BitmaskValue>,
}

// ── Enum builder ─────────────────────────────────────────────────────────────

pub fn build_enum(analysis: &Analysis, ty: &xml::Enum) -> (EnumDef, Vec<EnumDebugVariant>) {
    let tags = &analysis.registry().tags;
    let req = analysis.req_enum_data();

    let value_prefix = if ty.name == "VkResult" {
        "VK".to_string()
    } else {
        let prefix = strip_vendor_suffix(ty.name, tags).to_shouty_snake_case();
        separate_trailing_number(&prefix).to_string()
    };

    let name = if ty.name == "VkResult" {
        "Result".to_string()
    } else {
        normalize_ty_name(ty.name).to_string()
    };

    let mut debug_variants: Vec<EnumDebugVariant> = Vec::new();
    let mut visited = HashSet::new();

    // Base variants
    let base_variants: Vec<EnumVariant> = ty
        .values
        .iter()
        .map(|bit| {
            let vname = normalize_variant_name(bit.name, &value_prefix);
            visited.insert(vname.clone());
            debug_variants.push(EnumDebugVariant {
                name: vname.clone(),
                provisional: false,
            });
            EnumVariant {
                name: vname,
                value: bit.value.to_string(),
                comment: bit.comment,
            }
        })
        .collect();

    // Module contributions
    let mut module_groups = Vec::new();
    if let Some(modules) = req.get(ty.name) {
        for (module_name, contributions) in modules {
            let provisional = analysis.is_provisional_extension(module_name);
            let mut variants = Vec::new();
            let mut aliases = Vec::new();

            for v in &contributions.variants {
                let vname = normalize_variant_name(v.name, &value_prefix);
                if visited.insert(vname.clone()) {
                    variants.push(EnumVariant {
                        name: vname.clone(),
                        value: v.value.to_string(),
                        comment: v.comment,
                    });
                    debug_variants.push(EnumDebugVariant {
                        name: vname,
                        provisional,
                    });
                }
            }
            for v in &contributions.values {
                let vname = normalize_variant_name(v.name, &value_prefix);
                if visited.insert(vname.clone()) {
                    variants.push(EnumVariant {
                        name: vname.clone(),
                        value: v.literal_value.to_string(),
                        comment: v.comment,
                    });
                    debug_variants.push(EnumDebugVariant {
                        name: vname,
                        provisional,
                    });
                }
            }
            for a in &contributions.aliases {
                let aname = normalize_variant_name(a.name, &value_prefix);
                if visited.insert(aname.clone()) {
                    let target = normalize_variant_name(a.target, &value_prefix);
                    aliases.push(EnumAlias {
                        name: aname,
                        target,
                    });
                }
            }

            module_groups.push(EnumModuleGroup {
                module_name: module_name.clone(),
                provisional,
                variants,
                aliases,
            });
        }
    }

    let def = EnumDef {
        name,
        c_name: ty.name,
        base_variants,
        module_groups,
    };

    (def, debug_variants)
}

// ── Bitmask builder ──────────────────────────────────────────────────────────

pub fn build_bitmask(
    analysis: &Analysis,
    ty: &xml::BitMaskType,
    bitmask: Option<&xml::BitMask>,
    req: &ReqEnumData,
) -> BitmaskDef {
    let flags_name = normalize_ty_name(ty.name);
    let base_type = base_ctype_to_rust_str(ty.ty);

    let flagbits = bitmask.map(|bitmask| build_flagbits(analysis, bitmask, req));

    BitmaskDef {
        flags_name,
        flags_c_name: ty.name,
        base_type,
        flagbits,
    }
}

fn build_flagbits(analysis: &Analysis, bitmask: &xml::BitMask, req: &ReqEnumData) -> FlagBitsDef {
    let tags = &analysis.registry().tags;
    let bitmask_name = normalize_ty_name(bitmask.name);

    let value_prefix = {
        let prefix = strip_vendor_suffix(bitmask.name, tags)
            .replace("FlagBits", "")
            .to_shouty_snake_case();
        separate_trailing_number(&prefix).to_string()
    };

    // Base bits
    let mut base_bits: Vec<BitmaskBit> = bitmask
        .bits
        .iter()
        .map(|bit| BitmaskBit {
            name: normalize_bit_name(bit.name, Some(value_prefix.as_str())),
            bitpos: bit.bitpos,
            comment: bit.comment,
        })
        .collect();
    base_bits.sort_by_key(|b| b.bitpos);

    // Base-level aliases
    let modules = req.get(bitmask.name);

    // Collect all bit names for alias validation
    let mut all_bit_names: HashSet<String> = base_bits.iter().map(|b| b.name.clone()).collect();
    if let Some(modules) = modules {
        for c in modules.values() {
            for bp in &c.bitpositions {
                all_bit_names.insert(normalize_bit_name(bp.name, Some(value_prefix.as_str())));
            }
        }
    }

    let base_aliases: Vec<BitmaskAlias> = bitmask
        .aliases
        .iter()
        .map(|alias| {
            let aname = normalize_bit_name(alias.name, Some(value_prefix.as_str()));
            let target = normalize_bit_name(alias.alias, Some(value_prefix.as_str()));
            BitmaskAlias {
                name: aname,
                target,
            }
        })
        .collect();

    // Flags-level values (non-bit values like NONE = 0)
    let flags_values: Vec<BitmaskValue> = bitmask
        .values
        .iter()
        .map(|value| {
            let vname = value
                .name
                .strip_prefix(&value_prefix)
                .unwrap()
                .strip_prefix('_')
                .unwrap();
            let vname = strip_vendor_suffix(vname, tags);
            BitmaskValue {
                name: vname.to_string(),
                value: value.value,
                comment: value.comment,
            }
        })
        .collect();

    // Module contributions
    let mut module_groups = Vec::new();
    let mut flags_module_values = Vec::new();

    if let Some(modules) = modules {
        for (name, c) in modules {
            let provisional = analysis.is_provisional_extension(name);

            let mut bits: Vec<BitmaskBit> = c
                .bitpositions
                .iter()
                .map(|bp| BitmaskBit {
                    name: normalize_bit_name(bp.name, Some(value_prefix.as_str())),
                    bitpos: bp.bitpos,
                    comment: bp.comment,
                })
                .collect();
            bits.sort_by_key(|b| b.bitpos);

            let aliases: Vec<BitmaskAlias> = c
                .aliases
                .iter()
                .map(|a| BitmaskAlias {
                    name: normalize_bit_name(a.name, Some(value_prefix.as_str())),
                    target: normalize_bit_name(a.target, Some(value_prefix.as_str())),
                })
                .collect();

            module_groups.push(BitmaskModuleGroup {
                module_name: name.clone(),
                provisional,
                bits,
                aliases,
            });

            let values: Vec<BitmaskValue> = c
                .values
                .iter()
                .map(|v| BitmaskValue {
                    name: normalize_bit_name(v.name, Some(value_prefix.as_str())),
                    value: v.literal_value,
                    comment: v.comment,
                })
                .collect();

            if !values.is_empty() {
                flags_module_values.push(BitmaskModuleValues {
                    module_name: name.clone(),
                    provisional,
                    values,
                });
            }
        }
    }

    let all_bit_names_vec: Vec<String> = all_bit_names.into_iter().collect();

    FlagBitsDef {
        name: bitmask_name,
        c_name: bitmask.name,
        bitwidth: bitmask.bitwidth.unwrap_or(32),
        base_bits,
        base_aliases,
        module_groups,
        all_bit_names: all_bit_names_vec,
        flags_values,
        flags_module_values,
    }
}

// ── Shared helpers ───────────────────────────────────────────────────────────

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
        format!("_{name}")
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
        format!("_{name}")
    } else {
        name
    }
}

static TRAILING_NUMBER: OnceLock<Regex> = OnceLock::new();

fn separate_trailing_number(name: &str) -> Cow<'_, str> {
    let trailing_number = TRAILING_NUMBER.get_or_init(|| regex::Regex::new(r"(\d+)$").unwrap());
    trailing_number.replace(name, "_$1")
}
