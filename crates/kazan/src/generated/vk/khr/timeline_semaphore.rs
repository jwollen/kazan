#![allow(unused_imports)]
use crate::*;
use core::ffi::{c_char, c_int, c_void, CStr};
use kazan_sys::{vk::*, *};
pub struct DeviceFn {
    get_semaphore_counter_value: PFN_vkGetSemaphoreCounterValue,
    wait_semaphores: PFN_vkWaitSemaphores,
    signal_semaphore: PFN_vkSignalSemaphore,
}
impl DeviceFn {
    pub unsafe fn get_semaphore_counter_value_khr(
        &self,
        device: Device,
        semaphore: Semaphore,
        value: &mut u64,
    ) -> Result {
        unsafe { (self.get_semaphore_counter_value)(device, semaphore, value) }
    }
    pub unsafe fn wait_semaphores_khr(
        &self,
        device: Device,
        wait_info: &SemaphoreWaitInfo,
        timeout: u64,
    ) -> Result {
        unsafe { (self.wait_semaphores)(device, wait_info, timeout) }
    }
    pub unsafe fn signal_semaphore_khr(
        &self,
        device: Device,
        signal_info: &SemaphoreSignalInfo,
    ) -> Result {
        unsafe { (self.signal_semaphore)(device, signal_info) }
    }
}
