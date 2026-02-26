#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{c_char, c_int, c_void};
use core::marker::PhantomData;
pub type ScopeNV = ScopeKHR;
pub type ComponentTypeNV = ComponentTypeKHR;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceCooperativeMatrixFeaturesNV<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub cooperative_matrix: Bool32,
    pub cooperative_matrix_robust_buffer_access: Bool32,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for PhysicalDeviceCooperativeMatrixFeaturesNV<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::PHYSICAL_DEVICE_COOPERATIVE_MATRIX_FEATURES_NV,
            p_next: core::ptr::null_mut(),
            cooperative_matrix: Default::default(),
            cooperative_matrix_robust_buffer_access: Default::default(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceCooperativeMatrixPropertiesNV<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub cooperative_matrix_supported_stages: ShaderStageFlags,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for PhysicalDeviceCooperativeMatrixPropertiesNV<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::PHYSICAL_DEVICE_COOPERATIVE_MATRIX_PROPERTIES_NV,
            p_next: core::ptr::null_mut(),
            cooperative_matrix_supported_stages: Default::default(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct CooperativeMatrixPropertiesNV<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub m_size: u32,
    pub n_size: u32,
    pub k_size: u32,
    pub a_type: ComponentTypeNV,
    pub b_type: ComponentTypeNV,
    pub c_type: ComponentTypeNV,
    pub d_type: ComponentTypeNV,
    pub scope: ScopeNV,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for CooperativeMatrixPropertiesNV<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::COOPERATIVE_MATRIX_PROPERTIES_NV,
            p_next: core::ptr::null_mut(),
            m_size: Default::default(),
            n_size: Default::default(),
            k_size: Default::default(),
            a_type: Default::default(),
            b_type: Default::default(),
            c_type: Default::default(),
            d_type: Default::default(),
            scope: Default::default(),
            _marker: PhantomData,
        }
    }
}
pub type PFN_vkGetPhysicalDeviceCooperativeMatrixPropertiesNV = unsafe extern "system" fn(
    physical_device: PhysicalDevice,
    p_property_count: *mut u32,
    p_properties: *mut CooperativeMatrixPropertiesNV<'_>,
)
    -> Result;
