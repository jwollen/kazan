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

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPhysicalDeviceInlineUniformBlockFeaturesEXT.html>
    pub type PhysicalDeviceInlineUniformBlockFeaturesEXT<'a> =
        PhysicalDeviceInlineUniformBlockFeatures<'a>;
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPhysicalDeviceInlineUniformBlockPropertiesEXT.html>
    pub type PhysicalDeviceInlineUniformBlockPropertiesEXT<'a> =
        PhysicalDeviceInlineUniformBlockProperties<'a>;
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkWriteDescriptorSetInlineUniformBlockEXT.html>
    pub type WriteDescriptorSetInlineUniformBlockEXT<'a> = WriteDescriptorSetInlineUniformBlock<'a>;
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkDescriptorPoolInlineUniformBlockCreateInfoEXT.html>
    pub type DescriptorPoolInlineUniformBlockCreateInfoEXT<'a> =
        DescriptorPoolInlineUniformBlockCreateInfo<'a>;
}
