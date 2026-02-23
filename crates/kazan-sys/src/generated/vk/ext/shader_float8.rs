#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{c_char, c_int, c_void};
#[repr(C)]
pub struct PhysicalDeviceShaderFloat8FeaturesEXT {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub shader_float8: Bool32,
    pub shader_float8_cooperative_matrix: Bool32,
}
