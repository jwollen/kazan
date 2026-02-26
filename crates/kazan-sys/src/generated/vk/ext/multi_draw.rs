#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{c_char, c_int, c_void};
use core::marker::PhantomData;
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct MultiDrawInfoEXT {
    pub first_vertex: u32,
    pub vertex_count: u32,
}
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct MultiDrawIndexedInfoEXT {
    pub first_index: u32,
    pub index_count: u32,
    pub vertex_offset: i32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceMultiDrawPropertiesEXT<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub max_multi_draw_count: u32,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for PhysicalDeviceMultiDrawPropertiesEXT<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::PHYSICAL_DEVICE_MULTI_DRAW_PROPERTIES_EXT,
            p_next: core::ptr::null_mut(),
            max_multi_draw_count: Default::default(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceMultiDrawFeaturesEXT<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub multi_draw: Bool32,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for PhysicalDeviceMultiDrawFeaturesEXT<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::PHYSICAL_DEVICE_MULTI_DRAW_FEATURES_EXT,
            p_next: core::ptr::null_mut(),
            multi_draw: Default::default(),
            _marker: PhantomData,
        }
    }
}
pub type PFN_vkCmdDrawMultiEXT = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    draw_count: u32,
    p_vertex_info: *const MultiDrawInfoEXT,
    instance_count: u32,
    first_instance: u32,
    stride: u32,
);
pub type PFN_vkCmdDrawMultiIndexedEXT = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    draw_count: u32,
    p_index_info: *const MultiDrawIndexedInfoEXT,
    instance_count: u32,
    first_instance: u32,
    stride: u32,
    p_vertex_offset: *const i32,
);
