#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use std::ffi::{c_char, c_int, c_void};
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceMaintenance9FeaturesKHR {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub maintenance9: Bool32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceMaintenance9PropertiesKHR {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub image2_d_view_of3_d_sparse: Bool32,
    pub default_vertex_attribute_value: DefaultVertexAttributeValueKHR,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct QueueFamilyOwnershipTransferPropertiesKHR {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub optimal_image_transfer_to_queue_families: u32,
}
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DefaultVertexAttributeValueKHR(i32);
impl DefaultVertexAttributeValueKHR {
    pub const ZERO_ZERO_ZERO_ZERO_KHR: Self = Self(0);
    pub const ZERO_ZERO_ZERO_ONE_KHR: Self = Self(1);
}
