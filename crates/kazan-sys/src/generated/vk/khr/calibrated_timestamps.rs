#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{CStr, c_char, c_int, c_void};
use core::marker::PhantomData;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct CalibratedTimestampInfoKHR<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub time_domain: TimeDomainKHR,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for CalibratedTimestampInfoKHR<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::CALIBRATED_TIMESTAMP_INFO_KHR,
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
pub type PFN_vkGetPhysicalDeviceCalibrateableTimeDomainsKHR = unsafe extern "system" fn(
    physical_device: PhysicalDevice,
    p_time_domain_count: *mut u32,
    p_time_domains: *mut TimeDomainKHR,
) -> Result;
pub type PFN_vkGetCalibratedTimestampsKHR = unsafe extern "system" fn(
    device: Device,
    timestamp_count: u32,
    p_timestamp_infos: *const CalibratedTimestampInfoKHR<'_>,
    p_timestamps: *mut u64,
    p_max_deviation: *mut u64,
) -> Result;
