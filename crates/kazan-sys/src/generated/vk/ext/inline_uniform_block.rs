#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{c_char, c_int, c_void};
use core::marker::PhantomData;
pub type PhysicalDeviceInlineUniformBlockFeaturesEXT<'a> =
    PhysicalDeviceInlineUniformBlockFeatures<'a>;
pub type PhysicalDeviceInlineUniformBlockPropertiesEXT<'a> =
    PhysicalDeviceInlineUniformBlockProperties<'a>;
pub type WriteDescriptorSetInlineUniformBlockEXT<'a> = WriteDescriptorSetInlineUniformBlock<'a>;
pub type DescriptorPoolInlineUniformBlockCreateInfoEXT<'a> =
    DescriptorPoolInlineUniformBlockCreateInfo<'a>;
