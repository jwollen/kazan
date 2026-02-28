use std::{
    borrow::Cow,
    collections::{BTreeMap, HashSet},
    sync::OnceLock,
};

use heck::ToShoutySnakeCase;
use regex::Regex;

use crate::{analysis::Analysis, ctype_to_rust_type_str, normalize_ty_name, xml};

/// Data from `<require>` blocks: extension/version enum variants, aliases, values, and bit positions.
pub struct ReqEnumData<'a> {
    pub enum_variants: &'a BTreeMap<&'static str, BTreeMap<&'static str, i32>>,
    pub enum_aliases: &'a BTreeMap<&'static str, BTreeMap<&'static str, &'static str>>,
    pub enum_values: &'a BTreeMap<&'static str, BTreeMap<&'static str, &'static str>>,
    pub bitspos: &'a BTreeMap<&'static str, BTreeMap<&'static str, u8>>,
}

fn strip_vendor_suffix(name: &str) -> &str {
    let vendors = [
        "AMD",
        "AMDX",
        "ANDROID",
        "ARM",
        "BRCM",
        "CHROMIUM",
        "EXT",
        "FB",
        "FSL",
        "FUCHSIA",
        "GGP",
        "GOOGLE",
        "HUAWEI",
        "IMG",
        "INTEL",
        "JUICE",
        "KDAB",
        "KHR",
        "KHX",
        "LUNARG",
        "MESA",
        "MSFT",
        "MVK",
        "NN",
        "NV",
        "NVX",
        "NXP",
        "NZXT",
        "OHOS",
        "QCOM",
        "QNX",
        "RASTERGRID",
        "RENDERDOC",
        "SAMSUNG",
        "SEC",
        "TIZEN",
        "VALVE",
        "VIV",
        "VSI",
        "VK",
        "VN",
        "VNN",
        "WEIXIN",
        "XR",
    ];
    let vendor = vendors
        .iter()
        .find(|&&vendor| name.ends_with(vendor))
        .copied();
    if let Some(vendor) = vendor {
        let name = name.strip_suffix(vendor).unwrap();
        if name.ends_with('_') {
            name.strip_suffix('_').unwrap()
        } else {
            name
        }
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
    if name
        .chars()
        .next()
        .map(|c| c.is_ascii_digit())
        .unwrap_or_default()
    {
        format!("_{}", name)
    } else {
        name.to_string()
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
    if name
        .chars()
        .next()
        .map(|c| c.is_ascii_digit())
        .unwrap_or_default()
    {
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

pub fn write_enum(file: &mut impl std::io::Write, req: &ReqEnumData<'_>, ty: &xml::Enum) {
    let value_prefix = if ty.name == "VkResult" {
        "VK".to_string()
    } else {
        let prefix = strip_vendor_suffix(ty.name).to_shouty_snake_case();
        separate_trailing_number(&prefix).to_string()
    };

    let variants = {
        let bits = ty.values.iter().map(|bit| {
            (
                normalize_variant_name(bit.name, &value_prefix),
                bit.value.to_string(),
            )
        });

        let req_enum = req.enum_variants.get(ty.name);
        let req_variants = req_enum.iter().flat_map(|bits| {
            bits.iter().map(|(name, variant)| {
                (
                    normalize_variant_name(name, &value_prefix),
                    variant.to_string(),
                )
            })
        });

        let bits = bits.chain(req_variants).collect::<Vec<_>>();
        bits
    };

    let name = if ty.name == "VkResult" {
        "Result"
    } else {
        normalize_ty_name(ty.name)
    };

    writeln!(
        file,
        "#[repr(transparent)]
        #[derive(Copy, Clone, Default,PartialEq, Eq, PartialOrd, Ord, Hash)]
        pub struct {}(i32);",
        name
    )
    .unwrap();

    writeln!(file, "impl {} {{", name).unwrap();
    for (name, value) in &variants {
        writeln!(file, "pub const {}: Self = Self({});", name, value).unwrap();
    }

    if let Some(values) = req.enum_values.get(ty.name) {
        for (name, value) in values {
            let name = normalize_variant_name(name, &value_prefix);
            writeln!(file, "pub const {}: Self = Self({});", name, value).unwrap();
        }
    }

    if let Some(aliases) = req.enum_aliases.get(ty.name) {
        for (name, alias) in aliases {
            let name = normalize_variant_name(name, &value_prefix);
            let alias = normalize_variant_name(alias, &value_prefix);
            writeln!(file, "pub const {}: Self = Self::{};", name, alias).unwrap();
        }
    }

    writeln!(file, "}}").unwrap();
}

/// Writes a bitmask type (bitflags + optional FlagBits struct) to `file`.
pub fn write_bitmask(
    file: &mut impl std::io::Write,
    _analysis: &Analysis,
    ty: &xml::BitMaskType,
    bitmask: Option<&xml::BitMask>,
    req: &ReqEnumData<'_>,
) {
    let name = normalize_ty_name(ty.name);
    let base_type = ctype_to_rust_type_str(ty.ty);

    writeln!(
        file,
        "bitflags! {{
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct {}: {} {{",
        name, base_type
    )
    .unwrap();

    let value_prefix = bitmask.map(|bitmask| {
        let prefix = strip_vendor_suffix(bitmask.name)
            .replace("FlagBits", "")
            .to_shouty_snake_case();
        separate_trailing_number(&prefix).to_string()
    });

    let bits = if let Some(bitmask) = bitmask {
        let bits = bitmask.bits.iter().map(|bit| {
            (
                normalize_bit_name(bit.name, value_prefix.as_deref()),
                bit.bitpos,
            )
        });

        let req_bitmask = req.bitspos.get(bitmask.name);
        let req_bits = req_bitmask.iter().flat_map(|bits| {
            bits.iter()
                .map(|(name, bitpos)| (normalize_bit_name(name, value_prefix.as_deref()), *bitpos))
        });

        let mut bits = bits.chain(req_bits).collect::<Vec<_>>();
        bits.sort_by_key(|(_, bit)| *bit);
        bits
    } else {
        Vec::new()
    };

    if let Some(bitmask) = bitmask {
        let bitmask_name = normalize_ty_name(bitmask.name);

        let mut visited_bits = HashSet::new();

        for (name, _bit) in &bits {
            if !visited_bits.insert(name.clone()) {
                continue;
            }
            writeln!(
                file,
                "        const {} = {}::{}.0;",
                name, bitmask_name, name
            )
            .unwrap();
        }

        for alias in &bitmask.aliases {
            let name = normalize_bit_name(alias.name, value_prefix.as_deref());
            let alias = normalize_bit_name(alias.alias, value_prefix.as_deref());

            if !visited_bits.insert(name.clone()) {
                continue;
            }

            writeln!(file, "        const {} = Self::{}.bits();", name, alias).unwrap();
        }

        if let Some(aliases) = req.enum_aliases.get(bitmask.name) {
            for (name, alias) in aliases {
                let name = normalize_bit_name(name, value_prefix.as_deref());
                let alias = normalize_bit_name(alias, value_prefix.as_deref());

                if !visited_bits.insert(name.clone()) {
                    continue;
                }

                writeln!(file, "        const {} = Self::{}.bits();", name, alias).unwrap();
            }
        }

        for value in &bitmask.values {
            let name = value
                .name
                .strip_prefix(value_prefix.as_ref().unwrap())
                .unwrap()
                .strip_prefix('_')
                .unwrap();
            let name = strip_vendor_suffix(name);
            writeln!(file, "        const {} = {};", name, value.value).unwrap();
        }

        if let Some(values) = req.enum_values.get(bitmask.name) {
            for (name, value) in values {
                let name = normalize_bit_name(name, value_prefix.as_deref());
                writeln!(file, "        const {} = {};", name, value).unwrap();
            }
        }
    }

    writeln!(file, "    }}").unwrap();
    writeln!(file, "}}").unwrap();

    if let Some(bitmask) = bitmask {
        let bitmask_name = normalize_ty_name(bitmask.name);
        writeln!(
            file,
            "#[repr(transparent)]
            #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
            pub struct {}(u{});
            impl {} {{",
            bitmask_name,
            bitmask.bitwidth.unwrap_or(32),
            bitmask_name,
        )
        .unwrap();

        let mut visited_bits = HashSet::new();

        for (name, bit) in &bits {
            if !visited_bits.insert(name.clone()) {
                continue;
            }
            writeln!(file, "pub const {}: Self = Self(1 << {});", name, bit).unwrap();
        }

        if let Some(aliases) = req.enum_aliases.get(bitmask.name) {
            for (name, alias) in aliases {
                let name = normalize_bit_name(name, value_prefix.as_deref());
                let alias = normalize_bit_name(alias, value_prefix.as_deref());

                if !bits.iter().any(|(n, _)| n.as_str() == alias) {
                    continue;
                }

                if !visited_bits.insert(name.clone()) {
                    continue;
                }

                writeln!(file, "pub const {}: Self = Self::{};", name, alias).unwrap();
            }
        }

        writeln!(file, "}}").unwrap();
    }
}
