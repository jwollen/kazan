//! <https://registry.khronos.org/vulkan/specs/latest/man/html/VK_EXT_robustness2.html>
#![allow(unused_imports)]
use crate::{vk::Result as VkResult, vk::*, *};
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
use core::ptr;

pub const EXTENSION_NAME: &CStr = c"VK_EXT_robustness2";

pub(super) mod defs {
    #![allow(non_camel_case_types, unused_imports)]
    use crate::{vk::*, *};
    use core::ffi::{CStr, c_char, c_int, c_void};
    use core::fmt;
    use core::marker::PhantomData;
    use core::ptr;

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPhysicalDeviceRobustness2FeaturesEXT.html>
    pub type PhysicalDeviceRobustness2FeaturesEXT<'a> = PhysicalDeviceRobustness2FeaturesKHR<'a>;
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPhysicalDeviceRobustness2PropertiesEXT.html>
    pub type PhysicalDeviceRobustness2PropertiesEXT<'a> =
        PhysicalDeviceRobustness2PropertiesKHR<'a>;
}
