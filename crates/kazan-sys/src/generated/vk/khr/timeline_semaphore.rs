#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{CStr, c_char, c_int, c_void};
use core::marker::PhantomData;
pub type SemaphoreTypeKHR = SemaphoreType;
pub type PhysicalDeviceTimelineSemaphoreFeaturesKHR<'a> =
    PhysicalDeviceTimelineSemaphoreFeatures<'a>;
pub type PhysicalDeviceTimelineSemaphorePropertiesKHR<'a> =
    PhysicalDeviceTimelineSemaphoreProperties<'a>;
pub type SemaphoreTypeCreateInfoKHR<'a> = SemaphoreTypeCreateInfo<'a>;
pub type TimelineSemaphoreSubmitInfoKHR<'a> = TimelineSemaphoreSubmitInfo<'a>;
pub type SemaphoreWaitInfoKHR<'a> = SemaphoreWaitInfo<'a>;
pub type SemaphoreSignalInfoKHR<'a> = SemaphoreSignalInfo<'a>;
pub type SemaphoreWaitFlagsKHR = SemaphoreWaitFlags;
pub type PFN_vkGetSemaphoreCounterValueKHR = PFN_vkGetSemaphoreCounterValue;
pub type PFN_vkWaitSemaphoresKHR = PFN_vkWaitSemaphores;
pub type PFN_vkSignalSemaphoreKHR = PFN_vkSignalSemaphore;
