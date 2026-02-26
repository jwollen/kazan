#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{c_char, c_int, c_void};
use core::marker::PhantomData;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct BeginCustomResolveInfoEXT<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for BeginCustomResolveInfoEXT<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::BEGIN_CUSTOM_RESOLVE_INFO_EXT,
            p_next: core::ptr::null_mut(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceCustomResolveFeaturesEXT<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub custom_resolve: Bool32,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for PhysicalDeviceCustomResolveFeaturesEXT<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::PHYSICAL_DEVICE_CUSTOM_RESOLVE_FEATURES_EXT,
            p_next: core::ptr::null_mut(),
            custom_resolve: Default::default(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct CustomResolveCreateInfoEXT<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub custom_resolve: Bool32,
    pub color_attachment_count: u32,
    pub p_color_attachment_formats: *const Format,
    pub depth_attachment_format: Format,
    pub stencil_attachment_format: Format,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for CustomResolveCreateInfoEXT<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::CUSTOM_RESOLVE_CREATE_INFO_EXT,
            p_next: core::ptr::null(),
            custom_resolve: Default::default(),
            color_attachment_count: Default::default(),
            p_color_attachment_formats: core::ptr::null(),
            depth_attachment_format: Default::default(),
            stencil_attachment_format: Default::default(),
            _marker: PhantomData,
        }
    }
}
pub type PFN_vkCmdBeginCustomResolveEXT = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    p_begin_custom_resolve_info: *const BeginCustomResolveInfoEXT<'_>,
);
