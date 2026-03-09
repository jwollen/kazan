//! <https://registry.khronos.org/vulkan/specs/latest/man/html/VK_QCOM_fragment_density_map_offset.html>
#![allow(unused_imports)]
use crate::{vk::Result as VkResult, vk::*, *};
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
use core::ptr;

pub const EXTENSION_NAME: &CStr = c"VK_QCOM_fragment_density_map_offset";

pub(super) mod defs {
    #![allow(non_camel_case_types, unused_imports)]
    use crate::{vk::*, *};
    use core::ffi::{CStr, c_char, c_int, c_void};
    use core::fmt;
    use core::marker::PhantomData;
    use core::ptr;

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPhysicalDeviceFragmentDensityMapOffsetFeaturesQCOM.html>
    pub type PhysicalDeviceFragmentDensityMapOffsetFeaturesQCOM<'a> =
        PhysicalDeviceFragmentDensityMapOffsetFeaturesEXT<'a>;
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPhysicalDeviceFragmentDensityMapOffsetPropertiesQCOM.html>
    pub type PhysicalDeviceFragmentDensityMapOffsetPropertiesQCOM<'a> =
        PhysicalDeviceFragmentDensityMapOffsetPropertiesEXT<'a>;
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkSubpassFragmentDensityMapOffsetEndInfoQCOM.html>
    pub type SubpassFragmentDensityMapOffsetEndInfoQCOM<'a> =
        RenderPassFragmentDensityMapOffsetEndInfoEXT<'a>;
}

#[cfg(feature = "ffi")]
pub(super) mod ffi {
    #![allow(non_camel_case_types)]
    use super::defs::*;

    pub type VkPhysicalDeviceFragmentDensityMapOffsetFeaturesQCOM =
        PhysicalDeviceFragmentDensityMapOffsetFeaturesQCOM<'static>;
    pub type VkPhysicalDeviceFragmentDensityMapOffsetPropertiesQCOM =
        PhysicalDeviceFragmentDensityMapOffsetPropertiesQCOM<'static>;
    pub type VkSubpassFragmentDensityMapOffsetEndInfoQCOM =
        SubpassFragmentDensityMapOffsetEndInfoQCOM<'static>;
}
