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
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPhysicalDeviceDescriptorIndexingFeaturesEXT.html>
    pub type PhysicalDeviceDescriptorIndexingFeaturesEXT<'a> =
        PhysicalDeviceDescriptorIndexingFeatures<'a>;
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPhysicalDeviceDescriptorIndexingPropertiesEXT.html>
    pub type PhysicalDeviceDescriptorIndexingPropertiesEXT<'a> =
        PhysicalDeviceDescriptorIndexingProperties<'a>;
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkDescriptorSetLayoutBindingFlagsCreateInfoEXT.html>
    pub type DescriptorSetLayoutBindingFlagsCreateInfoEXT<'a> =
        DescriptorSetLayoutBindingFlagsCreateInfo<'a>;
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkDescriptorSetVariableDescriptorCountAllocateInfoEXT.html>
    pub type DescriptorSetVariableDescriptorCountAllocateInfoEXT<'a> =
        DescriptorSetVariableDescriptorCountAllocateInfo<'a>;
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkDescriptorSetVariableDescriptorCountLayoutSupportEXT.html>
    pub type DescriptorSetVariableDescriptorCountLayoutSupportEXT<'a> =
        DescriptorSetVariableDescriptorCountLayoutSupport<'a>;
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkDescriptorBindingFlagsEXT.html>
    pub type DescriptorBindingFlagsEXT = DescriptorBindingFlags;
}
