#![allow(unused_imports)]
use crate::*;
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
use kazan_sys::{vk::*, *};
pub struct InstanceFn {
    enumerate_physical_device_queue_family_performance_counters_by_region_arm:
        PFN_vkEnumeratePhysicalDeviceQueueFamilyPerformanceCountersByRegionARM,
}
impl InstanceFn {
    pub unsafe fn load(
        load: impl Fn(&CStr) -> Option<PFN_vkVoidFunction>,
    ) -> core::result::Result<Self, LoadingError> {
        unsafe {
            Ok(Self {
                enumerate_physical_device_queue_family_performance_counters_by_region_arm:
                    transmute(
                        load(c"vkEnumeratePhysicalDeviceQueueFamilyPerformanceCountersByRegionARM")
                            .ok_or(LoadingError)?,
                    ),
            })
        }
    }
}
impl InstanceFn {
    pub unsafe fn enumerate_physical_device_queue_family_performance_counters_by_region_arm(
        &self,
        physical_device: PhysicalDevice,
        queue_family_index: u32,
        counters: impl ExtendUninit<PerformanceCounterARM>,
        counter_descriptions: impl ExtendUninit<PerformanceCounterDescriptionARM>,
    ) -> crate::Result<()> {
        unsafe {
            try_extend_uninit2(
                counters,
                counter_descriptions,
                |counter_count, counters, counter_descriptions| {
                    result(
(self.enumerate_physical_device_queue_family_performance_counters_by_region_arm)(
physical_device,
queue_family_index,
counter_count,
counters as _,
counter_descriptions as _,
)
)
                },
            )
        }
    }
}
