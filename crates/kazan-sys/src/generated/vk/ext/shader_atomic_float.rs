#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{c_char, c_int, c_void};
use core::marker::PhantomData;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceShaderAtomicFloatFeaturesEXT<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub shader_buffer_float32_atomics: Bool32,
    pub shader_buffer_float32_atomic_add: Bool32,
    pub shader_buffer_float64_atomics: Bool32,
    pub shader_buffer_float64_atomic_add: Bool32,
    pub shader_shared_float32_atomics: Bool32,
    pub shader_shared_float32_atomic_add: Bool32,
    pub shader_shared_float64_atomics: Bool32,
    pub shader_shared_float64_atomic_add: Bool32,
    pub shader_image_float32_atomics: Bool32,
    pub shader_image_float32_atomic_add: Bool32,
    pub sparse_image_float32_atomics: Bool32,
    pub sparse_image_float32_atomic_add: Bool32,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for PhysicalDeviceShaderAtomicFloatFeaturesEXT<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::PHYSICAL_DEVICE_SHADER_ATOMIC_FLOAT_FEATURES_EXT,
            p_next: core::ptr::null_mut(),
            shader_buffer_float32_atomics: Default::default(),
            shader_buffer_float32_atomic_add: Default::default(),
            shader_buffer_float64_atomics: Default::default(),
            shader_buffer_float64_atomic_add: Default::default(),
            shader_shared_float32_atomics: Default::default(),
            shader_shared_float32_atomic_add: Default::default(),
            shader_shared_float64_atomics: Default::default(),
            shader_shared_float64_atomic_add: Default::default(),
            shader_image_float32_atomics: Default::default(),
            shader_image_float32_atomic_add: Default::default(),
            sparse_image_float32_atomics: Default::default(),
            sparse_image_float32_atomic_add: Default::default(),
            _marker: PhantomData,
        }
    }
}
