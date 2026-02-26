#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{c_char, c_int, c_void};
use core::marker::PhantomData;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceCooperativeMatrix2FeaturesNV<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub cooperative_matrix_workgroup_scope: Bool32,
    pub cooperative_matrix_flexible_dimensions: Bool32,
    pub cooperative_matrix_reductions: Bool32,
    pub cooperative_matrix_conversions: Bool32,
    pub cooperative_matrix_per_element_operations: Bool32,
    pub cooperative_matrix_tensor_addressing: Bool32,
    pub cooperative_matrix_block_loads: Bool32,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for PhysicalDeviceCooperativeMatrix2FeaturesNV<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::PHYSICAL_DEVICE_COOPERATIVE_MATRIX_2_FEATURES_NV,
            p_next: core::ptr::null_mut(),
            cooperative_matrix_workgroup_scope: Default::default(),
            cooperative_matrix_flexible_dimensions: Default::default(),
            cooperative_matrix_reductions: Default::default(),
            cooperative_matrix_conversions: Default::default(),
            cooperative_matrix_per_element_operations: Default::default(),
            cooperative_matrix_tensor_addressing: Default::default(),
            cooperative_matrix_block_loads: Default::default(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceCooperativeMatrix2PropertiesNV<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub cooperative_matrix_workgroup_scope_max_workgroup_size: u32,
    pub cooperative_matrix_flexible_dimensions_max_dimension: u32,
    pub cooperative_matrix_workgroup_scope_reserved_shared_memory: u32,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for PhysicalDeviceCooperativeMatrix2PropertiesNV<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::PHYSICAL_DEVICE_COOPERATIVE_MATRIX_2_PROPERTIES_NV,
            p_next: core::ptr::null_mut(),
            cooperative_matrix_workgroup_scope_max_workgroup_size: Default::default(),
            cooperative_matrix_flexible_dimensions_max_dimension: Default::default(),
            cooperative_matrix_workgroup_scope_reserved_shared_memory: Default::default(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct CooperativeMatrixFlexibleDimensionsPropertiesNV<'a> {
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
    pub _marker: PhantomData<&'a ()>,
}
impl Default for CooperativeMatrixFlexibleDimensionsPropertiesNV<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::COOPERATIVE_MATRIX_FLEXIBLE_DIMENSIONS_PROPERTIES_NV,
            p_next: core::ptr::null_mut(),
            m_granularity: Default::default(),
            n_granularity: Default::default(),
            k_granularity: Default::default(),
            a_type: Default::default(),
            b_type: Default::default(),
            c_type: Default::default(),
            result_type: Default::default(),
            saturating_accumulation: Default::default(),
            scope: Default::default(),
            workgroup_invocations: Default::default(),
            _marker: PhantomData,
        }
    }
}
pub type PFN_vkGetPhysicalDeviceCooperativeMatrixFlexibleDimensionsPropertiesNV =
    unsafe extern "system" fn(
        physical_device: PhysicalDevice,
        p_property_count: *mut u32,
        p_properties: *mut CooperativeMatrixFlexibleDimensionsPropertiesNV<'_>,
    ) -> Result;
