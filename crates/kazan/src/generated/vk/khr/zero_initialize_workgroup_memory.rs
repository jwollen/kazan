//! <https://registry.khronos.org/vulkan/specs/latest/man/html/VK_KHR_zero_initialize_workgroup_memory.html>
#![allow(unused_imports)]
use crate::{vk::Result as VkResult, vk::*, *};
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
use core::ptr;

pub const EXTENSION_NAME: &CStr = c"VK_KHR_zero_initialize_workgroup_memory";

pub(super) mod defs {
    #![allow(non_camel_case_types, unused_imports)]
    use crate::{vk::*, *};
    use core::ffi::{CStr, c_char, c_int, c_void};
    use core::fmt;
    use core::marker::PhantomData;
    use core::ptr;

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPhysicalDeviceZeroInitializeWorkgroupMemoryFeaturesKHR.html>
    pub type PhysicalDeviceZeroInitializeWorkgroupMemoryFeaturesKHR<'a> =
        PhysicalDeviceZeroInitializeWorkgroupMemoryFeatures<'a>;
}
