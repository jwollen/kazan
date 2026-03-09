//! <https://registry.khronos.org/vulkan/specs/latest/man/html/VK_EXT_pipeline_robustness.html>
#![allow(unused_imports)]
use crate::{vk::Result as VkResult, vk::*, *};
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
use core::ptr;

pub const EXTENSION_NAME: &CStr = c"VK_EXT_pipeline_robustness";

pub(super) mod defs {
    #![allow(non_camel_case_types, unused_imports)]
    use crate::{vk::*, *};
    use core::ffi::{CStr, c_char, c_int, c_void};
    use core::fmt;
    use core::marker::PhantomData;
    use core::ptr;

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

#[cfg(feature = "ffi")]
pub(super) mod ffi {
    #![allow(non_camel_case_types)]
    use super::defs::*;

    pub type VkPipelineRobustnessBufferBehaviorEXT = PipelineRobustnessBufferBehaviorEXT;
    pub type VkPipelineRobustnessImageBehaviorEXT = PipelineRobustnessImageBehaviorEXT;
    pub type VkPhysicalDevicePipelineRobustnessFeaturesEXT =
        PhysicalDevicePipelineRobustnessFeaturesEXT<'static>;
    pub type VkPipelineRobustnessCreateInfoEXT = PipelineRobustnessCreateInfoEXT<'static>;
    pub type VkPhysicalDevicePipelineRobustnessPropertiesEXT =
        PhysicalDevicePipelineRobustnessPropertiesEXT<'static>;
}
