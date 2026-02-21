#![allow(unused_imports)]
use crate::*;
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
use kazan_sys::{vk::*, *};
pub struct DeviceFn {
    cmd_begin_custom_resolve_ext: Option<PFN_vkCmdBeginCustomResolveEXT>,
}
impl DeviceFn {
    pub unsafe fn load(
        load: impl Fn(&CStr) -> Option<PFN_vkVoidFunction>,
    ) -> core::result::Result<Self, LoadingError> {
        unsafe {
            Ok(Self {
                cmd_begin_custom_resolve_ext: transmute(load(c"vkCmdBeginCustomResolveEXT")),
            })
        }
    }
}
impl DeviceFn {
    pub unsafe fn cmd_begin_custom_resolve_ext(
        &self,
        command_buffer: CommandBuffer,
        begin_custom_resolve_info: Option<&BeginCustomResolveInfoEXT>,
    ) {
        unsafe {
            (self.cmd_begin_custom_resolve_ext.unwrap())(
                command_buffer,
                begin_custom_resolve_info.to_raw_ptr(),
            )
        }
    }
}
