#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{c_char, c_int, c_void};
#[repr(C)]
pub struct PhysicalDeviceExtendedSparseAddressSpaceFeaturesNV {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub extended_sparse_address_space: Bool32,
}
#[repr(C)]
pub struct PhysicalDeviceExtendedSparseAddressSpacePropertiesNV {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub extended_sparse_address_space_size: DeviceSize,
    pub extended_sparse_image_usage_flags: ImageUsageFlags,
    pub extended_sparse_buffer_usage_flags: BufferUsageFlags,
}
