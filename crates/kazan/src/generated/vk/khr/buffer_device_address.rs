#![allow(unused_imports)]
use crate::{vk::Result as VkResult, vk::*, *};
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
pub(super) mod defs {
    #![allow(non_camel_case_types, unused_imports)]
    use crate::{vk::*, *};
    use core::ffi::{CStr, c_char, c_int, c_void};
    use core::fmt;
    use core::marker::PhantomData;
    pub type PhysicalDeviceBufferDeviceAddressFeaturesKHR<'a> =
        PhysicalDeviceBufferDeviceAddressFeatures<'a>;
    pub type BufferDeviceAddressInfoKHR<'a> = BufferDeviceAddressInfo<'a>;
    pub type BufferOpaqueCaptureAddressCreateInfoKHR<'a> = BufferOpaqueCaptureAddressCreateInfo<'a>;
    pub type MemoryOpaqueCaptureAddressAllocateInfoKHR<'a> =
        MemoryOpaqueCaptureAddressAllocateInfo<'a>;
    pub type DeviceMemoryOpaqueCaptureAddressInfoKHR<'a> = DeviceMemoryOpaqueCaptureAddressInfo<'a>;
    pub type PFN_vkGetBufferOpaqueCaptureAddressKHR = PFN_vkGetBufferOpaqueCaptureAddress;
    pub type PFN_vkGetBufferDeviceAddressKHR = PFN_vkGetBufferDeviceAddress;
    pub type PFN_vkGetDeviceMemoryOpaqueCaptureAddressKHR =
        PFN_vkGetDeviceMemoryOpaqueCaptureAddress;
}
pub struct DeviceFn {
    get_buffer_device_address_khr: PFN_vkGetBufferDeviceAddress,
    get_buffer_opaque_capture_address_khr: PFN_vkGetBufferOpaqueCaptureAddress,
    get_device_memory_opaque_capture_address_khr: PFN_vkGetDeviceMemoryOpaqueCaptureAddress,
}
impl DeviceFn {
    pub unsafe fn load(
        load: impl Fn(&CStr) -> Option<PFN_vkVoidFunction>,
    ) -> core::result::Result<Self, MissingEntryPointError> {
        unsafe {
            Ok(Self {
                get_buffer_device_address_khr: transmute(
                    load(c"vkGetBufferDeviceAddressKHR").ok_or(MissingEntryPointError)?,
                ),
                get_buffer_opaque_capture_address_khr: transmute(
                    load(c"vkGetBufferOpaqueCaptureAddressKHR").ok_or(MissingEntryPointError)?,
                ),
                get_device_memory_opaque_capture_address_khr: transmute(
                    load(c"vkGetDeviceMemoryOpaqueCaptureAddressKHR")
                        .ok_or(MissingEntryPointError)?,
                ),
            })
        }
    }
}
impl DeviceFn {
    pub unsafe fn get_buffer_device_address_khr(
        &self,
        device: Device,
        info: &BufferDeviceAddressInfo<'_>,
    ) -> DeviceAddress {
        unsafe { (self.get_buffer_device_address_khr)(device, info) }
    }
    pub unsafe fn get_buffer_opaque_capture_address_khr(
        &self,
        device: Device,
        info: &BufferDeviceAddressInfo<'_>,
    ) -> u64 {
        unsafe { (self.get_buffer_opaque_capture_address_khr)(device, info) }
    }
    pub unsafe fn get_device_memory_opaque_capture_address_khr(
        &self,
        device: Device,
        info: &DeviceMemoryOpaqueCaptureAddressInfo<'_>,
    ) -> u64 {
        unsafe { (self.get_device_memory_opaque_capture_address_khr)(device, info) }
    }
}
