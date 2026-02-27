#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{CStr, c_char, c_int, c_void};
use core::marker::PhantomData;
pub type PhysicalDeviceExternalSemaphoreInfoKHR<'a> = PhysicalDeviceExternalSemaphoreInfo<'a>;
pub type ExternalSemaphorePropertiesKHR<'a> = ExternalSemaphoreProperties<'a>;
pub type ExternalSemaphoreHandleTypeFlagsKHR = ExternalSemaphoreHandleTypeFlags;
pub type ExternalSemaphoreFeatureFlagsKHR = ExternalSemaphoreFeatureFlags;
pub type PFN_vkGetPhysicalDeviceExternalSemaphorePropertiesKHR =
    PFN_vkGetPhysicalDeviceExternalSemaphoreProperties;
