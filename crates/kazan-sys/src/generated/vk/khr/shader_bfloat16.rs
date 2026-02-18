#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use std::ffi::{c_char, c_int, c_void};
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceShaderBfloat16FeaturesKHR {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub shader_b_float16_type: Bool32,
    pub shader_b_float16_dot_product: Bool32,
    pub shader_b_float16_cooperative_matrix: Bool32,
}
