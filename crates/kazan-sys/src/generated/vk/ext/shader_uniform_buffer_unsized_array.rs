#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{CStr, c_char, c_int, c_void};
use core::marker::PhantomData;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceShaderUniformBufferUnsizedArrayFeaturesEXT<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub shader_uniform_buffer_unsized_array: Bool32,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for PhysicalDeviceShaderUniformBufferUnsizedArrayFeaturesEXT<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::PHYSICAL_DEVICE_SHADER_UNIFORM_BUFFER_UNSIZED_ARRAY_FEATURES_EXT,
            p_next: core::ptr::null_mut(),
            shader_uniform_buffer_unsized_array: Default::default(),
            _marker: PhantomData,
        }
    }
}
impl<'a> PhysicalDeviceShaderUniformBufferUnsizedArrayFeaturesEXT<'a> {
    pub fn shader_uniform_buffer_unsized_array(
        mut self,
        shader_uniform_buffer_unsized_array: Bool32,
    ) -> Self {
        self.shader_uniform_buffer_unsized_array = shader_uniform_buffer_unsized_array;
        self
    }
}
