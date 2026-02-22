#![allow(unused_imports)]
use crate::*;
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
use kazan_sys::{vk::Result as VkResult, vk::*, *};
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
        rendering_info: &RenderingInfo,
    ) {
        unsafe { (self.cmd_begin_rendering_khr)(command_buffer, rendering_info) }
    }
    pub unsafe fn cmd_end_rendering_khr(&self, command_buffer: CommandBuffer) {
        unsafe { (self.cmd_end_rendering_khr)(command_buffer) }
    }
}
