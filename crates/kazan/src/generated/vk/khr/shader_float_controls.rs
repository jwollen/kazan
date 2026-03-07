//! <https://registry.khronos.org/vulkan/specs/latest/man/html/VK_KHR_shader_float_controls.html>
#![allow(unused_imports)]
use crate::{vk::Result as VkResult, vk::*, *};
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;

pub const EXTENSION_NAME: &CStr = c"VK_KHR_shader_float_controls";

pub(super) mod defs {
    #![allow(non_camel_case_types, unused_imports)]
    use crate::{vk::*, *};
    use core::ffi::{CStr, c_char, c_int, c_void};
    use core::fmt;
    use core::marker::PhantomData;

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkShaderFloatControlsIndependenceKHR.html>
    pub type ShaderFloatControlsIndependenceKHR = ShaderFloatControlsIndependence;
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPhysicalDeviceFloatControlsPropertiesKHR.html>
    pub type PhysicalDeviceFloatControlsPropertiesKHR<'a> =
        PhysicalDeviceFloatControlsProperties<'a>;
}
