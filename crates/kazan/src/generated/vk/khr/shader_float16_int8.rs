//! <https://registry.khronos.org/vulkan/specs/latest/man/html/VK_KHR_shader_float16_int8.html>
#![allow(unused_imports)]
use crate::{vk::Result as VkResult, vk::*, *};
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;

pub const EXTENSION_NAME: &CStr = c"VK_KHR_shader_float16_int8";

pub(super) mod defs {
    #![allow(non_camel_case_types, unused_imports)]
    use crate::{vk::*, *};
    use core::ffi::{CStr, c_char, c_int, c_void};
    use core::fmt;
    use core::marker::PhantomData;

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPhysicalDeviceShaderFloat16Int8FeaturesKHR.html>
    pub type PhysicalDeviceShaderFloat16Int8FeaturesKHR<'a> =
        PhysicalDeviceShaderFloat16Int8Features<'a>;
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPhysicalDeviceFloat16Int8FeaturesKHR.html>
    pub type PhysicalDeviceFloat16Int8FeaturesKHR<'a> = PhysicalDeviceShaderFloat16Int8Features<'a>;
}
