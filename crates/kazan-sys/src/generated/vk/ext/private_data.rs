#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{c_char, c_int, c_void};
pub type PrivateDataSlotEXT = PrivateDataSlot;
pub type DevicePrivateDataCreateInfoEXT = DevicePrivateDataCreateInfo;
pub type PrivateDataSlotCreateInfoEXT = PrivateDataSlotCreateInfo;
pub type PhysicalDevicePrivateDataFeaturesEXT = PhysicalDevicePrivateDataFeatures;
pub type PrivateDataSlotCreateFlagsEXT = PrivateDataSlotCreateFlags;
pub type PFN_vkCreatePrivateDataSlotEXT = PFN_vkCreatePrivateDataSlot;
pub type PFN_vkDestroyPrivateDataSlotEXT = PFN_vkDestroyPrivateDataSlot;
pub type PFN_vkSetPrivateDataEXT = PFN_vkSetPrivateData;
pub type PFN_vkGetPrivateDataEXT = PFN_vkGetPrivateData;
