#![allow(unused_imports)]
use crate::{vk::Result as VkResult, vk::*, *};
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;

pub const EXTENSION_NAME: &CStr = c"VK_KHR_vertex_attribute_divisor";

pub(super) mod defs {
    #![allow(non_camel_case_types, unused_imports)]
    use crate::{vk::*, *};
    use core::ffi::{CStr, c_char, c_int, c_void};
    use core::fmt;
    use core::marker::PhantomData;

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkVertexInputBindingDivisorDescriptionKHR.html>
    pub type VertexInputBindingDivisorDescriptionKHR = VertexInputBindingDivisorDescription;
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPipelineVertexInputDivisorStateCreateInfoKHR.html>
    pub type PipelineVertexInputDivisorStateCreateInfoKHR<'a> =
        PipelineVertexInputDivisorStateCreateInfo<'a>;
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPhysicalDeviceVertexAttributeDivisorPropertiesKHR.html>
    pub type PhysicalDeviceVertexAttributeDivisorPropertiesKHR<'a> =
        PhysicalDeviceVertexAttributeDivisorProperties<'a>;
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPhysicalDeviceVertexAttributeDivisorFeaturesKHR.html>
    pub type PhysicalDeviceVertexAttributeDivisorFeaturesKHR<'a> =
        PhysicalDeviceVertexAttributeDivisorFeatures<'a>;
}
