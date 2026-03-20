use std::io::Write;

use anyhow::Result;

use crate::analysis::CommandType;

/// Write a hand-crafted command wrapper override, returning `true` if one exists.
///
/// Some Vulkan commands have semantics that can't be expressed correctly by the
/// general-purpose wrapper generator. This function provides per-command overrides.
pub fn write_command_override(
    _file: &mut impl Write,
    _command_name: &str,
    _optional: bool,
) -> Result<bool> {
    Ok(false)
}

/// How to represent the success code in the return type when there are multiple ok codes.
#[allow(dead_code)]
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
        _ => format!("{base}s"),
    }
}

/// Returns true if a struct member should not get a setter.
///
/// Used to suppress setters for reserved/padding fields.
pub fn skip_setter(_struct_name: &str, field_name: &str) -> bool {
    field_name == "reserved"
}

/// Override the Rust type for a struct member or command parameter based on its C name.
///
/// Returns `Some("ApiVersion")` for `apiVersion` fields, replacing the default `u32`.
pub fn member_type_override(c_name: &str) -> Option<&'static str> {
    match c_name {
        "apiVersion" => Some("ApiVersion"),
        _ => None,
    }
}

/// How to classify a command into a dispatch table.
pub enum CommandTypeOp {
    /// Exclude from generated `*Fn` structs entirely (provided by hand-written code).
    Skip,
    /// Use the default classification based on the first parameter's handle type.
    Default,
    /// Place on a specific dispatch table, overriding the default.
    Override(CommandType),
}

/// Override the dispatch table a command belongs to.
///
/// By default, commands are classified by their first parameter's handle type.
pub fn command_type_override(command_name: &str) -> CommandTypeOp {
    match command_name {
        // Provided by the hand-written `StaticFn` struct, loaded directly from the DLL.
        "vkGetInstanceProcAddr" => CommandTypeOp::Skip,
        // vkGetDeviceProcAddr must live on InstanceFn because DeviceFn is loaded *using* it.
        "vkGetDeviceProcAddr" => CommandTypeOp::Override(CommandType::Instance),
        _ => CommandTypeOp::Default,
    }
}

/// Confirms whether a noautovalidity command array parameter has a nullable pointer.
///
/// `noautovalidity` in vk.xml can mean either:
/// - The pointer itself may be NULL (when count is optional)
/// - Individual elements may be null handles (pointer always valid)
///
/// Since this is ambiguous, each case must be explicitly confirmed here.
/// Panics on unknown parameters to force review when vk.xml is updated.
pub fn noautovalidity_pointer_nullable(command_name: &str, param_name: &str) -> bool {
    match (command_name, param_name) {
        // Pointer nullable: count is optional, pointer ignored/null when count=0
        ("vkCmdDrawMultiEXT", "pVertexInfo") => true,
        ("vkCmdDrawMultiIndexedEXT", "pIndexInfo") => true,
        ("vkCmdBindTransformFeedbackBuffersEXT", "pSizes") => true,
        ("vkCmdBeginTransformFeedbackEXT", "pCounterBuffers") => true,
        ("vkCmdEndTransformFeedbackEXT", "pCounterBuffers") => true,
        // Element validity only: elements can be VK_NULL_HANDLE, pointer always valid
        ("vkFreeDescriptorSets", "pDescriptorSets") => false,
        ("vkFreeCommandBuffers", "pCommandBuffers") => false,
        _ => panic!(
            "Unconfirmed noautovalidity array command parameter: \
             {command_name}::{param_name}. \
             Add to overrides::noautovalidity_pointer_nullable()."
        ),
    }
}

/// Deprecated struct member override.
///
/// When a struct member has `deprecated` in vk.xml, the spec may freely change its other
/// attributes (`len`, `optional`, etc.) in future updates. To prevent those changes from
/// silently altering the generated API, every deprecated member must be explicitly listed here.
///
/// Returns `Some(reason)` to allow the deprecated member (the reason must match the XML's
/// `deprecated` value). Returns `None` (the default) to reject it, which causes a panic
/// during generation so new cases are caught immediately.
pub fn allow_deprecated_member(struct_name: &str, member_name: &str) -> Option<&'static str> {
    match (struct_name, member_name) {
        // Device layers were deprecated in Vulkan 1.0.13 and fully unused since 1.4.347.
        // Setters are preserved for backward compatibility with older drivers.
        ("VkDeviceCreateInfo", "enabledLayerCount" | "ppEnabledLayerNames") => Some("unused"),
        _ => None,
    }
}

/// Override a struct member's `len` attribute.
///
/// When the spec removes or changes `len` on a (typically deprecated) member, this override
/// restores it so the generated setter signature stays stable.
///
/// Returns `Some(slice)` to replace the member's `len`, or `None` to use the XML value as-is.
pub fn member_len_override(struct_name: &str, member_name: &str) -> Option<Vec<&'static str>> {
    match (struct_name, member_name) {
        // len was removed in 1.4.347 when deprecated changed from "ignored" to "unused".
        ("VkDeviceCreateInfo", "ppEnabledLayerNames") => {
            Some(vec!["enabledLayerCount", "null-terminated"])
        }
        _ => None,
    }
}

/// Override a struct member's `optional` attribute.
///
/// When the spec removes or changes `optional` on a (typically deprecated) member, this
/// override restores it so the generated setter signature stays stable.
///
/// Returns `Some(slice)` to replace the member's `optional`, or `None` to use the XML value.
pub fn member_optional_override(struct_name: &str, member_name: &str) -> Option<Vec<&'static str>> {
    match (struct_name, member_name) {
        // optional was removed in 1.4.347 when deprecated changed from "ignored" to "unused".
        ("VkDeviceCreateInfo", "enabledLayerCount") => Some(vec!["true"]),
        _ => None,
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
