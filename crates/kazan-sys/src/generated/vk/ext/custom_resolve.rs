#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{c_char, c_int, c_void};
#[repr(C)]
pub struct BeginCustomResolveInfoEXT {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
}
#[repr(C)]
pub struct PhysicalDeviceCustomResolveFeaturesEXT {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub custom_resolve: Bool32,
}
#[repr(C)]
pub struct CustomResolveCreateInfoEXT {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub custom_resolve: Bool32,
    pub color_attachment_count: u32,
    pub p_color_attachment_formats: *const Format,
    pub depth_attachment_format: Format,
    pub stencil_attachment_format: Format,
}
pub type PFN_vkCmdBeginCustomResolveEXT = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    p_begin_custom_resolve_info: *const BeginCustomResolveInfoEXT,
);
