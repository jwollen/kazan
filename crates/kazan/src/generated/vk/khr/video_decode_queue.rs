#![allow(unused_imports)]
use crate::*;
use kazan_sys::{vk::*, *};
use std::ffi::{c_char, c_int, c_void, CStr};
pub struct DeviceFn {
    cmd_decode_video_khr: PFN_vkCmdDecodeVideoKHR,
}
impl DeviceFn {
    pub unsafe fn cmd_decode_video_khr(
        &self,
        command_buffer: CommandBuffer,
        decode_info: &VideoDecodeInfoKHR,
    ) {
        unsafe { (self.cmd_decode_video_khr)(command_buffer, decode_info) }
    }
}
