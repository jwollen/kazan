#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{c_char, c_int, c_void};
use core::marker::PhantomData;
#[repr(C)]
pub struct PhysicalDeviceFaultFeaturesEXT<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub device_fault: Bool32,
    pub device_fault_vendor_binary: Bool32,
    pub _marker: PhantomData<&'a ()>,
}
#[repr(C)]
pub struct DeviceFaultAddressInfoEXT {
    pub address_type: DeviceFaultAddressTypeEXT,
    pub reported_address: DeviceAddress,
    pub address_precision: DeviceSize,
}
#[repr(C)]
pub struct DeviceFaultVendorInfoEXT {
    pub description: [c_char; MAX_DESCRIPTION_SIZE as usize],
    pub vendor_fault_code: u64,
    pub vendor_fault_data: u64,
}
#[repr(C)]
pub struct DeviceFaultCountsEXT<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub address_info_count: u32,
    pub vendor_info_count: u32,
    pub vendor_binary_size: DeviceSize,
    pub _marker: PhantomData<&'a ()>,
}
#[repr(C)]
pub struct DeviceFaultInfoEXT<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub description: [c_char; MAX_DESCRIPTION_SIZE as usize],
    pub p_address_infos: *mut DeviceFaultAddressInfoEXT,
    pub p_vendor_infos: *mut DeviceFaultVendorInfoEXT,
    pub p_vendor_binary_data: *mut c_void,
    pub _marker: PhantomData<&'a ()>,
}
#[repr(C)]
pub struct DeviceFaultVendorBinaryHeaderVersionOneEXT {
    pub header_size: u32,
    pub header_version: DeviceFaultVendorBinaryHeaderVersionEXT,
    pub vendor_id: u32,
    pub device_id: u32,
    pub driver_version: u32,
    pub pipeline_cache_uuid: [u8; UUID_SIZE as usize],
    pub application_name_offset: u32,
    pub application_version: u32,
    pub engine_name_offset: u32,
    pub engine_version: u32,
    pub api_version: u32,
}
#[repr(transparent)]
#[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DeviceFaultAddressTypeEXT(i32);
impl DeviceFaultAddressTypeEXT {
    pub const NONE_EXT: Self = Self(0);
    pub const READ_INVALID_EXT: Self = Self(1);
    pub const WRITE_INVALID_EXT: Self = Self(2);
    pub const EXECUTE_INVALID_EXT: Self = Self(3);
    pub const INSTRUCTION_POINTER_UNKNOWN_EXT: Self = Self(4);
    pub const INSTRUCTION_POINTER_INVALID_EXT: Self = Self(5);
    pub const INSTRUCTION_POINTER_FAULT_EXT: Self = Self(6);
}
#[repr(transparent)]
#[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DeviceFaultVendorBinaryHeaderVersionEXT(i32);
impl DeviceFaultVendorBinaryHeaderVersionEXT {
    pub const ONE_EXT: Self = Self(1);
}
pub type PFN_vkGetDeviceFaultInfoEXT = unsafe extern "system" fn(
    device: Device,
    p_fault_counts: *mut DeviceFaultCountsEXT<'_>,
    p_fault_info: *mut DeviceFaultInfoEXT<'_>,
) -> Result;
