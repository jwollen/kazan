#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{c_char, c_int, c_void};
use core::marker::PhantomData;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceMaintenance9FeaturesKHR<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub maintenance9: Bool32,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for PhysicalDeviceMaintenance9FeaturesKHR<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::PHYSICAL_DEVICE_MAINTENANCE_9_FEATURES_KHR,
            p_next: core::ptr::null_mut(),
            maintenance9: Default::default(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceMaintenance9PropertiesKHR<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub image2_d_view_of3_d_sparse: Bool32,
    pub default_vertex_attribute_value: DefaultVertexAttributeValueKHR,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for PhysicalDeviceMaintenance9PropertiesKHR<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::PHYSICAL_DEVICE_MAINTENANCE_9_PROPERTIES_KHR,
            p_next: core::ptr::null_mut(),
            image2_d_view_of3_d_sparse: Default::default(),
            default_vertex_attribute_value: Default::default(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct QueueFamilyOwnershipTransferPropertiesKHR<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub optimal_image_transfer_to_queue_families: u32,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for QueueFamilyOwnershipTransferPropertiesKHR<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::QUEUE_FAMILY_OWNERSHIP_TRANSFER_PROPERTIES_KHR,
            p_next: core::ptr::null_mut(),
            optimal_image_transfer_to_queue_families: Default::default(),
            _marker: PhantomData,
        }
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DefaultVertexAttributeValueKHR(i32);
impl DefaultVertexAttributeValueKHR {
    pub const ZERO_ZERO_ZERO_ZERO_KHR: Self = Self(0);
    pub const ZERO_ZERO_ZERO_ONE_KHR: Self = Self(1);
}
