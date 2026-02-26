#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{c_char, c_int, c_void};
use core::marker::PhantomData;
pub type PhysicalDeviceSubgroupSizeControlFeaturesEXT<'a> =
    PhysicalDeviceSubgroupSizeControlFeatures<'a>;
pub type PhysicalDeviceSubgroupSizeControlPropertiesEXT<'a> =
    PhysicalDeviceSubgroupSizeControlProperties<'a>;
pub type PipelineShaderStageRequiredSubgroupSizeCreateInfoEXT<'a> =
    PipelineShaderStageRequiredSubgroupSizeCreateInfo<'a>;
