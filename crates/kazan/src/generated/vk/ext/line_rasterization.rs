//! <https://registry.khronos.org/vulkan/specs/latest/man/html/VK_EXT_line_rasterization.html>
#![allow(unused_imports)]
use crate::{vk::Result as VkResult, vk::*, *};
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
use core::ptr;

pub const EXTENSION_NAME: &CStr = c"VK_EXT_line_rasterization";

pub(super) mod defs {
    #![allow(non_camel_case_types, unused_imports)]
    use crate::{vk::*, *};
    use core::ffi::{CStr, c_char, c_int, c_void};
    use core::fmt;
    use core::marker::PhantomData;
    use core::ptr;

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkLineRasterizationModeEXT.html>
    pub type LineRasterizationModeEXT = LineRasterizationMode;
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPhysicalDeviceLineRasterizationFeaturesEXT.html>
    pub type PhysicalDeviceLineRasterizationFeaturesEXT<'a> =
        PhysicalDeviceLineRasterizationFeatures<'a>;
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPhysicalDeviceLineRasterizationPropertiesEXT.html>
    pub type PhysicalDeviceLineRasterizationPropertiesEXT<'a> =
        PhysicalDeviceLineRasterizationProperties<'a>;
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPipelineRasterizationLineStateCreateInfoEXT.html>
    pub type PipelineRasterizationLineStateCreateInfoEXT<'a> =
        PipelineRasterizationLineStateCreateInfo<'a>;
    pub type PFN_vkCmdSetLineStippleEXT = PFN_vkCmdSetLineStipple;
}

#[cfg(feature = "ffi")]
pub(super) mod ffi {
    #![allow(non_camel_case_types)]
    use super::defs::*;

    pub type VkLineRasterizationModeEXT = LineRasterizationModeEXT;
    pub type VkPhysicalDeviceLineRasterizationFeaturesEXT =
        PhysicalDeviceLineRasterizationFeaturesEXT<'static>;
    pub type VkPhysicalDeviceLineRasterizationPropertiesEXT =
        PhysicalDeviceLineRasterizationPropertiesEXT<'static>;
    pub type VkPipelineRasterizationLineStateCreateInfoEXT =
        PipelineRasterizationLineStateCreateInfoEXT<'static>;
}

pub struct DeviceFn {
    cmd_set_line_stipple_ext: PFN_vkCmdSetLineStipple,
}

impl LoadDeviceFn for DeviceFn {
    unsafe fn load_with(
        load: impl Fn(&CStr) -> Option<PFN_vkVoidFunction>,
    ) -> core::result::Result<Self, MissingEntryPointError> {
        unsafe {
            Ok(Self {
                cmd_set_line_stipple_ext: transmute(
                    load(c"vkCmdSetLineStippleEXT").ok_or(MissingEntryPointError)?,
                ),
            })
        }
    }
}

impl DeviceFn {
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdSetLineStippleEXT.html>
    #[inline]
    pub unsafe fn cmd_set_line_stipple_ext(
        &self,
        command_buffer: CommandBuffer,
        line_stipple_factor: u32,
        line_stipple_pattern: u16,
    ) {
        unsafe {
            (self.cmd_set_line_stipple_ext)(
                command_buffer,
                line_stipple_factor,
                line_stipple_pattern,
            )
        }
    }
}
