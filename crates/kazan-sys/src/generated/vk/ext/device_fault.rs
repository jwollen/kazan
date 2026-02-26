#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{c_char, c_int, c_void};
use core::marker::PhantomData;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceFaultFeaturesEXT<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub device_fault: Bool32,
    pub device_fault_vendor_binary: Bool32,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for PhysicalDeviceFaultFeaturesEXT<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::PHYSICAL_DEVICE_FAULT_FEATURES_EXT,
            p_next: core::ptr::null_mut(),
            device_fault: Default::default(),
            device_fault_vendor_binary: Default::default(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct DeviceFaultAddressInfoEXT {
    pub address_type: DeviceFaultAddressTypeEXT,
    pub reported_address: DeviceAddress,
    pub address_precision: DeviceSize,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct DeviceFaultVendorInfoEXT {
    pub description: [c_char; MAX_DESCRIPTION_SIZE as usize],
    pub vendor_fault_code: u64,
    pub vendor_fault_data: u64,
}
impl Default for DeviceFaultVendorInfoEXT {
    fn default() -> Self {
        Self {
            description: [Default::default(); _],
            vendor_fault_code: Default::default(),
            vendor_fault_data: Default::default(),
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct DeviceFaultCountsEXT<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub address_info_count: u32,
    pub vendor_info_count: u32,
    pub vendor_binary_size: DeviceSize,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for DeviceFaultCountsEXT<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::DEVICE_FAULT_COUNTS_EXT,
            p_next: core::ptr::null_mut(),
            address_info_count: Default::default(),
            vendor_info_count: Default::default(),
            vendor_binary_size: Default::default(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct DeviceFaultInfoEXT<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub description: [c_char; MAX_DESCRIPTION_SIZE as usize],
    pub p_address_infos: *mut DeviceFaultAddressInfoEXT,
    pub p_vendor_infos: *mut DeviceFaultVendorInfoEXT,
    pub p_vendor_binary_data: *mut c_void,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for DeviceFaultInfoEXT<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::DEVICE_FAULT_INFO_EXT,
            p_next: core::ptr::null_mut(),
            description: [Default::default(); _],
            p_address_infos: core::ptr::null_mut(),
            p_vendor_infos: core::ptr::null_mut(),
            p_vendor_binary_data: core::ptr::null_mut(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
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
impl Default for DeviceFaultVendorBinaryHeaderVersionOneEXT {
    fn default() -> Self {
        Self {
            header_size: Default::default(),
            header_version: Default::default(),
            vendor_id: Default::default(),
            device_id: Default::default(),
            driver_version: Default::default(),
            pipeline_cache_uuid: [Default::default(); _],
            application_name_offset: Default::default(),
            application_version: Default::default(),
            engine_name_offset: Default::default(),
            engine_version: Default::default(),
            api_version: Default::default(),
        }
    }
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
