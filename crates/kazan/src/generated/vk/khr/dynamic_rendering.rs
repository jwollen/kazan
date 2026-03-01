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
    pub type PipelineRenderingCreateInfoKHR<'a> = PipelineRenderingCreateInfo<'a>;
    pub type RenderingInfoKHR<'a> = RenderingInfo<'a>;
    pub type RenderingAttachmentInfoKHR<'a> = RenderingAttachmentInfo<'a>;
    pub type PhysicalDeviceDynamicRenderingFeaturesKHR<'a> =
        PhysicalDeviceDynamicRenderingFeatures<'a>;
    pub type CommandBufferInheritanceRenderingInfoKHR<'a> =
        CommandBufferInheritanceRenderingInfo<'a>;
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
    ) -> core::result::Result<Self, LoadingError> {
        unsafe {
            Ok(Self {
                cmd_begin_rendering_khr: transmute(
                    load(c"vkCmdBeginRenderingKHR").ok_or(LoadingError)?,
                ),
                cmd_end_rendering_khr: transmute(
                    load(c"vkCmdEndRenderingKHR").ok_or(LoadingError)?,
                ),
            })
        }
    }
}
impl DeviceFn {
    pub unsafe fn cmd_begin_rendering_khr(
        &self,
        command_buffer: CommandBuffer,
        rendering_info: &RenderingInfo<'_>,
    ) {
        unsafe { (self.cmd_begin_rendering_khr)(command_buffer, rendering_info) }
    }
    pub unsafe fn cmd_end_rendering_khr(&self, command_buffer: CommandBuffer) {
        unsafe { (self.cmd_end_rendering_khr)(command_buffer) }
    }
}
