#![allow(unused_imports)]
use crate::*;
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
use kazan_sys::{vk::Result as VkResult, vk::*, *};
pub struct InstanceFn {
    enumerate_physical_device_queue_family_performance_query_counters_khr:
        PFN_vkEnumeratePhysicalDeviceQueueFamilyPerformanceQueryCountersKHR,
    get_physical_device_queue_family_performance_query_passes_khr:
        PFN_vkGetPhysicalDeviceQueueFamilyPerformanceQueryPassesKHR,
}
impl InstanceFn {
    pub unsafe fn load(
        load: impl Fn(&CStr) -> Option<PFN_vkVoidFunction>,
    ) -> core::result::Result<Self, LoadingError> {
        unsafe {
            Ok(Self {
                enumerate_physical_device_queue_family_performance_query_counters_khr: transmute(
                    load(c"vkEnumeratePhysicalDeviceQueueFamilyPerformanceQueryCountersKHR")
                        .ok_or(LoadingError)?,
                ),
                get_physical_device_queue_family_performance_query_passes_khr: transmute(
                    load(c"vkGetPhysicalDeviceQueueFamilyPerformanceQueryPassesKHR")
                        .ok_or(LoadingError)?,
                ),
            })
        }
    }
}
impl InstanceFn {
    pub unsafe fn enumerate_physical_device_queue_family_performance_query_counters_khr(
        &self,
        physical_device: PhysicalDevice,
        queue_family_index: u32,
        counters: impl ExtendUninit<PerformanceCounterKHR>,
        counter_descriptions: impl ExtendUninit<PerformanceCounterDescriptionKHR>,
    ) -> crate::Result<()> {
        unsafe {
            try_extend_uninit2(
                counters,
                counter_descriptions,
                |counter_count, counters, counter_descriptions| {
                    let result = (self
                        .enumerate_physical_device_queue_family_performance_query_counters_khr)(
                        physical_device,
                        queue_family_index,
                        counter_count,
                        counters as _,
                        counter_descriptions as _,
                    );

                    match result {
                        VkResult::SUCCESS => Ok(()),
                        VkResult::INCOMPLETE => Ok(()),
                        err => Err(err),
                    }
                },
            )
        }
    }
    pub unsafe fn get_physical_device_queue_family_performance_query_passes_khr(
        &self,
        physical_device: PhysicalDevice,
        performance_query_create_info: &QueryPoolPerformanceCreateInfoKHR,
    ) -> u32 {
        unsafe {
            let mut num_passes = core::mem::MaybeUninit::uninit();
            (self.get_physical_device_queue_family_performance_query_passes_khr)(
                physical_device,
                performance_query_create_info,
                num_passes.as_mut_ptr(),
            );
            num_passes.assume_init()
        }
    }
}
pub struct DeviceFn {
    acquire_profiling_lock_khr: PFN_vkAcquireProfilingLockKHR,
    release_profiling_lock_khr: PFN_vkReleaseProfilingLockKHR,
}
impl DeviceFn {
    pub unsafe fn load(
        load: impl Fn(&CStr) -> Option<PFN_vkVoidFunction>,
    ) -> core::result::Result<Self, LoadingError> {
        unsafe {
            Ok(Self {
                acquire_profiling_lock_khr: transmute(
                    load(c"vkAcquireProfilingLockKHR").ok_or(LoadingError)?,
                ),
                release_profiling_lock_khr: transmute(
                    load(c"vkReleaseProfilingLockKHR").ok_or(LoadingError)?,
                ),
            })
        }
    }
}
impl DeviceFn {
    pub unsafe fn acquire_profiling_lock_khr(
        &self,
        device: Device,
        info: &AcquireProfilingLockInfoKHR,
    ) -> crate::Result<()> {
        unsafe {
            let result = (self.acquire_profiling_lock_khr)(device, info);

            match result {
                VkResult::SUCCESS => Ok(()),
                err => Err(err),
            }
        }
    }
    pub unsafe fn release_profiling_lock_khr(&self, device: Device) {
        unsafe { (self.release_profiling_lock_khr)(device) }
    }
}
