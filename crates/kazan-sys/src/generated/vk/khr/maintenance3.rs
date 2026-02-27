#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{CStr, c_char, c_int, c_void};
use core::marker::PhantomData;
pub type PhysicalDeviceMaintenance3PropertiesKHR<'a> = PhysicalDeviceMaintenance3Properties<'a>;
pub type DescriptorSetLayoutSupportKHR<'a> = DescriptorSetLayoutSupport<'a>;
pub type PFN_vkGetDescriptorSetLayoutSupportKHR = PFN_vkGetDescriptorSetLayoutSupport;
