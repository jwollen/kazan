#![allow(unused_imports)]
use crate::*;
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
use kazan_sys::{vk::*, *};
pub struct DeviceFn {
    get_semaphore_counter_value_khr: PFN_vkGetSemaphoreCounterValue,
    wait_semaphores_khr: PFN_vkWaitSemaphores,
    signal_semaphore_khr: PFN_vkSignalSemaphore,
}
impl DeviceFn {
    pub unsafe fn load(
        load: impl Fn(&CStr) -> Option<PFN_vkVoidFunction>,
    ) -> core::result::Result<Self, LoadingError> {
        unsafe {
            Ok(Self {
                get_semaphore_counter_value_khr: transmute(
                    load(c"vkGetSemaphoreCounterValueKHR").ok_or(LoadingError)?,
                ),
                wait_semaphores_khr: transmute(load(c"vkWaitSemaphoresKHR").ok_or(LoadingError)?),
                signal_semaphore_khr: transmute(load(c"vkSignalSemaphoreKHR").ok_or(LoadingError)?),
            })
        }
    }
}
impl DeviceFn {
    pub unsafe fn get_semaphore_counter_value_khr(
        &self,
        device: Device,
        semaphore: Semaphore,
        value: &mut u64,
    ) -> crate::Result<()> {
        unsafe {
            result((self.get_semaphore_counter_value_khr)(
                device, semaphore, value,
            ))
        }
    }
    pub unsafe fn wait_semaphores_khr(
        &self,
        device: Device,
        wait_info: &SemaphoreWaitInfo,
        timeout: u64,
    ) -> crate::Result<()> {
        unsafe { result((self.wait_semaphores_khr)(device, wait_info, timeout)) }
    }
    pub unsafe fn signal_semaphore_khr(
        &self,
        device: Device,
        signal_info: &SemaphoreSignalInfo,
    ) -> crate::Result<()> {
        unsafe { result((self.signal_semaphore_khr)(device, signal_info)) }
    }
}
