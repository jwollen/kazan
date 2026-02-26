#![allow(unused_imports)]
use crate::*;
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
use kazan_sys::{vk::Result as VkResult, vk::*, *};
pub struct DeviceFn {
    get_buffer_device_address_khr: PFN_vkGetBufferDeviceAddress,
    get_buffer_opaque_capture_address_khr: PFN_vkGetBufferOpaqueCaptureAddress,
    get_device_memory_opaque_capture_address_khr: PFN_vkGetDeviceMemoryOpaqueCaptureAddress,
}
impl DeviceFn {
    pub unsafe fn load(
        load: impl Fn(&CStr) -> Option<PFN_vkVoidFunction>,
    ) -> core::result::Result<Self, LoadingError> {
        unsafe {
            Ok(Self {
                get_buffer_device_address_khr: transmute(
                    load(c"vkGetBufferDeviceAddressKHR").ok_or(LoadingError)?,
                ),
                get_buffer_opaque_capture_address_khr: transmute(
                    load(c"vkGetBufferOpaqueCaptureAddressKHR").ok_or(LoadingError)?,
                ),
                get_device_memory_opaque_capture_address_khr: transmute(
                    load(c"vkGetDeviceMemoryOpaqueCaptureAddressKHR").ok_or(LoadingError)?,
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
