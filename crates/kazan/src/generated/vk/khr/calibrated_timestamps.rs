#![allow(unused_imports)]
use crate::*;
use kazan_sys::{vk::*, *};
use std::ffi::{c_char, c_int, c_void, CStr};
pub struct InstanceFn {
    get_physical_device_calibrateable_time_domains_khr:
        PFN_vkGetPhysicalDeviceCalibrateableTimeDomainsKHR,
}
impl InstanceFn {
    pub unsafe fn get_physical_device_calibrateable_time_domains_khr(
        &self,
        physical_device: PhysicalDevice,
        time_domains: impl ExtendUninit<TimeDomainKHR>,
    ) -> Result {
        unsafe {
            try_extend_uninit(time_domains, |time_domain_count, time_domains| {
                (self.get_physical_device_calibrateable_time_domains_khr)(
                    physical_device,
                    time_domain_count,
                    time_domains as _,
                )
            })
        }
    }
}
pub struct DeviceFn {
    get_calibrated_timestamps_khr: PFN_vkGetCalibratedTimestampsKHR,
}
impl DeviceFn {
    pub unsafe fn get_calibrated_timestamps_khr(
        &self,
        device: Device,
        timestamp_infos: &[CalibratedTimestampInfoKHR],
        timestamps: &mut [u64],
        max_deviation: &mut u64,
    ) -> Result {
        unsafe {
            (self.get_calibrated_timestamps_khr)(
                device,
                timestamp_infos.len().try_into().unwrap(),
                timestamp_infos.as_ptr() as _,
                timestamps.as_mut_ptr() as _,
                max_deviation,
            )
        }
    }
}
