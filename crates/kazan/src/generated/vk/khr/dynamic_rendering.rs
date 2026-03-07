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
    pub unsafe fn cmd_begin_rendering_khr(
        &self,
        command_buffer: CommandBuffer,
        rendering_info: &RenderingInfo<'_>,
    ) {
        unsafe { (self.cmd_begin_rendering_khr)(command_buffer, rendering_info) }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdEndRenderingKHR.html>
    pub unsafe fn cmd_end_rendering_khr(&self, command_buffer: CommandBuffer) {
        unsafe { (self.cmd_end_rendering_khr)(command_buffer) }
    }
}
