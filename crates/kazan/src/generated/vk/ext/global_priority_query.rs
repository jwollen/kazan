#![allow(unused_imports)]
use crate::{vk::Result as VkResult, vk::*, *};
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;

pub const EXTENSION_NAME: &CStr = c"VK_EXT_global_priority_query";

pub(super) mod defs {
    #![allow(non_camel_case_types, unused_imports)]
    use crate::{vk::*, *};
    use core::ffi::{CStr, c_char, c_int, c_void};
    use core::fmt;
    use core::marker::PhantomData;

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPhysicalDeviceGlobalPriorityQueryFeaturesEXT.html>
    pub type PhysicalDeviceGlobalPriorityQueryFeaturesEXT<'a> =
        PhysicalDeviceGlobalPriorityQueryFeatures<'a>;
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkQueueFamilyGlobalPriorityPropertiesEXT.html>
    pub type QueueFamilyGlobalPriorityPropertiesEXT<'a> = QueueFamilyGlobalPriorityProperties<'a>;
}
