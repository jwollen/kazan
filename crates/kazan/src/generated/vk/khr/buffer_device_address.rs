#![allow(unused_imports)]
use crate::*;
use core::ffi::{c_char, c_int, c_void, CStr};
use kazan_sys::{vk::*, *};
pub struct DeviceFn {
    get_buffer_device_address: PFN_vkGetBufferDeviceAddress,
    get_buffer_opaque_capture_address: PFN_vkGetBufferOpaqueCaptureAddress,
    get_device_memory_opaque_capture_address: PFN_vkGetDeviceMemoryOpaqueCaptureAddress,
}
impl DeviceFn {
    pub unsafe fn get_buffer_device_address(
        &self,
        device: Device,
        info: &BufferDeviceAddressInfo,
    ) -> DeviceAddress {
        unsafe { (self.get_buffer_device_address)(device, info) }
    }
    pub unsafe fn get_buffer_opaque_capture_address(
        &self,
        device: Device,
        info: &BufferDeviceAddressInfo,
    ) -> u64 {
        unsafe { (self.get_buffer_opaque_capture_address)(device, info) }
    }
    pub unsafe fn get_device_memory_opaque_capture_address(
        &self,
        device: Device,
        info: &DeviceMemoryOpaqueCaptureAddressInfo,
    ) -> u64 {
        unsafe { (self.get_device_memory_opaque_capture_address)(device, info) }
    }
}
