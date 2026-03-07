#![allow(unused_imports)]
use crate::{vk::Result as VkResult, vk::*, *};
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;

pub(super) mod defs {
    #![allow(non_camel_case_types, unused_imports)]
    use crate::{vk::*, *};
    use core::ffi::{CStr, c_char, c_int, c_void};
    use core::fmt;
    use core::marker::PhantomData;

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPipelineRobustnessBufferBehaviorEXT.html>
    pub type PipelineRobustnessBufferBehaviorEXT = PipelineRobustnessBufferBehavior;
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPipelineRobustnessImageBehaviorEXT.html>
    pub type PipelineRobustnessImageBehaviorEXT = PipelineRobustnessImageBehavior;
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPhysicalDevicePipelineRobustnessFeaturesEXT.html>
    pub type PhysicalDevicePipelineRobustnessFeaturesEXT<'a> =
        PhysicalDevicePipelineRobustnessFeatures<'a>;
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPipelineRobustnessCreateInfoEXT.html>
    pub type PipelineRobustnessCreateInfoEXT<'a> = PipelineRobustnessCreateInfo<'a>;
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPhysicalDevicePipelineRobustnessPropertiesEXT.html>
    pub type PhysicalDevicePipelineRobustnessPropertiesEXT<'a> =
        PhysicalDevicePipelineRobustnessProperties<'a>;
}
