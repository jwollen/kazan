#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{c_char, c_int, c_void};
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceShaderFmaFeaturesKHR {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub shader_fma_float16: Bool32,
    pub shader_fma_float32: Bool32,
    pub shader_fma_float64: Bool32,
}
