#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{c_char, c_int, c_void};
pub type PhysicalDeviceExternalFenceInfoKHR = PhysicalDeviceExternalFenceInfo;
pub type ExternalFencePropertiesKHR = ExternalFenceProperties;
pub type ExternalFenceHandleTypeFlagsKHR = ExternalFenceHandleTypeFlags;
pub type ExternalFenceFeatureFlagsKHR = ExternalFenceFeatureFlags;
pub type PFN_vkGetPhysicalDeviceExternalFencePropertiesKHR =
    PFN_vkGetPhysicalDeviceExternalFenceProperties;
