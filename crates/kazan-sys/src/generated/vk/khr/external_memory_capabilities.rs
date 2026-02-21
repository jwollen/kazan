#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{c_char, c_int, c_void};
pub type ExternalMemoryPropertiesKHR = ExternalMemoryProperties;
pub type PhysicalDeviceExternalImageFormatInfoKHR = PhysicalDeviceExternalImageFormatInfo;
pub type ExternalImageFormatPropertiesKHR = ExternalImageFormatProperties;
pub type PhysicalDeviceExternalBufferInfoKHR = PhysicalDeviceExternalBufferInfo;
pub type ExternalBufferPropertiesKHR = ExternalBufferProperties;
pub type PhysicalDeviceIDPropertiesKHR = PhysicalDeviceIDProperties;
pub type ExternalMemoryHandleTypeFlagsKHR = ExternalMemoryHandleTypeFlags;
pub type ExternalMemoryFeatureFlagsKHR = ExternalMemoryFeatureFlags;
pub type PFN_vkGetPhysicalDeviceExternalBufferPropertiesKHR =
    PFN_vkGetPhysicalDeviceExternalBufferProperties;
