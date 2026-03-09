//! <https://registry.khronos.org/vulkan/specs/latest/man/html/VK_KHR_dynamic_rendering.html>
#![allow(unused_imports)]
use crate::{vk::Result as VkResult, vk::*, *};
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
use core::ptr;

pub const EXTENSION_NAME: &CStr = c"VK_KHR_dynamic_rendering";

pub(super) mod defs {
    #![allow(non_camel_case_types, unused_imports)]
    use crate::{vk::*, *};
    use core::ffi::{CStr, c_char, c_int, c_void};
    use core::fmt;
    use core::marker::PhantomData;
    use core::ptr;

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPipelineRenderingCreateInfoKHR.html>
    pub type PipelineRenderingCreateInfoKHR<'a> = PipelineRenderingCreateInfo<'a>;
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkRenderingInfoKHR.html>
    pub type RenderingInfoKHR<'a> = RenderingInfo<'a>;
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkRenderingAttachmentInfoKHR.html>
    pub type RenderingAttachmentInfoKHR<'a> = RenderingAttachmentInfo<'a>;
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPhysicalDeviceDynamicRenderingFeaturesKHR.html>
    pub type PhysicalDeviceDynamicRenderingFeaturesKHR<'a> =
        PhysicalDeviceDynamicRenderingFeatures<'a>;
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkCommandBufferInheritanceRenderingInfoKHR.html>
    pub type CommandBufferInheritanceRenderingInfoKHR<'a> =
        CommandBufferInheritanceRenderingInfo<'a>;
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkRenderingFlagsKHR.html>
    pub type RenderingFlagsKHR = RenderingFlags;
    pub type PFN_vkCmdBeginRenderingKHR = PFN_vkCmdBeginRendering;
    pub type PFN_vkCmdEndRenderingKHR = PFN_vkCmdEndRendering;
}

#[cfg(feature = "ffi")]
pub(super) mod ffi {
    #![allow(non_camel_case_types)]
    use super::defs::*;

    pub type VkPipelineRenderingCreateInfoKHR = PipelineRenderingCreateInfoKHR<'static>;
    pub type VkRenderingInfoKHR = RenderingInfoKHR<'static>;
    pub type VkRenderingAttachmentInfoKHR = RenderingAttachmentInfoKHR<'static>;
    pub type VkPhysicalDeviceDynamicRenderingFeaturesKHR =
        PhysicalDeviceDynamicRenderingFeaturesKHR<'static>;
    pub type VkCommandBufferInheritanceRenderingInfoKHR =
        CommandBufferInheritanceRenderingInfoKHR<'static>;
    pub type VkRenderingFlagsKHR = RenderingFlagsKHR;
}

pub struct DeviceFn {
    cmd_begin_rendering_khr: PFN_vkCmdBeginRendering,
    cmd_end_rendering_khr: PFN_vkCmdEndRendering,
}

impl DeviceFn {
    pub unsafe fn load(
        load: impl Fn(&CStr) -> Option<PFN_vkVoidFunction>,
    ) -> core::result::Result<Self, MissingEntryPointError> {
        unsafe {
            Ok(Self {
                cmd_begin_rendering_khr: transmute(
                    load(c"vkCmdBeginRenderingKHR").ok_or(MissingEntryPointError)?,
                ),
                cmd_end_rendering_khr: transmute(
                    load(c"vkCmdEndRenderingKHR").ok_or(MissingEntryPointError)?,
                ),
            })
        }
    }
}

impl DeviceFn {
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdBeginRenderingKHR.html>
    #[inline]
    pub unsafe fn cmd_begin_rendering_khr(
        &self,
        command_buffer: CommandBuffer,
        rendering_info: &RenderingInfo<'_>,
    ) {
        unsafe { (self.cmd_begin_rendering_khr)(command_buffer, rendering_info) }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdEndRenderingKHR.html>
    #[inline]
    pub unsafe fn cmd_end_rendering_khr(&self, command_buffer: CommandBuffer) {
        unsafe { (self.cmd_end_rendering_khr)(command_buffer) }
    }
}
