#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{c_char, c_int, c_void};
use core::marker::PhantomData;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceVertexInputDynamicStateFeaturesEXT<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub vertex_input_dynamic_state: Bool32,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for PhysicalDeviceVertexInputDynamicStateFeaturesEXT<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::PHYSICAL_DEVICE_VERTEX_INPUT_DYNAMIC_STATE_FEATURES_EXT,
            p_next: core::ptr::null_mut(),
            vertex_input_dynamic_state: Default::default(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VertexInputBindingDescription2EXT<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub binding: u32,
    pub stride: u32,
    pub input_rate: VertexInputRate,
    pub divisor: u32,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for VertexInputBindingDescription2EXT<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::VERTEX_INPUT_BINDING_DESCRIPTION_2_EXT,
            p_next: core::ptr::null_mut(),
            binding: Default::default(),
            stride: Default::default(),
            input_rate: Default::default(),
            divisor: Default::default(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VertexInputAttributeDescription2EXT<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub location: u32,
    pub binding: u32,
    pub format: Format,
    pub offset: u32,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for VertexInputAttributeDescription2EXT<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::VERTEX_INPUT_ATTRIBUTE_DESCRIPTION_2_EXT,
            p_next: core::ptr::null_mut(),
            location: Default::default(),
            binding: Default::default(),
            format: Default::default(),
            offset: Default::default(),
            _marker: PhantomData,
        }
    }
}
pub type PFN_vkCmdSetVertexInputEXT = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    vertex_binding_description_count: u32,
    p_vertex_binding_descriptions: *const VertexInputBindingDescription2EXT<'_>,
    vertex_attribute_description_count: u32,
    p_vertex_attribute_descriptions: *const VertexInputAttributeDescription2EXT<'_>,
);
