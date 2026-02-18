#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use std::ffi::{c_char, c_int, c_void};
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceFragmentShaderBarycentricFeaturesKHR {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub fragment_shader_barycentric: Bool32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceFragmentShaderBarycentricPropertiesKHR {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub tri_strip_vertex_order_independent_of_provoking_vertex: Bool32,
}
