#![allow(unused_imports)]
use crate::*;
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
use kazan_sys::{vk::*, *};
pub struct DeviceFn {
    cmd_set_color_write_enable_ext: PFN_vkCmdSetColorWriteEnableEXT,
}
impl DeviceFn {
    pub unsafe fn load(
        load: impl Fn(&CStr) -> Option<PFN_vkVoidFunction>,
    ) -> core::result::Result<Self, LoadingError> {
        unsafe {
            Ok(Self {
                cmd_set_color_write_enable_ext: transmute(
                    load(c"vkCmdSetColorWriteEnableEXT").ok_or(LoadingError)?,
                ),
            })
        }
    }
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
