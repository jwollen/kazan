#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{c_char, c_int, c_void};
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceCooperativeMatrix2FeaturesNV {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub cooperative_matrix_workgroup_scope: Bool32,
    pub cooperative_matrix_flexible_dimensions: Bool32,
    pub cooperative_matrix_reductions: Bool32,
    pub cooperative_matrix_conversions: Bool32,
    pub cooperative_matrix_per_element_operations: Bool32,
    pub cooperative_matrix_tensor_addressing: Bool32,
    pub cooperative_matrix_block_loads: Bool32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceCooperativeMatrix2PropertiesNV {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub cooperative_matrix_workgroup_scope_max_workgroup_size: u32,
    pub cooperative_matrix_flexible_dimensions_max_dimension: u32,
    pub cooperative_matrix_workgroup_scope_reserved_shared_memory: u32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct CooperativeMatrixFlexibleDimensionsPropertiesNV {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub m_granularity: u32,
    pub n_granularity: u32,
    pub k_granularity: u32,
    pub a_type: ComponentTypeKHR,
    pub b_type: ComponentTypeKHR,
    pub c_type: ComponentTypeKHR,
    pub result_type: ComponentTypeKHR,
    pub saturating_accumulation: Bool32,
    pub scope: ScopeKHR,
    pub workgroup_invocations: u32,
}
pub type PFN_vkGetPhysicalDeviceCooperativeMatrixFlexibleDimensionsPropertiesNV =
    unsafe extern "system" fn(
        physical_device: PhysicalDevice,
        p_property_count: *mut u32,
        p_properties: *mut CooperativeMatrixFlexibleDimensionsPropertiesNV,
    ) -> Result;
