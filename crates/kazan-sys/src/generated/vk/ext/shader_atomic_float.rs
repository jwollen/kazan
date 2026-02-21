#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{c_char, c_int, c_void};
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceShaderAtomicFloatFeaturesEXT {
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
}
