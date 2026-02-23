#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{c_char, c_int, c_void};
#[repr(C)]
pub struct PhysicalDeviceColorWriteEnableFeaturesEXT {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub color_write_enable: Bool32,
}
#[repr(C)]
pub struct PipelineColorWriteCreateInfoEXT {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub attachment_count: u32,
    pub p_color_write_enables: *const Bool32,
}
pub type PFN_vkCmdSetColorWriteEnableEXT = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    attachment_count: u32,
    p_color_write_enables: *const Bool32,
);
