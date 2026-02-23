#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{c_char, c_int, c_void};
pub type ScopeNV = ScopeKHR;
pub type ComponentTypeNV = ComponentTypeKHR;
#[repr(C)]
pub struct PhysicalDeviceCooperativeMatrixFeaturesNV {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub cooperative_matrix: Bool32,
    pub cooperative_matrix_robust_buffer_access: Bool32,
}
#[repr(C)]
pub struct PhysicalDeviceCooperativeMatrixPropertiesNV {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub cooperative_matrix_supported_stages: ShaderStageFlags,
}
#[repr(C)]
pub struct CooperativeMatrixPropertiesNV {
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
}
pub type PFN_vkGetPhysicalDeviceCooperativeMatrixPropertiesNV = unsafe extern "system" fn(
    physical_device: PhysicalDevice,
    p_property_count: *mut u32,
    p_properties: *mut CooperativeMatrixPropertiesNV,
)
    -> Result;
