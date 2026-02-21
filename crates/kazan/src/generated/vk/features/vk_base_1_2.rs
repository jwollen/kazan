#![allow(unused_imports)]
use crate::*;
use core::ffi::{c_char, c_int, c_void, CStr};
use kazan_sys::{vk::*, *};
pub struct DeviceFn {
    reset_query_pool: PFN_vkResetQueryPool,
    get_semaphore_counter_value: PFN_vkGetSemaphoreCounterValue,
    wait_semaphores: PFN_vkWaitSemaphores,
    signal_semaphore: PFN_vkSignalSemaphore,
    get_buffer_device_address: PFN_vkGetBufferDeviceAddress,
    get_buffer_opaque_capture_address: PFN_vkGetBufferOpaqueCaptureAddress,
    get_device_memory_opaque_capture_address: PFN_vkGetDeviceMemoryOpaqueCaptureAddress,
}
impl DeviceFn {
    pub unsafe fn reset_query_pool(
        &self,
        device: Device,
        query_pool: QueryPool,
        first_query: u32,
        query_count: u32,
    ) {
        unsafe { (self.reset_query_pool)(device, query_pool, first_query, query_count) }
    }
    pub unsafe fn get_semaphore_counter_value(
        &self,
        device: Device,
        semaphore: Semaphore,
        value: &mut u64,
    ) -> Result {
        unsafe { (self.get_semaphore_counter_value)(device, semaphore, value) }
    }
    pub unsafe fn wait_semaphores(
        &self,
        device: Device,
        wait_info: &SemaphoreWaitInfo,
        timeout: u64,
    ) -> Result {
        unsafe { (self.wait_semaphores)(device, wait_info, timeout) }
    }
    pub unsafe fn signal_semaphore(
        &self,
        device: Device,
        signal_info: &SemaphoreSignalInfo,
    ) -> Result {
        unsafe { (self.signal_semaphore)(device, signal_info) }
    }
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
