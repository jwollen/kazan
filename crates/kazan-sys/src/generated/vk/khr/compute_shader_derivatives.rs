#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{c_char, c_int, c_void};
#[repr(C)]
pub struct PhysicalDeviceComputeShaderDerivativesFeaturesKHR {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub compute_derivative_group_quads: Bool32,
    pub compute_derivative_group_linear: Bool32,
}
#[repr(C)]
pub struct PhysicalDeviceComputeShaderDerivativesPropertiesKHR {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub mesh_and_task_shader_derivatives: Bool32,
}
