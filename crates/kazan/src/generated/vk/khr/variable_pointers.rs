//! <https://registry.khronos.org/vulkan/specs/latest/man/html/VK_KHR_variable_pointers.html>
#![allow(unused_imports)]
use crate::{vk::Result as VkResult, vk::*, *};
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
use core::ptr;

pub const EXTENSION_NAME: &CStr = c"VK_KHR_variable_pointers";

pub(super) mod defs {
    #![allow(non_camel_case_types, unused_imports)]
    use crate::{vk::*, *};
    use core::ffi::{CStr, c_char, c_int, c_void};
    use core::fmt;
    use core::marker::PhantomData;
    use core::ptr;

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPhysicalDeviceVariablePointersFeaturesKHR.html>
    pub type PhysicalDeviceVariablePointersFeaturesKHR<'a> =
        PhysicalDeviceVariablePointersFeatures<'a>;
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPhysicalDeviceVariablePointerFeaturesKHR.html>
    pub type PhysicalDeviceVariablePointerFeaturesKHR<'a> =
        PhysicalDeviceVariablePointersFeatures<'a>;
}

#[cfg(feature = "ffi")]
pub(super) mod ffi {
    #![allow(non_camel_case_types)]
    use super::defs::*;

    pub type VkPhysicalDeviceVariablePointersFeaturesKHR =
        PhysicalDeviceVariablePointersFeaturesKHR<'static>;
    pub type VkPhysicalDeviceVariablePointerFeaturesKHR =
        PhysicalDeviceVariablePointerFeaturesKHR<'static>;
}
