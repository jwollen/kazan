#![allow(unused_imports)]
use crate::*;
use core::ffi::{c_char, c_int, c_void, CStr};
use kazan_sys::{vk::*, *};
pub struct DeviceFn {
    cmd_begin_custom_resolve_ext: PFN_vkCmdBeginCustomResolveEXT,
}
impl DeviceFn {
    pub unsafe fn cmd_begin_custom_resolve_ext(
        &self,
        command_buffer: CommandBuffer,
        begin_custom_resolve_info: Option<&BeginCustomResolveInfoEXT>,
    ) {
        unsafe {
            (self.cmd_begin_custom_resolve_ext)(
                command_buffer,
                begin_custom_resolve_info.to_raw_ptr(),
            )
        }
    }
}
