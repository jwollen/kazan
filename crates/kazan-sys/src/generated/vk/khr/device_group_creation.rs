#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{CStr, c_char, c_int, c_void};
use core::marker::PhantomData;
pub type PhysicalDeviceGroupPropertiesKHR<'a> = PhysicalDeviceGroupProperties<'a>;
pub type DeviceGroupDeviceCreateInfoKHR<'a> = DeviceGroupDeviceCreateInfo<'a>;
pub type PFN_vkEnumeratePhysicalDeviceGroupsKHR = PFN_vkEnumeratePhysicalDeviceGroups;
