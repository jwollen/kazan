//! <https://registry.khronos.org/vulkan/specs/latest/man/html/VK_EXT_subgroup_size_control.html>
#![allow(unused_imports)]
use crate::{vk::Result as VkResult, vk::*, *};
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
use core::ptr;

pub const EXTENSION_NAME: &CStr = c"VK_EXT_subgroup_size_control";

pub(super) mod defs {
    #![allow(non_camel_case_types, unused_imports)]
    use crate::{vk::*, *};
    use core::ffi::{CStr, c_char, c_int, c_void};
    use core::fmt;
    use core::marker::PhantomData;
    use core::ptr;

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPhysicalDeviceSubgroupSizeControlFeaturesEXT.html>
    pub type PhysicalDeviceSubgroupSizeControlFeaturesEXT<'a> =
        PhysicalDeviceSubgroupSizeControlFeatures<'a>;
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPhysicalDeviceSubgroupSizeControlPropertiesEXT.html>
    pub type PhysicalDeviceSubgroupSizeControlPropertiesEXT<'a> =
        PhysicalDeviceSubgroupSizeControlProperties<'a>;
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPipelineShaderStageRequiredSubgroupSizeCreateInfoEXT.html>
    pub type PipelineShaderStageRequiredSubgroupSizeCreateInfoEXT<'a> =
        PipelineShaderStageRequiredSubgroupSizeCreateInfo<'a>;
}

#[cfg(feature = "ffi")]
pub(super) mod ffi {
    #![allow(non_camel_case_types)]
    use super::defs::*;

    pub type VkPhysicalDeviceSubgroupSizeControlFeaturesEXT =
        PhysicalDeviceSubgroupSizeControlFeaturesEXT<'static>;
    pub type VkPhysicalDeviceSubgroupSizeControlPropertiesEXT =
        PhysicalDeviceSubgroupSizeControlPropertiesEXT<'static>;
    pub type VkPipelineShaderStageRequiredSubgroupSizeCreateInfoEXT =
        PipelineShaderStageRequiredSubgroupSizeCreateInfoEXT<'static>;
}
