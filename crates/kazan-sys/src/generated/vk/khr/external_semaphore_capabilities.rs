#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{c_char, c_int, c_void};
pub type PhysicalDeviceExternalSemaphoreInfoKHR = PhysicalDeviceExternalSemaphoreInfo;
pub type ExternalSemaphorePropertiesKHR = ExternalSemaphoreProperties;
pub type ExternalSemaphoreHandleTypeFlagsKHR = ExternalSemaphoreHandleTypeFlags;
pub type ExternalSemaphoreFeatureFlagsKHR = ExternalSemaphoreFeatureFlags;
pub type PFN_vkGetPhysicalDeviceExternalSemaphorePropertiesKHR =
    PFN_vkGetPhysicalDeviceExternalSemaphoreProperties;
