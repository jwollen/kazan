#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{c_char, c_int, c_void};
pub type PhysicalDeviceBufferAddressFeaturesEXT = PhysicalDeviceBufferDeviceAddressFeaturesEXT;
pub type BufferDeviceAddressInfoEXT = BufferDeviceAddressInfo;
pub type PFN_vkGetBufferDeviceAddressEXT = PFN_vkGetBufferDeviceAddress;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceBufferDeviceAddressFeaturesEXT {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub buffer_device_address: Bool32,
    pub buffer_device_address_capture_replay: Bool32,
    pub buffer_device_address_multi_device: Bool32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct BufferDeviceAddressCreateInfoEXT {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub device_address: DeviceAddress,
}
