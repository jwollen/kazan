#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{c_char, c_int, c_void};
use core::marker::PhantomData;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceShaderAtomicFloat16VectorFeaturesNV<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub shader_float16_vector_atomics: Bool32,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for PhysicalDeviceShaderAtomicFloat16VectorFeaturesNV<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::PHYSICAL_DEVICE_SHADER_ATOMIC_FLOAT16_VECTOR_FEATURES_NV,
            p_next: core::ptr::null_mut(),
            shader_float16_vector_atomics: Default::default(),
            _marker: PhantomData,
        }
    }
}
