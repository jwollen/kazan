#![allow(unused_imports)]
use crate::*;
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
use kazan_sys::{vk::*, *};
pub struct InstanceFn {
    get_physical_device_calibrateable_time_domains_ext:
        PFN_vkGetPhysicalDeviceCalibrateableTimeDomainsKHR,
}
impl InstanceFn {
    pub unsafe fn load(
        load: impl Fn(&CStr) -> Option<PFN_vkVoidFunction>,
    ) -> core::result::Result<Self, LoadingError> {
        unsafe {
            Ok(Self {
                get_physical_device_calibrateable_time_domains_ext: transmute(
                    load(c"vkGetPhysicalDeviceCalibrateableTimeDomainsEXT").ok_or(LoadingError)?,
                ),
            })
        }
    }
}
impl InstanceFn {
    pub unsafe fn get_physical_device_calibrateable_time_domains_ext(
        &self,
        physical_device: PhysicalDevice,
        time_domains: impl ExtendUninit<TimeDomainKHR>,
    ) -> crate::Result<()> {
        unsafe {
            try_extend_uninit(time_domains, |time_domain_count, time_domains| {
                result((self.get_physical_device_calibrateable_time_domains_ext)(
                    physical_device,
                    time_domain_count,
                    time_domains as _,
                ))
            })
        }
    }
}
pub struct DeviceFn {
    get_calibrated_timestamps_ext: PFN_vkGetCalibratedTimestampsKHR,
}
impl DeviceFn {
    pub unsafe fn load(
        load: impl Fn(&CStr) -> Option<PFN_vkVoidFunction>,
    ) -> core::result::Result<Self, LoadingError> {
        unsafe {
            Ok(Self {
                get_calibrated_timestamps_ext: transmute(
                    load(c"vkGetCalibratedTimestampsEXT").ok_or(LoadingError)?,
                ),
            })
        }
    }
}
impl DeviceFn {
    pub unsafe fn get_calibrated_timestamps_ext(
        &self,
        device: Device,
        timestamp_infos: &[CalibratedTimestampInfoKHR],
        timestamps: &mut [u64],
        max_deviation: &mut u64,
    ) -> crate::Result<()> {
        unsafe {
            result((self.get_calibrated_timestamps_ext)(
                device,
                timestamp_infos.len().try_into().unwrap(),
                timestamp_infos.as_ptr() as _,
                timestamps.as_mut_ptr() as _,
                max_deviation,
            ))
        }
    }
}
