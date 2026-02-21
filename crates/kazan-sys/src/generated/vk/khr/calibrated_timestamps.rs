#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{c_char, c_int, c_void};
#[repr(C)]
#[derive(Copy, Clone)]
pub struct CalibratedTimestampInfoKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub time_domain: TimeDomainKHR,
}
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TimeDomainKHR(i32);
impl TimeDomainKHR {
    pub const DEVICE_KHR: Self = Self(0);
    pub const CLOCK_MONOTONIC_KHR: Self = Self(1);
    pub const CLOCK_MONOTONIC_RAW_KHR: Self = Self(2);
    pub const QUERY_PERFORMANCE_COUNTER_KHR: Self = Self(3);
}
pub type PFN_vkGetPhysicalDeviceCalibrateableTimeDomainsKHR = unsafe extern "system" fn(
    physical_device: PhysicalDevice,
    p_time_domain_count: *mut u32,
    p_time_domains: *mut TimeDomainKHR,
) -> Result;
pub type PFN_vkGetCalibratedTimestampsKHR = unsafe extern "system" fn(
    device: Device,
    timestamp_count: u32,
    p_timestamp_infos: *const CalibratedTimestampInfoKHR,
    p_timestamps: *mut u64,
    p_max_deviation: *mut u64,
) -> Result;
