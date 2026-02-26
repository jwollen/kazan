#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{c_char, c_int, c_void};
use core::marker::PhantomData;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceComputeShaderDerivativesFeaturesKHR<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub compute_derivative_group_quads: Bool32,
    pub compute_derivative_group_linear: Bool32,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for PhysicalDeviceComputeShaderDerivativesFeaturesKHR<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::PHYSICAL_DEVICE_COMPUTE_SHADER_DERIVATIVES_FEATURES_KHR,
            p_next: core::ptr::null_mut(),
            compute_derivative_group_quads: Default::default(),
            compute_derivative_group_linear: Default::default(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceComputeShaderDerivativesPropertiesKHR<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub mesh_and_task_shader_derivatives: Bool32,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for PhysicalDeviceComputeShaderDerivativesPropertiesKHR<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::PHYSICAL_DEVICE_COMPUTE_SHADER_DERIVATIVES_PROPERTIES_KHR,
            p_next: core::ptr::null_mut(),
            mesh_and_task_shader_derivatives: Default::default(),
            _marker: PhantomData,
        }
    }
}
