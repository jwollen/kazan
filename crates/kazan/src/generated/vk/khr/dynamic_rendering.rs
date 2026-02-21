#![allow(unused_imports)]
use crate::*;
use core::ffi::{c_char, c_int, c_void, CStr};
use kazan_sys::{vk::*, *};
pub struct DeviceFn {
    cmd_begin_rendering: PFN_vkCmdBeginRendering,
    cmd_end_rendering: PFN_vkCmdEndRendering,
}
impl DeviceFn {
    pub unsafe fn cmd_begin_rendering_khr(
        &self,
        command_buffer: CommandBuffer,
        rendering_info: &RenderingInfo,
    ) {
        unsafe { (self.cmd_begin_rendering)(command_buffer, rendering_info) }
    }
    pub unsafe fn cmd_end_rendering_khr(&self, command_buffer: CommandBuffer) {
        unsafe { (self.cmd_end_rendering)(command_buffer) }
    }
}
