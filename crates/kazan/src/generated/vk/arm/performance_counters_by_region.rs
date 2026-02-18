#![allow(unused_imports)]
use crate::*;
use kazan_sys::{vk::*, *};
use std::ffi::{c_char, c_int, c_void, CStr};
pub struct InstanceFn {
    enumerate_physical_device_queue_family_performance_counters_by_region_arm:
        PFN_vkEnumeratePhysicalDeviceQueueFamilyPerformanceCountersByRegionARM,
}
impl InstanceFn {
    pub unsafe fn enumerate_physical_device_queue_family_performance_counters_by_region_arm(
        &self,
        physical_device: PhysicalDevice,
        queue_family_index: u32,
        counters: impl ExtendUninit<PerformanceCounterARM>,
        counter_descriptions: impl ExtendUninit<PerformanceCounterDescriptionARM>,
    ) -> Result {
        unsafe {
            try_extend_uninit2(
                counters,
                counter_descriptions,
                |counter_count, counters, counter_descriptions| {
                    (self.enumerate_physical_device_queue_family_performance_counters_by_region_arm)(
                        physical_device,
                        queue_family_index,
                        counter_count,
                        counters as _,
                        counter_descriptions as _,
                    )
                },
            )
        }
    }
}
