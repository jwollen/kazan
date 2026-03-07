//! <https://registry.khronos.org/vulkan/specs/latest/man/html/VK_KHR_global_priority.html>
#![allow(unused_imports)]
use crate::{vk::Result as VkResult, vk::*, *};
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;

pub const EXTENSION_NAME: &CStr = c"VK_KHR_global_priority";

pub(super) mod defs {
    #![allow(non_camel_case_types, unused_imports)]
    use crate::{vk::*, *};
    use core::ffi::{CStr, c_char, c_int, c_void};
    use core::fmt;
    use core::marker::PhantomData;

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkQueueGlobalPriorityKHR.html>
    pub type QueueGlobalPriorityKHR = QueueGlobalPriority;
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkDeviceQueueGlobalPriorityCreateInfoKHR.html>
    pub type DeviceQueueGlobalPriorityCreateInfoKHR<'a> = DeviceQueueGlobalPriorityCreateInfo<'a>;
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPhysicalDeviceGlobalPriorityQueryFeaturesKHR.html>
    pub type PhysicalDeviceGlobalPriorityQueryFeaturesKHR<'a> =
        PhysicalDeviceGlobalPriorityQueryFeatures<'a>;
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkQueueFamilyGlobalPriorityPropertiesKHR.html>
    pub type QueueFamilyGlobalPriorityPropertiesKHR<'a> = QueueFamilyGlobalPriorityProperties<'a>;
}
