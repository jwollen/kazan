#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{c_char, c_int, c_void};
use core::marker::PhantomData;
#[repr(C)]
pub struct PhysicalDeviceShaderAtomicFloat2FeaturesEXT<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub shader_buffer_float16_atomics: Bool32,
    pub shader_buffer_float16_atomic_add: Bool32,
    pub shader_buffer_float16_atomic_min_max: Bool32,
    pub shader_buffer_float32_atomic_min_max: Bool32,
    pub shader_buffer_float64_atomic_min_max: Bool32,
    pub shader_shared_float16_atomics: Bool32,
    pub shader_shared_float16_atomic_add: Bool32,
    pub shader_shared_float16_atomic_min_max: Bool32,
    pub shader_shared_float32_atomic_min_max: Bool32,
    pub shader_shared_float64_atomic_min_max: Bool32,
    pub shader_image_float32_atomic_min_max: Bool32,
    pub sparse_image_float32_atomic_min_max: Bool32,
    pub _marker: PhantomData<&'a ()>,
}
