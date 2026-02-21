#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{c_char, c_int, c_void};
pub type TimeDomainEXT = TimeDomainKHR;
pub type CalibratedTimestampInfoEXT = CalibratedTimestampInfoKHR;
pub type PFN_vkGetPhysicalDeviceCalibrateableTimeDomainsEXT =
    PFN_vkGetPhysicalDeviceCalibrateableTimeDomainsKHR;
pub type PFN_vkGetCalibratedTimestampsEXT = PFN_vkGetCalibratedTimestampsKHR;
