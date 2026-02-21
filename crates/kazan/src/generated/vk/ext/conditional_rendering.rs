#![allow(unused_imports)]
use crate::*;
use core::ffi::{c_char, c_int, c_void, CStr};
use kazan_sys::{vk::*, *};
pub struct DeviceFn {
    cmd_begin_conditional_rendering_ext: PFN_vkCmdBeginConditionalRenderingEXT,
    cmd_end_conditional_rendering_ext: PFN_vkCmdEndConditionalRenderingEXT,
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
