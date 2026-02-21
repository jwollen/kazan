#![allow(unused_imports)]
use crate::*;
use core::ffi::{c_char, c_int, c_void, CStr};
use kazan_sys::{vk::*, *};
pub struct DeviceFn {
    cmd_set_viewport_w_scaling_nv: PFN_vkCmdSetViewportWScalingNV,
}
impl DeviceFn {
    pub unsafe fn cmd_set_viewport_w_scaling_nv(
        &self,
        command_buffer: CommandBuffer,
        first_viewport: u32,
        viewport_w_scalings: &[ViewportWScalingNV],
    ) {
        unsafe {
            (self.cmd_set_viewport_w_scaling_nv)(
                command_buffer,
                first_viewport,
                viewport_w_scalings.len().try_into().unwrap(),
                viewport_w_scalings.as_ptr() as _,
            )
        }
    }
}
