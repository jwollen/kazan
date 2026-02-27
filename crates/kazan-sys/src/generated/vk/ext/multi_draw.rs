#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{CStr, c_char, c_int, c_void};
use core::marker::PhantomData;
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct MultiDrawInfoEXT {
    pub first_vertex: u32,
    pub vertex_count: u32,
}
impl MultiDrawInfoEXT {
    pub fn first_vertex(mut self, first_vertex: u32) -> Self {
        self.first_vertex = first_vertex;
        self
    }
    pub fn vertex_count(mut self, vertex_count: u32) -> Self {
        self.vertex_count = vertex_count;
        self
    }
}
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct MultiDrawIndexedInfoEXT {
    pub first_index: u32,
    pub index_count: u32,
    pub vertex_offset: i32,
}
impl MultiDrawIndexedInfoEXT {
    pub fn first_index(mut self, first_index: u32) -> Self {
        self.first_index = first_index;
        self
    }
    pub fn index_count(mut self, index_count: u32) -> Self {
        self.index_count = index_count;
        self
    }
    pub fn vertex_offset(mut self, vertex_offset: i32) -> Self {
        self.vertex_offset = vertex_offset;
        self
    }
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
impl<'a> PhysicalDeviceMultiDrawPropertiesEXT<'a> {
    pub fn max_multi_draw_count(mut self, max_multi_draw_count: u32) -> Self {
        self.max_multi_draw_count = max_multi_draw_count;
        self
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
impl<'a> PhysicalDeviceMultiDrawFeaturesEXT<'a> {
    pub fn multi_draw(mut self, multi_draw: Bool32) -> Self {
        self.multi_draw = multi_draw;
        self
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
