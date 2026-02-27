#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{CStr, c_char, c_int, c_void};
use core::marker::PhantomData;
pub type PhysicalDeviceDescriptorIndexingFeaturesEXT<'a> =
    PhysicalDeviceDescriptorIndexingFeatures<'a>;
pub type PhysicalDeviceDescriptorIndexingPropertiesEXT<'a> =
    PhysicalDeviceDescriptorIndexingProperties<'a>;
pub type DescriptorSetLayoutBindingFlagsCreateInfoEXT<'a> =
    DescriptorSetLayoutBindingFlagsCreateInfo<'a>;
pub type DescriptorSetVariableDescriptorCountAllocateInfoEXT<'a> =
    DescriptorSetVariableDescriptorCountAllocateInfo<'a>;
pub type DescriptorSetVariableDescriptorCountLayoutSupportEXT<'a> =
    DescriptorSetVariableDescriptorCountLayoutSupport<'a>;
pub type DescriptorBindingFlagsEXT = DescriptorBindingFlags;
