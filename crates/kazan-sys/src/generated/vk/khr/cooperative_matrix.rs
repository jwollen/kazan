#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{c_char, c_int, c_void};
use core::marker::PhantomData;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceCooperativeMatrixFeaturesKHR<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub cooperative_matrix: Bool32,
    pub cooperative_matrix_robust_buffer_access: Bool32,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for PhysicalDeviceCooperativeMatrixFeaturesKHR<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::PHYSICAL_DEVICE_COOPERATIVE_MATRIX_FEATURES_KHR,
            p_next: core::ptr::null_mut(),
            cooperative_matrix: Default::default(),
            cooperative_matrix_robust_buffer_access: Default::default(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct CooperativeMatrixPropertiesKHR<'a> {
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
    pub _marker: PhantomData<&'a ()>,
}
impl Default for CooperativeMatrixPropertiesKHR<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::COOPERATIVE_MATRIX_PROPERTIES_KHR,
            p_next: core::ptr::null_mut(),
            m_size: Default::default(),
            n_size: Default::default(),
            k_size: Default::default(),
            a_type: Default::default(),
            b_type: Default::default(),
            c_type: Default::default(),
            result_type: Default::default(),
            saturating_accumulation: Default::default(),
            scope: Default::default(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceCooperativeMatrixPropertiesKHR<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub cooperative_matrix_supported_stages: ShaderStageFlags,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for PhysicalDeviceCooperativeMatrixPropertiesKHR<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::PHYSICAL_DEVICE_COOPERATIVE_MATRIX_PROPERTIES_KHR,
            p_next: core::ptr::null_mut(),
            cooperative_matrix_supported_stages: Default::default(),
            _marker: PhantomData,
        }
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ScopeKHR(i32);
impl ScopeKHR {
    pub const DEVICE_KHR: Self = Self(1);
    pub const WORKGROUP_KHR: Self = Self(2);
    pub const SUBGROUP_KHR: Self = Self(3);
    pub const QUEUE_FAMILY_KHR: Self = Self(5);
    pub const DEVICE_NV: Self = Self::DEVICE_KHR;
    pub const QUEUE_FAMILY_NV: Self = Self::QUEUE_FAMILY_KHR;
    pub const SUBGROUP_NV: Self = Self::SUBGROUP_KHR;
    pub const WORKGROUP_NV: Self = Self::WORKGROUP_KHR;
}
pub type PFN_vkGetPhysicalDeviceCooperativeMatrixPropertiesKHR =
    unsafe extern "system" fn(
        physical_device: PhysicalDevice,
        p_property_count: *mut u32,
        p_properties: *mut CooperativeMatrixPropertiesKHR<'_>,
    ) -> Result;
