#![allow(unused_imports)]
use crate::*;
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
use kazan_sys::{vk::Result as VkResult, vk::*, *};
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
    ) -> crate::Result<u64> {
        unsafe {
            let mut value = core::mem::MaybeUninit::uninit();
            let result =
                (self.get_semaphore_counter_value_khr)(device, semaphore, value.as_mut_ptr());

            match result {
                VkResult::SUCCESS => Ok(value.assume_init()),
                err => Err(err),
            }
        }
    }
    pub unsafe fn wait_semaphores_khr(
        &self,
        device: Device,
        wait_info: &SemaphoreWaitInfo,
        timeout: u64,
    ) -> crate::Result<()> {
        unsafe {
            let result = (self.wait_semaphores_khr)(device, wait_info, timeout);

            match result {
                VkResult::SUCCESS => Ok(()),
                VkResult::TIMEOUT => Ok(()),
                err => Err(err),
            }
        }
    }
    pub unsafe fn signal_semaphore_khr(
        &self,
        device: Device,
        signal_info: &SemaphoreSignalInfo,
    ) -> crate::Result<()> {
        unsafe {
            let result = (self.signal_semaphore_khr)(device, signal_info);

            match result {
                VkResult::SUCCESS => Ok(()),
                err => Err(err),
            }
        }
    }
}
