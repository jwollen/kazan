#![allow(unused_imports)]
use crate::*;
use kazan_sys::{vk::*, *};
use std::ffi::{c_char, c_int, c_void, CStr};
pub struct DeviceFn {
    cmd_end_rendering2_khr: PFN_vkCmdEndRendering2KHR,
}
impl DeviceFn {
    pub unsafe fn cmd_end_rendering2_khr(
        &self,
        command_buffer: CommandBuffer,
        rendering_end_info: &RenderingEndInfoKHR,
    ) {
        unsafe { (self.cmd_end_rendering2_khr)(command_buffer, rendering_end_info) }
    }
}
