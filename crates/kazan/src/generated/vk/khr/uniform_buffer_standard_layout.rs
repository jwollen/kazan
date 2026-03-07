//! <https://registry.khronos.org/vulkan/specs/latest/man/html/VK_KHR_uniform_buffer_standard_layout.html>
#![allow(unused_imports)]
use crate::{vk::Result as VkResult, vk::*, *};
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
use core::ptr;

pub const EXTENSION_NAME: &CStr = c"VK_KHR_uniform_buffer_standard_layout";

pub(super) mod defs {
    #![allow(non_camel_case_types, unused_imports)]
    use crate::{vk::*, *};
    use core::ffi::{CStr, c_char, c_int, c_void};
    use core::fmt;
    use core::marker::PhantomData;
    use core::ptr;

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPhysicalDeviceUniformBufferStandardLayoutFeaturesKHR.html>
    pub type PhysicalDeviceUniformBufferStandardLayoutFeaturesKHR<'a> =
        PhysicalDeviceUniformBufferStandardLayoutFeatures<'a>;
}
