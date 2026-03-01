#![allow(unused_imports)]
use crate::{vk::Result as VkResult, vk::*, *};
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
pub(super) mod defs {
    #![allow(non_camel_case_types, unused_imports)]
    use crate::{vk::*, *};
    use core::ffi::{CStr, c_char, c_int, c_void};
    use core::fmt;
    use core::marker::PhantomData;
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct CalibratedTimestampInfoKHR<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub time_domain: TimeDomainKHR,
        pub _marker: PhantomData<&'a ()>,
    }
    unsafe impl<'a> TaggedStructure<'a> for CalibratedTimestampInfoKHR<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::CALIBRATED_TIMESTAMP_INFO_KHR;
    }
    impl Default for CalibratedTimestampInfoKHR<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null(),
                time_domain: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> CalibratedTimestampInfoKHR<'a> {
        pub fn time_domain(mut self, time_domain: TimeDomainKHR) -> Self {
            self.time_domain = time_domain;
            self
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct TimeDomainKHR(i32);
    impl TimeDomainKHR {
        pub const DEVICE_KHR: Self = Self(0);
        pub const CLOCK_MONOTONIC_KHR: Self = Self(1);
        pub const CLOCK_MONOTONIC_RAW_KHR: Self = Self(2);
        pub const QUERY_PERFORMANCE_COUNTER_KHR: Self = Self(3);
        pub const PRESENT_STAGE_LOCAL_EXT: Self = Self(1000208000);
        pub const SWAPCHAIN_LOCAL_EXT: Self = Self(1000208001);
        pub const CLOCK_MONOTONIC_EXT: Self = Self::CLOCK_MONOTONIC_KHR;
        pub const CLOCK_MONOTONIC_RAW_EXT: Self = Self::CLOCK_MONOTONIC_RAW_KHR;
        pub const DEVICE_EXT: Self = Self::DEVICE_KHR;
        pub const QUERY_PERFORMANCE_COUNTER_EXT: Self = Self::QUERY_PERFORMANCE_COUNTER_KHR;
    }
    pub type PFN_vkGetPhysicalDeviceCalibrateableTimeDomainsKHR =
        unsafe extern "system" fn(
            physical_device: PhysicalDevice,
            p_time_domain_count: *mut u32,
            p_time_domains: *mut TimeDomainKHR,
        ) -> vk::Result;
    pub type PFN_vkGetCalibratedTimestampsKHR = unsafe extern "system" fn(
        device: Device,
        timestamp_count: u32,
        p_timestamp_infos: *const CalibratedTimestampInfoKHR<'_>,
        p_timestamps: *mut u64,
        p_max_deviation: *mut u64,
    ) -> vk::Result;
}
pub struct InstanceFn {
    get_physical_device_calibrateable_time_domains_khr:
        PFN_vkGetPhysicalDeviceCalibrateableTimeDomainsKHR,
}
impl InstanceFn {
    pub unsafe fn load(
        load: impl Fn(&CStr) -> Option<PFN_vkVoidFunction>,
    ) -> core::result::Result<Self, LoadingError> {
        unsafe {
            Ok(Self {
                get_physical_device_calibrateable_time_domains_khr: transmute(
                    load(c"vkGetPhysicalDeviceCalibrateableTimeDomainsKHR").ok_or(LoadingError)?,
                ),
            })
        }
    }
}
impl InstanceFn {
    pub unsafe fn get_physical_device_calibrateable_time_domains_khr(
        &self,
        physical_device: PhysicalDevice,
        time_domains: impl ExtendUninit<TimeDomainKHR>,
    ) -> crate::Result<()> {
        unsafe {
            try_extend_uninit(time_domains, |time_domain_count, time_domains| {
                let result = (self.get_physical_device_calibrateable_time_domains_khr)(
                    physical_device,
                    time_domain_count,
                    time_domains as _,
                );

                match result {
                    VkResult::SUCCESS => Ok(()),
                    VkResult::INCOMPLETE => Ok(()),
                    err => Err(err),
                }
            })
        }
    }
}
pub struct DeviceFn {
    get_calibrated_timestamps_khr: PFN_vkGetCalibratedTimestampsKHR,
}
impl DeviceFn {
    pub unsafe fn load(
        load: impl Fn(&CStr) -> Option<PFN_vkVoidFunction>,
    ) -> core::result::Result<Self, LoadingError> {
        unsafe {
            Ok(Self {
                get_calibrated_timestamps_khr: transmute(
                    load(c"vkGetCalibratedTimestampsKHR").ok_or(LoadingError)?,
                ),
            })
        }
    }
}
impl DeviceFn {
    pub unsafe fn get_calibrated_timestamps_khr(
        &self,
        device: Device,
        timestamp_infos: &[CalibratedTimestampInfoKHR<'_>],
        timestamps: &mut [u64],
    ) -> crate::Result<u64> {
        unsafe {
            let mut max_deviation = core::mem::MaybeUninit::uninit();
            let result = (self.get_calibrated_timestamps_khr)(
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
