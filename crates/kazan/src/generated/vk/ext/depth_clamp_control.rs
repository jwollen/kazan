#![allow(unused_imports)]
use crate::*;
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
use kazan_sys::{vk::Result as VkResult, vk::*, *};
pub struct DeviceFn {
    cmd_set_depth_clamp_range_ext: PFN_vkCmdSetDepthClampRangeEXT,
}
impl DeviceFn {
    pub unsafe fn load(
        load: impl Fn(&CStr) -> Option<PFN_vkVoidFunction>,
    ) -> core::result::Result<Self, LoadingError> {
        unsafe {
            Ok(Self {
                cmd_set_depth_clamp_range_ext: transmute(
                    load(c"vkCmdSetDepthClampRangeEXT").ok_or(LoadingError)?,
                ),
            })
        }
    }
}
impl DeviceFn {
    pub unsafe fn cmd_set_depth_clamp_range_ext(
        &self,
        command_buffer: CommandBuffer,
        depth_clamp_mode: DepthClampModeEXT,
        depth_clamp_range: Option<&DepthClampRangeEXT>,
    ) {
        unsafe {
            (self.cmd_set_depth_clamp_range_ext)(
                command_buffer,
                depth_clamp_mode,
                depth_clamp_range.to_raw_ptr(),
            )
        }
    }
}
