//! <https://registry.khronos.org/vulkan/specs/latest/man/html/VK_EXT_global_priority.html>
#![allow(unused_imports)]
use crate::{vk::Result as VkResult, vk::*, *};
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
use core::ptr;

pub const EXTENSION_NAME: &CStr = c"VK_EXT_global_priority";

pub(super) mod defs {
    #![allow(non_camel_case_types, unused_imports)]
    use crate::{vk::*, *};
    use core::ffi::{CStr, c_char, c_int, c_void};
    use core::fmt;
    use core::marker::PhantomData;
    use core::ptr;

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkQueueGlobalPriorityEXT.html>
    pub type QueueGlobalPriorityEXT = QueueGlobalPriority;
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkDeviceQueueGlobalPriorityCreateInfoEXT.html>
    pub type DeviceQueueGlobalPriorityCreateInfoEXT<'a> = DeviceQueueGlobalPriorityCreateInfo<'a>;
}

#[cfg(feature = "ffi")]
pub(super) mod ffi {
    #![allow(non_camel_case_types)]
    use super::defs::*;

    pub type VkQueueGlobalPriorityEXT = QueueGlobalPriorityEXT;
    pub type VkDeviceQueueGlobalPriorityCreateInfoEXT =
        DeviceQueueGlobalPriorityCreateInfoEXT<'static>;
}
