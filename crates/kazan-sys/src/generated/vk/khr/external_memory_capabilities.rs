#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{c_char, c_int, c_void};
use core::marker::PhantomData;
pub type ExternalMemoryPropertiesKHR = ExternalMemoryProperties;
pub type PhysicalDeviceExternalImageFormatInfoKHR<'a> = PhysicalDeviceExternalImageFormatInfo<'a>;
pub type ExternalImageFormatPropertiesKHR<'a> = ExternalImageFormatProperties<'a>;
pub type PhysicalDeviceExternalBufferInfoKHR<'a> = PhysicalDeviceExternalBufferInfo<'a>;
pub type ExternalBufferPropertiesKHR<'a> = ExternalBufferProperties<'a>;
pub type PhysicalDeviceIDPropertiesKHR<'a> = PhysicalDeviceIDProperties<'a>;
pub type ExternalMemoryHandleTypeFlagsKHR = ExternalMemoryHandleTypeFlags;
pub type ExternalMemoryFeatureFlagsKHR = ExternalMemoryFeatureFlags;
pub type PFN_vkGetPhysicalDeviceExternalBufferPropertiesKHR =
    PFN_vkGetPhysicalDeviceExternalBufferProperties;
