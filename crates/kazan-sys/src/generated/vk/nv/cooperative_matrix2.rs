#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{CStr, c_char, c_int, c_void};
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
impl<'a> PhysicalDeviceCooperativeMatrix2FeaturesNV<'a> {
    pub fn cooperative_matrix_workgroup_scope(
        mut self,
        cooperative_matrix_workgroup_scope: Bool32,
    ) -> Self {
        self.cooperative_matrix_workgroup_scope = cooperative_matrix_workgroup_scope;
        self
    }
    pub fn cooperative_matrix_flexible_dimensions(
        mut self,
        cooperative_matrix_flexible_dimensions: Bool32,
    ) -> Self {
        self.cooperative_matrix_flexible_dimensions = cooperative_matrix_flexible_dimensions;
        self
    }
    pub fn cooperative_matrix_reductions(mut self, cooperative_matrix_reductions: Bool32) -> Self {
        self.cooperative_matrix_reductions = cooperative_matrix_reductions;
        self
    }
    pub fn cooperative_matrix_conversions(
        mut self,
        cooperative_matrix_conversions: Bool32,
    ) -> Self {
        self.cooperative_matrix_conversions = cooperative_matrix_conversions;
        self
    }
    pub fn cooperative_matrix_per_element_operations(
        mut self,
        cooperative_matrix_per_element_operations: Bool32,
    ) -> Self {
        self.cooperative_matrix_per_element_operations = cooperative_matrix_per_element_operations;
        self
    }
    pub fn cooperative_matrix_tensor_addressing(
        mut self,
        cooperative_matrix_tensor_addressing: Bool32,
    ) -> Self {
        self.cooperative_matrix_tensor_addressing = cooperative_matrix_tensor_addressing;
        self
    }
    pub fn cooperative_matrix_block_loads(
        mut self,
        cooperative_matrix_block_loads: Bool32,
    ) -> Self {
        self.cooperative_matrix_block_loads = cooperative_matrix_block_loads;
        self
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
impl<'a> PhysicalDeviceCooperativeMatrix2PropertiesNV<'a> {
    pub fn cooperative_matrix_workgroup_scope_max_workgroup_size(
        mut self,
        cooperative_matrix_workgroup_scope_max_workgroup_size: u32,
    ) -> Self {
        self.cooperative_matrix_workgroup_scope_max_workgroup_size =
            cooperative_matrix_workgroup_scope_max_workgroup_size;
        self
    }
    pub fn cooperative_matrix_flexible_dimensions_max_dimension(
        mut self,
        cooperative_matrix_flexible_dimensions_max_dimension: u32,
    ) -> Self {
        self.cooperative_matrix_flexible_dimensions_max_dimension =
            cooperative_matrix_flexible_dimensions_max_dimension;
        self
    }
    pub fn cooperative_matrix_workgroup_scope_reserved_shared_memory(
        mut self,
        cooperative_matrix_workgroup_scope_reserved_shared_memory: u32,
    ) -> Self {
        self.cooperative_matrix_workgroup_scope_reserved_shared_memory =
            cooperative_matrix_workgroup_scope_reserved_shared_memory;
        self
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
impl<'a> CooperativeMatrixFlexibleDimensionsPropertiesNV<'a> {
    pub fn m_granularity(mut self, m_granularity: u32) -> Self {
        self.m_granularity = m_granularity;
        self
    }
    pub fn n_granularity(mut self, n_granularity: u32) -> Self {
        self.n_granularity = n_granularity;
        self
    }
    pub fn k_granularity(mut self, k_granularity: u32) -> Self {
        self.k_granularity = k_granularity;
        self
    }
    pub fn a_type(mut self, a_type: ComponentTypeKHR) -> Self {
        self.a_type = a_type;
        self
    }
    pub fn b_type(mut self, b_type: ComponentTypeKHR) -> Self {
        self.b_type = b_type;
        self
    }
    pub fn c_type(mut self, c_type: ComponentTypeKHR) -> Self {
        self.c_type = c_type;
        self
    }
    pub fn result_type(mut self, result_type: ComponentTypeKHR) -> Self {
        self.result_type = result_type;
        self
    }
    pub fn saturating_accumulation(mut self, saturating_accumulation: Bool32) -> Self {
        self.saturating_accumulation = saturating_accumulation;
        self
    }
    pub fn scope(mut self, scope: ScopeKHR) -> Self {
        self.scope = scope;
        self
    }
    pub fn workgroup_invocations(mut self, workgroup_invocations: u32) -> Self {
        self.workgroup_invocations = workgroup_invocations;
        self
    }
}
pub type PFN_vkGetPhysicalDeviceCooperativeMatrixFlexibleDimensionsPropertiesNV =
    unsafe extern "system" fn(
        physical_device: PhysicalDevice,
        p_property_count: *mut u32,
        p_properties: *mut CooperativeMatrixFlexibleDimensionsPropertiesNV<'_>,
    ) -> Result;
