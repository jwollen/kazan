#![allow(unused_imports)]
use crate::{vk::Result as VkResult, vk::*, *};
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
pub(super) mod defs {
    #![allow(non_camel_case_types, unused_imports)]
    use crate::{vk::*, *};
    use core::ffi::{CStr, c_char, c_int, c_void};
    use core::fmt;
    use core::marker::PhantomData;
    pub type SurfacePresentModeEXT<'a> = SurfacePresentModeKHR<'a>;
    pub type SurfacePresentScalingCapabilitiesEXT<'a> = SurfacePresentScalingCapabilitiesKHR<'a>;
    pub type SurfacePresentModeCompatibilityEXT<'a> = SurfacePresentModeCompatibilityKHR<'a>;
    pub type PresentScalingFlagsEXT = PresentScalingFlagsKHR;
    pub type PresentGravityFlagsEXT = PresentGravityFlagsKHR;
}
