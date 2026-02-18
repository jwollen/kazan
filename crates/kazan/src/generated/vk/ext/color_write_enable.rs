#![allow(unused_imports)]
use crate::*;
use kazan_sys::{vk::*, *};
use std::ffi::{c_char, c_int, c_void, CStr};
pub struct DeviceFn {
    cmd_set_color_write_enable_ext: PFN_vkCmdSetColorWriteEnableEXT,
}
impl DeviceFn {
    pub unsafe fn cmd_set_color_write_enable_ext(
        &self,
        command_buffer: CommandBuffer,
        color_write_enables: &[Bool32],
    ) {
        unsafe {
            (self.cmd_set_color_write_enable_ext)(
                command_buffer,
                color_write_enables.len().try_into().unwrap(),
                color_write_enables.as_ptr() as _,
            )
        }
    }
}
