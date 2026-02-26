#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{c_char, c_int, c_void};
use core::marker::PhantomData;
#[repr(C)]
pub struct PhysicalDeviceVertexInputDynamicStateFeaturesEXT<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub vertex_input_dynamic_state: Bool32,
    pub _marker: PhantomData<&'a ()>,
}
#[repr(C)]
pub struct VertexInputBindingDescription2EXT<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub binding: u32,
    pub stride: u32,
    pub input_rate: VertexInputRate,
    pub divisor: u32,
    pub _marker: PhantomData<&'a ()>,
}
#[repr(C)]
pub struct VertexInputAttributeDescription2EXT<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub location: u32,
    pub binding: u32,
    pub format: Format,
    pub offset: u32,
    pub _marker: PhantomData<&'a ()>,
}
pub type PFN_vkCmdSetVertexInputEXT = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    vertex_binding_description_count: u32,
    p_vertex_binding_descriptions: *const VertexInputBindingDescription2EXT<'_>,
    vertex_attribute_description_count: u32,
    p_vertex_attribute_descriptions: *const VertexInputAttributeDescription2EXT<'_>,
);
