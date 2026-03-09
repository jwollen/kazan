//! <https://registry.khronos.org/vulkan/specs/latest/man/html/VK_EXT_surface_maintenance1.html>
#![allow(unused_imports)]
use crate::{vk::Result as VkResult, vk::*, *};
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
use core::ptr;

pub const EXTENSION_NAME: &CStr = c"VK_EXT_surface_maintenance1";

pub(super) mod defs {
    #![allow(non_camel_case_types, unused_imports)]
    use crate::{vk::*, *};
    use core::ffi::{CStr, c_char, c_int, c_void};
    use core::fmt;
    use core::marker::PhantomData;
    use core::ptr;

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkSurfacePresentModeEXT.html>
    pub type SurfacePresentModeEXT<'a> = SurfacePresentModeKHR<'a>;
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkSurfacePresentScalingCapabilitiesEXT.html>
    pub type SurfacePresentScalingCapabilitiesEXT<'a> = SurfacePresentScalingCapabilitiesKHR<'a>;
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkSurfacePresentModeCompatibilityEXT.html>
    pub type SurfacePresentModeCompatibilityEXT<'a> = SurfacePresentModeCompatibilityKHR<'a>;
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPresentScalingFlagsEXT.html>
    pub type PresentScalingFlagsEXT = PresentScalingFlagsKHR;
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPresentGravityFlagsEXT.html>
    pub type PresentGravityFlagsEXT = PresentGravityFlagsKHR;
}

#[cfg(feature = "ffi")]
pub(super) mod ffi {
    #![allow(non_camel_case_types)]
    use super::defs::*;

    pub type VkSurfacePresentModeEXT = SurfacePresentModeEXT<'static>;
    pub type VkSurfacePresentScalingCapabilitiesEXT = SurfacePresentScalingCapabilitiesEXT<'static>;
    pub type VkSurfacePresentModeCompatibilityEXT = SurfacePresentModeCompatibilityEXT<'static>;
    pub type VkPresentScalingFlagsEXT = PresentScalingFlagsEXT;
    pub type VkPresentGravityFlagsEXT = PresentGravityFlagsEXT;
}
