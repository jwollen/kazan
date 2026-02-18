#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use std::ffi::{c_char, c_int, c_void};
pub type SemaphoreTypeKHR = SemaphoreType;
pub type PhysicalDeviceTimelineSemaphoreFeaturesKHR = PhysicalDeviceTimelineSemaphoreFeatures;
pub type PhysicalDeviceTimelineSemaphorePropertiesKHR = PhysicalDeviceTimelineSemaphoreProperties;
pub type SemaphoreTypeCreateInfoKHR = SemaphoreTypeCreateInfo;
pub type TimelineSemaphoreSubmitInfoKHR = TimelineSemaphoreSubmitInfo;
pub type SemaphoreWaitInfoKHR = SemaphoreWaitInfo;
pub type SemaphoreSignalInfoKHR = SemaphoreSignalInfo;
pub type SemaphoreWaitFlagsKHR = SemaphoreWaitFlags;
pub type PFN_vkGetSemaphoreCounterValueKHR = PFN_vkGetSemaphoreCounterValue;
pub type PFN_vkWaitSemaphoresKHR = PFN_vkWaitSemaphores;
pub type PFN_vkSignalSemaphoreKHR = PFN_vkSignalSemaphore;
