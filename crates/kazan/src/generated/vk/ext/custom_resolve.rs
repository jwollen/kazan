#![allow(unused_imports)]
use crate::*;
use kazan_sys::{vk::*, *};
use std::ffi::{c_char, c_int, c_void, CStr};
pub struct DeviceFn {
    cmd_begin_custom_resolve_ext: PFN_vkCmdBeginCustomResolveEXT,
}
impl DeviceFn {
    pub unsafe fn cmd_begin_custom_resolve_ext(
        &self,
        command_buffer: CommandBuffer,
        begin_custom_resolve_info: &BeginCustomResolveInfoEXT,
    ) {
        unsafe { (self.cmd_begin_custom_resolve_ext)(command_buffer, begin_custom_resolve_info) }
    }
}
