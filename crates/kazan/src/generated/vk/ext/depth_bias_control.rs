#![allow(unused_imports)]
use crate::*;
use core::ffi::{c_char, c_int, c_void, CStr};
use kazan_sys::{vk::*, *};
pub struct DeviceFn {
    cmd_set_depth_bias2_ext: PFN_vkCmdSetDepthBias2EXT,
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
