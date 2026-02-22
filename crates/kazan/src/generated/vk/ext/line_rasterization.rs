#![allow(unused_imports)]
use crate::*;
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
use kazan_sys::{vk::Result as VkResult, vk::*, *};
pub struct DeviceFn {
    cmd_set_line_stipple_ext: PFN_vkCmdSetLineStipple,
}
impl DeviceFn {
    pub unsafe fn load(
        load: impl Fn(&CStr) -> Option<PFN_vkVoidFunction>,
    ) -> core::result::Result<Self, LoadingError> {
        unsafe {
            Ok(Self {
                cmd_set_line_stipple_ext: transmute(
                    load(c"vkCmdSetLineStippleEXT").ok_or(LoadingError)?,
                ),
            })
        }
    }
}
impl DeviceFn {
    pub unsafe fn cmd_set_line_stipple_ext(
        &self,
        command_buffer: CommandBuffer,
        line_stipple_factor: u32,
        line_stipple_pattern: u16,
    ) {
        unsafe {
            (self.cmd_set_line_stipple_ext)(
                command_buffer,
                line_stipple_factor,
                line_stipple_pattern,
            )
        }
    }
}
