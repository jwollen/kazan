#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{CStr, c_char, c_int, c_void};
use core::marker::PhantomData;
pub type LineRasterizationModeKHR = LineRasterizationMode;
pub type PhysicalDeviceLineRasterizationFeaturesKHR<'a> =
    PhysicalDeviceLineRasterizationFeatures<'a>;
pub type PhysicalDeviceLineRasterizationPropertiesKHR<'a> =
    PhysicalDeviceLineRasterizationProperties<'a>;
pub type PipelineRasterizationLineStateCreateInfoKHR<'a> =
    PipelineRasterizationLineStateCreateInfo<'a>;
pub type PFN_vkCmdSetLineStippleKHR = PFN_vkCmdSetLineStipple;
