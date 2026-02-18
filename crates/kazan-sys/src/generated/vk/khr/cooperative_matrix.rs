#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use std::ffi::{c_char, c_int, c_void};
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceCooperativeMatrixFeaturesKHR {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub cooperative_matrix: Bool32,
    pub cooperative_matrix_robust_buffer_access: Bool32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct CooperativeMatrixPropertiesKHR {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub m_size: u32,
    pub n_size: u32,
    pub k_size: u32,
    pub a_type: ComponentTypeKHR,
    pub b_type: ComponentTypeKHR,
    pub c_type: ComponentTypeKHR,
    pub result_type: ComponentTypeKHR,
    pub saturating_accumulation: Bool32,
    pub scope: ScopeKHR,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceCooperativeMatrixPropertiesKHR {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub cooperative_matrix_supported_stages: ShaderStageFlags,
}
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ScopeKHR(i32);
impl ScopeKHR {
    pub const DEVICE_KHR: Self = Self(1);
    pub const WORKGROUP_KHR: Self = Self(2);
    pub const SUBGROUP_KHR: Self = Self(3);
    pub const QUEUE_FAMILY_KHR: Self = Self(5);
}
pub type PFN_vkGetPhysicalDeviceCooperativeMatrixPropertiesKHR =
    unsafe extern "system" fn(
        physical_device: PhysicalDevice,
        p_property_count: *mut u32,
        p_properties: *mut CooperativeMatrixPropertiesKHR,
    ) -> Result;
