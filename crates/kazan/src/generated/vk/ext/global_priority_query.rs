//! <https://registry.khronos.org/vulkan/specs/latest/man/html/VK_EXT_global_priority_query.html>
#![allow(unused_imports)]
use crate::{vk::Result as VkResult, vk::*, *};
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
use core::ptr;

pub const EXTENSION_NAME: &CStr = c"VK_EXT_global_priority_query";

pub(super) mod defs {
    #![allow(non_camel_case_types, unused_imports)]
    use crate::{vk::*, *};
    use core::ffi::{CStr, c_char, c_int, c_void};
    use core::fmt;
    use core::marker::PhantomData;
    use core::ptr;

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPhysicalDeviceGlobalPriorityQueryFeaturesEXT.html>
    pub type PhysicalDeviceGlobalPriorityQueryFeaturesEXT<'a> =
        PhysicalDeviceGlobalPriorityQueryFeatures<'a>;
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkQueueFamilyGlobalPriorityPropertiesEXT.html>
    pub type QueueFamilyGlobalPriorityPropertiesEXT<'a> = QueueFamilyGlobalPriorityProperties<'a>;
}

#[cfg(feature = "ffi")]
pub(super) mod ffi {
    #![allow(non_camel_case_types)]
    use super::defs::*;

    pub type VkPhysicalDeviceGlobalPriorityQueryFeaturesEXT =
        PhysicalDeviceGlobalPriorityQueryFeaturesEXT<'static>;
    pub type VkQueueFamilyGlobalPriorityPropertiesEXT =
        QueueFamilyGlobalPriorityPropertiesEXT<'static>;
}
