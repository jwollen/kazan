#![allow(unused_imports)]
use crate::*;
use core::ffi::{c_char, c_int, c_void, CStr};
use kazan_sys::{vk::*, *};
pub struct DeviceFn {
    cmd_end_rendering2_khr: PFN_vkCmdEndRendering2KHR,
}
impl DeviceFn {
    pub unsafe fn cmd_end_rendering2_ext(
        &self,
        command_buffer: CommandBuffer,
        rendering_end_info: Option<&RenderingEndInfoKHR>,
    ) {
        unsafe { (self.cmd_end_rendering2_khr)(command_buffer, rendering_end_info.to_raw_ptr()) }
    }
}
