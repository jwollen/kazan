#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{c_char, c_int, c_void};
use core::marker::PhantomData;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceColorWriteEnableFeaturesEXT<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub color_write_enable: Bool32,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for PhysicalDeviceColorWriteEnableFeaturesEXT<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::PHYSICAL_DEVICE_COLOR_WRITE_ENABLE_FEATURES_EXT,
            p_next: core::ptr::null_mut(),
            color_write_enable: Default::default(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PipelineColorWriteCreateInfoEXT<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub attachment_count: u32,
    pub p_color_write_enables: *const Bool32,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for PipelineColorWriteCreateInfoEXT<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::PIPELINE_COLOR_WRITE_CREATE_INFO_EXT,
            p_next: core::ptr::null(),
            attachment_count: Default::default(),
            p_color_write_enables: core::ptr::null(),
            _marker: PhantomData,
        }
    }
}
pub type PFN_vkCmdSetColorWriteEnableEXT = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    attachment_count: u32,
    p_color_write_enables: *const Bool32,
);
