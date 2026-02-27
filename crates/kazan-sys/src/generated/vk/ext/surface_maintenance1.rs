#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{CStr, c_char, c_int, c_void};
use core::marker::PhantomData;
pub type SurfacePresentModeEXT<'a> = SurfacePresentModeKHR<'a>;
pub type SurfacePresentScalingCapabilitiesEXT<'a> = SurfacePresentScalingCapabilitiesKHR<'a>;
pub type SurfacePresentModeCompatibilityEXT<'a> = SurfacePresentModeCompatibilityKHR<'a>;
pub type PresentScalingFlagsEXT = PresentScalingFlagsKHR;
pub type PresentGravityFlagsEXT = PresentGravityFlagsKHR;
