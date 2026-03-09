//! <https://registry.khronos.org/vulkan/specs/latest/man/html/VK_KHR_driver_properties.html>
#![allow(unused_imports)]
use crate::{vk::Result as VkResult, vk::*, *};
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
use core::ptr;

pub const EXTENSION_NAME: &CStr = c"VK_KHR_driver_properties";

pub(super) mod defs {
    #![allow(non_camel_case_types, unused_imports)]
    use crate::{vk::*, *};
    use core::ffi::{CStr, c_char, c_int, c_void};
    use core::fmt;
    use core::marker::PhantomData;
    use core::ptr;

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkDriverIdKHR.html>
    pub type DriverIdKHR = DriverId;
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkConformanceVersionKHR.html>
    pub type ConformanceVersionKHR = ConformanceVersion;
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPhysicalDeviceDriverPropertiesKHR.html>
    pub type PhysicalDeviceDriverPropertiesKHR<'a> = PhysicalDeviceDriverProperties<'a>;
}

#[cfg(feature = "ffi")]
pub(super) mod ffi {
    #![allow(non_camel_case_types)]
    use super::defs::*;

    pub type VkDriverIdKHR = DriverIdKHR;
    pub type VkConformanceVersionKHR = ConformanceVersionKHR;
    pub type VkPhysicalDeviceDriverPropertiesKHR = PhysicalDeviceDriverPropertiesKHR<'static>;
}
