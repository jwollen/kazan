#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{c_char, c_int, c_void};
use core::marker::PhantomData;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceExtendedSparseAddressSpaceFeaturesNV<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub extended_sparse_address_space: Bool32,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for PhysicalDeviceExtendedSparseAddressSpaceFeaturesNV<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::PHYSICAL_DEVICE_EXTENDED_SPARSE_ADDRESS_SPACE_FEATURES_NV,
            p_next: core::ptr::null_mut(),
            extended_sparse_address_space: Default::default(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceExtendedSparseAddressSpacePropertiesNV<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub extended_sparse_address_space_size: DeviceSize,
    pub extended_sparse_image_usage_flags: ImageUsageFlags,
    pub extended_sparse_buffer_usage_flags: BufferUsageFlags,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for PhysicalDeviceExtendedSparseAddressSpacePropertiesNV<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::PHYSICAL_DEVICE_EXTENDED_SPARSE_ADDRESS_SPACE_PROPERTIES_NV,
            p_next: core::ptr::null_mut(),
            extended_sparse_address_space_size: Default::default(),
            extended_sparse_image_usage_flags: Default::default(),
            extended_sparse_buffer_usage_flags: Default::default(),
            _marker: PhantomData,
        }
    }
}
