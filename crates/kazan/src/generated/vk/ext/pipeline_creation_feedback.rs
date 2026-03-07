//! <https://registry.khronos.org/vulkan/specs/latest/man/html/VK_EXT_pipeline_creation_feedback.html>
#![allow(unused_imports)]
use crate::{vk::Result as VkResult, vk::*, *};
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;

pub const EXTENSION_NAME: &CStr = c"VK_EXT_pipeline_creation_feedback";

pub(super) mod defs {
    #![allow(non_camel_case_types, unused_imports)]
    use crate::{vk::*, *};
    use core::ffi::{CStr, c_char, c_int, c_void};
    use core::fmt;
    use core::marker::PhantomData;

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPipelineCreationFeedbackEXT.html>
    pub type PipelineCreationFeedbackEXT = PipelineCreationFeedback;
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPipelineCreationFeedbackCreateInfoEXT.html>
    pub type PipelineCreationFeedbackCreateInfoEXT<'a> = PipelineCreationFeedbackCreateInfo<'a>;
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPipelineCreationFeedbackFlagsEXT.html>
    pub type PipelineCreationFeedbackFlagsEXT = PipelineCreationFeedbackFlags;
}
