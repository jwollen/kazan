#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{CStr, c_char, c_int, c_void};
use core::marker::PhantomData;
pub type SamplerReductionModeEXT = SamplerReductionMode;
pub type PhysicalDeviceSamplerFilterMinmaxPropertiesEXT<'a> =
    PhysicalDeviceSamplerFilterMinmaxProperties<'a>;
pub type SamplerReductionModeCreateInfoEXT<'a> = SamplerReductionModeCreateInfo<'a>;
