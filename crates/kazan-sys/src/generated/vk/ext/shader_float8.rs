#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{c_char, c_int, c_void};
use core::marker::PhantomData;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceShaderFloat8FeaturesEXT<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub shader_float8: Bool32,
    pub shader_float8_cooperative_matrix: Bool32,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for PhysicalDeviceShaderFloat8FeaturesEXT<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::PHYSICAL_DEVICE_SHADER_FLOAT8_FEATURES_EXT,
            p_next: core::ptr::null_mut(),
            shader_float8: Default::default(),
            shader_float8_cooperative_matrix: Default::default(),
            _marker: PhantomData,
        }
    }
}
