#![allow(unused_imports)]
use crate::*;
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
use kazan_sys::{vk::Result as VkResult, vk::*, *};
pub struct DeviceFn {
    cmd_decode_video_khr: PFN_vkCmdDecodeVideoKHR,
}
impl DeviceFn {
    pub unsafe fn load(
        load: impl Fn(&CStr) -> Option<PFN_vkVoidFunction>,
    ) -> core::result::Result<Self, LoadingError> {
        unsafe {
            Ok(Self {
                cmd_decode_video_khr: transmute(load(c"vkCmdDecodeVideoKHR").ok_or(LoadingError)?),
            })
        }
    }
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
