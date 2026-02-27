#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{CStr, c_char, c_int, c_void};
use core::marker::PhantomData;
pub type PrivateDataSlotEXT = PrivateDataSlot;
pub type DevicePrivateDataCreateInfoEXT<'a> = DevicePrivateDataCreateInfo<'a>;
pub type PrivateDataSlotCreateInfoEXT<'a> = PrivateDataSlotCreateInfo<'a>;
pub type PhysicalDevicePrivateDataFeaturesEXT<'a> = PhysicalDevicePrivateDataFeatures<'a>;
pub type PrivateDataSlotCreateFlagsEXT = PrivateDataSlotCreateFlags;
pub type PFN_vkCreatePrivateDataSlotEXT = PFN_vkCreatePrivateDataSlot;
pub type PFN_vkDestroyPrivateDataSlotEXT = PFN_vkDestroyPrivateDataSlot;
pub type PFN_vkSetPrivateDataEXT = PFN_vkSetPrivateData;
pub type PFN_vkGetPrivateDataEXT = PFN_vkGetPrivateData;
