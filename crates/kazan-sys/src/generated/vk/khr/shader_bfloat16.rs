#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{c_char, c_int, c_void};
use core::marker::PhantomData;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceShaderBfloat16FeaturesKHR<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub shader_b_float16_type: Bool32,
    pub shader_b_float16_dot_product: Bool32,
    pub shader_b_float16_cooperative_matrix: Bool32,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for PhysicalDeviceShaderBfloat16FeaturesKHR<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::PHYSICAL_DEVICE_SHADER_BFLOAT16_FEATURES_KHR,
            p_next: core::ptr::null_mut(),
            shader_b_float16_type: Default::default(),
            shader_b_float16_dot_product: Default::default(),
            shader_b_float16_cooperative_matrix: Default::default(),
            _marker: PhantomData,
        }
    }
}
