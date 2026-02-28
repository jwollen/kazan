#![allow(unused_imports)]
use crate::{vk::Result as VkResult, vk::*, *};
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
pub(super) mod defs {
    #![allow(non_camel_case_types, unused_imports)]
    use crate::{vk::*, *};
    use bitflags::bitflags;
    use core::ffi::{CStr, c_char, c_int, c_void};
    use core::marker::PhantomData;
    pub type SemaphoreTypeKHR = SemaphoreType;
    pub type PhysicalDeviceTimelineSemaphoreFeaturesKHR<'a> =
        PhysicalDeviceTimelineSemaphoreFeatures<'a>;
    pub type PhysicalDeviceTimelineSemaphorePropertiesKHR<'a> =
        PhysicalDeviceTimelineSemaphoreProperties<'a>;
    pub type SemaphoreTypeCreateInfoKHR<'a> = SemaphoreTypeCreateInfo<'a>;
    pub type TimelineSemaphoreSubmitInfoKHR<'a> = TimelineSemaphoreSubmitInfo<'a>;
    pub type SemaphoreWaitInfoKHR<'a> = SemaphoreWaitInfo<'a>;
    pub type SemaphoreSignalInfoKHR<'a> = SemaphoreSignalInfo<'a>;
    pub type SemaphoreWaitFlagsKHR = SemaphoreWaitFlags;
    pub type PFN_vkGetSemaphoreCounterValueKHR = PFN_vkGetSemaphoreCounterValue;
    pub type PFN_vkWaitSemaphoresKHR = PFN_vkWaitSemaphores;
    pub type PFN_vkSignalSemaphoreKHR = PFN_vkSignalSemaphore;
}
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
        wait_info: &SemaphoreWaitInfo<'_>,
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
        signal_info: &SemaphoreSignalInfo<'_>,
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
