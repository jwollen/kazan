#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use std::ffi::{c_char, c_int, c_void};
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceMaintenance10PropertiesKHR {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub rgba4_opaque_black_swizzled: Bool32,
    pub resolve_srgb_format_applies_transfer_function: Bool32,
    pub resolve_srgb_format_supports_transfer_function_control: Bool32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceMaintenance10FeaturesKHR {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub maintenance10: Bool32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct RenderingEndInfoKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct RenderingAttachmentFlagsInfoKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: RenderingAttachmentFlagsKHR,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ResolveImageModeInfoKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: ResolveImageFlagsKHR,
    pub resolve_mode: ResolveModeFlags,
    pub stencil_resolve_mode: ResolveModeFlags,
}
bitflags! {
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Default)]
    pub struct RenderingAttachmentFlagsKHR: Flags {
    }
}
bitflags! {
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Default)]
    pub struct ResolveImageFlagsKHR: Flags {
    }
}
pub type PFN_vkCmdEndRendering2KHR = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    p_rendering_end_info: *const RenderingEndInfoKHR,
);
