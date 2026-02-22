#![allow(unused_imports)]
use crate::*;
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
use kazan_sys::{vk::Result as VkResult, vk::*, *};
pub struct DeviceFn {
    cmd_set_depth_bias2_ext: PFN_vkCmdSetDepthBias2EXT,
}
impl DeviceFn {
    pub unsafe fn load(
        load: impl Fn(&CStr) -> Option<PFN_vkVoidFunction>,
    ) -> core::result::Result<Self, LoadingError> {
        unsafe {
            Ok(Self {
                cmd_set_depth_bias2_ext: transmute(
                    load(c"vkCmdSetDepthBias2EXT").ok_or(LoadingError)?,
                ),
            })
        }
    }
}
impl DeviceFn {
    pub unsafe fn cmd_set_depth_bias2_ext(
        &self,
        command_buffer: CommandBuffer,
        depth_bias_info: &DepthBiasInfoEXT,
    ) {
        unsafe { (self.cmd_set_depth_bias2_ext)(command_buffer, depth_bias_info) }
    }
}
