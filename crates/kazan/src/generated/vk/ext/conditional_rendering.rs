#![allow(unused_imports)]
use crate::*;
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
use kazan_sys::{vk::Result as VkResult, vk::*, *};
pub struct DeviceFn {
    cmd_begin_conditional_rendering_ext: PFN_vkCmdBeginConditionalRenderingEXT,
    cmd_end_conditional_rendering_ext: PFN_vkCmdEndConditionalRenderingEXT,
}
impl DeviceFn {
    pub unsafe fn load(
        load: impl Fn(&CStr) -> Option<PFN_vkVoidFunction>,
    ) -> core::result::Result<Self, LoadingError> {
        unsafe {
            Ok(Self {
                cmd_begin_conditional_rendering_ext: transmute(
                    load(c"vkCmdBeginConditionalRenderingEXT").ok_or(LoadingError)?,
                ),
                cmd_end_conditional_rendering_ext: transmute(
                    load(c"vkCmdEndConditionalRenderingEXT").ok_or(LoadingError)?,
                ),
            })
        }
    }
}
impl DeviceFn {
    pub unsafe fn cmd_begin_conditional_rendering_ext(
        &self,
        command_buffer: CommandBuffer,
        conditional_rendering_begin: &ConditionalRenderingBeginInfoEXT,
    ) {
        unsafe {
            (self.cmd_begin_conditional_rendering_ext)(command_buffer, conditional_rendering_begin)
        }
    }
    pub unsafe fn cmd_end_conditional_rendering_ext(&self, command_buffer: CommandBuffer) {
        unsafe { (self.cmd_end_conditional_rendering_ext)(command_buffer) }
    }
}
