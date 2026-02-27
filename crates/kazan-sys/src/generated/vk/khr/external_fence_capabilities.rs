#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{CStr, c_char, c_int, c_void};
use core::marker::PhantomData;
pub type PhysicalDeviceExternalFenceInfoKHR<'a> = PhysicalDeviceExternalFenceInfo<'a>;
pub type ExternalFencePropertiesKHR<'a> = ExternalFenceProperties<'a>;
pub type ExternalFenceHandleTypeFlagsKHR = ExternalFenceHandleTypeFlags;
pub type ExternalFenceFeatureFlagsKHR = ExternalFenceFeatureFlags;
pub type PFN_vkGetPhysicalDeviceExternalFencePropertiesKHR =
    PFN_vkGetPhysicalDeviceExternalFenceProperties;
