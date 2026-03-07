#![allow(unused_imports)]
use crate::{vk::Result as VkResult, vk::*, *};
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;

pub const EXTENSION_NAME: &CStr = c"VK_EXT_calibrated_timestamps";

pub(super) mod defs {
    #![allow(non_camel_case_types, unused_imports)]
    use crate::{vk::*, *};
    use core::ffi::{CStr, c_char, c_int, c_void};
    use core::fmt;
    use core::marker::PhantomData;

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkTimeDomainEXT.html>
    pub type TimeDomainEXT = TimeDomainKHR;
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkCalibratedTimestampInfoEXT.html>
    pub type CalibratedTimestampInfoEXT<'a> = CalibratedTimestampInfoKHR<'a>;
    pub type PFN_vkGetPhysicalDeviceCalibrateableTimeDomainsEXT =
        PFN_vkGetPhysicalDeviceCalibrateableTimeDomainsKHR;
    pub type PFN_vkGetCalibratedTimestampsEXT = PFN_vkGetCalibratedTimestampsKHR;
}

pub struct InstanceFn {
    get_physical_device_calibrateable_time_domains_ext:
        PFN_vkGetPhysicalDeviceCalibrateableTimeDomainsKHR,
}

impl InstanceFn {
    pub unsafe fn load(
        load: impl Fn(&CStr) -> Option<PFN_vkVoidFunction>,
    ) -> core::result::Result<Self, MissingEntryPointError> {
        unsafe {
            Ok(Self {
                get_physical_device_calibrateable_time_domains_ext: transmute(
                    load(c"vkGetPhysicalDeviceCalibrateableTimeDomainsEXT")
                        .ok_or(MissingEntryPointError)?,
                ),
            })
        }
    }
}

impl InstanceFn {
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkGetPhysicalDeviceCalibrateableTimeDomainsEXT.html>
    pub unsafe fn get_physical_device_calibrateable_time_domains_ext(
        &self,
        physical_device: PhysicalDevice,
        mut time_domains: impl ExtendUninit<TimeDomainKHR>,
    ) -> crate::Result<()> {
        unsafe {
            let call = |time_domain_count, time_domains| {
                let result = (self.get_physical_device_calibrateable_time_domains_ext)(
                    physical_device,
                    time_domain_count,
                    time_domains as _,
                );

                match result {
                    VkResult::SUCCESS => Ok(()),
                    VkResult::INCOMPLETE => Ok(()),
                    err => Err(err),
                }
            };
            let mut len = 0;
            call(&mut len, std::ptr::null_mut())?;
            let capacity = len.try_into().expect("failed to convert `N` to usize");
            let time_domains_buf = time_domains.reserve(capacity);
            len = time_domains_buf.len().try_into().unwrap();
            let result = call(&mut len, time_domains_buf.as_mut_ptr() as *mut _)?;
            time_domains.set_len(len.try_into().unwrap());
            Ok(result)
        }
    }
}

pub struct DeviceFn {
    get_calibrated_timestamps_ext: PFN_vkGetCalibratedTimestampsKHR,
}

impl DeviceFn {
    pub unsafe fn load(
        load: impl Fn(&CStr) -> Option<PFN_vkVoidFunction>,
    ) -> core::result::Result<Self, MissingEntryPointError> {
        unsafe {
            Ok(Self {
                get_calibrated_timestamps_ext: transmute(
                    load(c"vkGetCalibratedTimestampsEXT").ok_or(MissingEntryPointError)?,
                ),
            })
        }
    }
}

impl DeviceFn {
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkGetCalibratedTimestampsEXT.html>
    pub unsafe fn get_calibrated_timestamps_ext(
        &self,
        device: Device,
        timestamp_infos: &[CalibratedTimestampInfoKHR<'_>],
        timestamps: &mut [u64],
    ) -> crate::Result<u64> {
        unsafe {
            let mut max_deviation = core::mem::MaybeUninit::uninit();
            let result = (self.get_calibrated_timestamps_ext)(
                device,
                timestamp_infos.len().try_into().unwrap(),
                timestamp_infos.as_ptr() as _,
                timestamps.as_mut_ptr() as _,
                max_deviation.as_mut_ptr(),
            );

            match result {
                VkResult::SUCCESS => Ok(max_deviation.assume_init()),
                err => Err(err),
            }
        }
    }
}
