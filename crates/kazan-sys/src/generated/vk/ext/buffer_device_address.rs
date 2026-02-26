#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{c_char, c_int, c_void};
use core::marker::PhantomData;
pub type PhysicalDeviceBufferAddressFeaturesEXT<'a> =
    PhysicalDeviceBufferDeviceAddressFeaturesEXT<'a>;
pub type BufferDeviceAddressInfoEXT<'a> = BufferDeviceAddressInfo<'a>;
pub type PFN_vkGetBufferDeviceAddressEXT = PFN_vkGetBufferDeviceAddress;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceBufferDeviceAddressFeaturesEXT<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub buffer_device_address: Bool32,
    pub buffer_device_address_capture_replay: Bool32,
    pub buffer_device_address_multi_device: Bool32,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for PhysicalDeviceBufferDeviceAddressFeaturesEXT<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::PHYSICAL_DEVICE_BUFFER_DEVICE_ADDRESS_FEATURES_EXT,
            p_next: core::ptr::null_mut(),
            buffer_device_address: Default::default(),
            buffer_device_address_capture_replay: Default::default(),
            buffer_device_address_multi_device: Default::default(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct BufferDeviceAddressCreateInfoEXT<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub device_address: DeviceAddress,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for BufferDeviceAddressCreateInfoEXT<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::BUFFER_DEVICE_ADDRESS_CREATE_INFO_EXT,
            p_next: core::ptr::null(),
            device_address: Default::default(),
            _marker: PhantomData,
        }
    }
}
