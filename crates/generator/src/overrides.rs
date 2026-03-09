use std::io::Write;

use anyhow::Result;

/// Write a hand-crafted command wrapper override, returning `true` if one exists.
///
/// Some Vulkan commands have semantics that can't be expressed correctly by the
/// general-purpose wrapper generator. This function provides per-command overrides.
pub fn write_command_override(
    _file: &mut impl Write,
    command_name: &str,
    _optional: bool,
) -> Result<bool> {
    match command_name {
        _ => Ok(false),
    }
}

/// How to represent the success code in the return type when there are multiple ok codes.
pub enum SuccessCodeRepr {
    /// Return the raw `VkResult` so the caller can match on it.
    RawResult,
    /// Map to `bool`: `false` for the first ok code, `true` for the second.
    /// Only valid when there are exactly 2 ok codes.
    Bool,
}

pub struct OkCodes<'a> {
    pub codes: &'a [&'a str],
    pub repr: SuccessCodeRepr,
}

/// Return the setter name for a merged array setter (2+ params sharing a length field).
///
/// `len_member` is the C length member name (e.g. `"descriptorCount"`).
/// `base` is the snake_case name derived by stripping `_count` from the normalized length member.
///
/// The default is to append `s` to `base` (unless it already ends in `s`).
/// Override specific cases for irregular plurals.
pub fn merged_setter_name(_struct_name: &str, len_member: &str, base: &str) -> String {
    match len_member {
        "binaryCount" => "binaries".to_string(),
        "geometryCount" => "geometries".to_string(),
        _ if base.ends_with('s') => base.to_string(),
        _ => format!("{}s", base),
    }
}

/// Override which success codes mean "output is valid" for a command.
///
/// By default, only `VK_SUCCESS` maps to `Ok`. Some commands write their output
/// params on additional success codes (e.g. `VK_SUBOPTIMAL_KHR` for acquire image).
/// Returns `None` to use the default (SUCCESS only).
pub fn ok_codes(command_name: &str) -> Option<OkCodes<'static>> {
    match command_name {
        // pImageIndex is written on SUCCESS and SUBOPTIMAL_KHR,
        // but NOT on TIMEOUT or NOT_READY.
        "vkAcquireNextImageKHR" | "vkAcquireNextImage2KHR" => Some(OkCodes {
            codes: &["VK_SUCCESS", "VK_SUBOPTIMAL_KHR"],
            repr: SuccessCodeRepr::Bool,
        }),
        _ => None,
    }
}
