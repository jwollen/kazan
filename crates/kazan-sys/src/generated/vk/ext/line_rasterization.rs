#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{c_char, c_int, c_void};
use core::marker::PhantomData;
pub type LineRasterizationModeEXT = LineRasterizationMode;
pub type PhysicalDeviceLineRasterizationFeaturesEXT<'a> =
    PhysicalDeviceLineRasterizationFeatures<'a>;
pub type PhysicalDeviceLineRasterizationPropertiesEXT<'a> =
    PhysicalDeviceLineRasterizationProperties<'a>;
pub type PipelineRasterizationLineStateCreateInfoEXT<'a> =
    PipelineRasterizationLineStateCreateInfo<'a>;
pub type PFN_vkCmdSetLineStippleEXT = PFN_vkCmdSetLineStipple;
