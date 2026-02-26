#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{c_char, c_int, c_void};
use core::marker::PhantomData;
#[repr(C)]
pub struct PhysicalDeviceMaintenance9FeaturesKHR<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub maintenance9: Bool32,
    pub _marker: PhantomData<&'a ()>,
}
#[repr(C)]
pub struct PhysicalDeviceMaintenance9PropertiesKHR<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub image2_d_view_of3_d_sparse: Bool32,
    pub default_vertex_attribute_value: DefaultVertexAttributeValueKHR,
    pub _marker: PhantomData<&'a ()>,
}
#[repr(C)]
pub struct QueueFamilyOwnershipTransferPropertiesKHR<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub optimal_image_transfer_to_queue_families: u32,
    pub _marker: PhantomData<&'a ()>,
}
#[repr(transparent)]
#[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DefaultVertexAttributeValueKHR(i32);
impl DefaultVertexAttributeValueKHR {
    pub const ZERO_ZERO_ZERO_ZERO_KHR: Self = Self(0);
    pub const ZERO_ZERO_ZERO_ONE_KHR: Self = Self(1);
}
