#![allow(unused_imports)]
use crate::*;
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
use kazan_sys::{vk::Result as VkResult, vk::*, *};
pub struct DeviceFn {
    cmd_set_viewport_w_scaling_nv: PFN_vkCmdSetViewportWScalingNV,
}
impl DeviceFn {
    pub unsafe fn load(
        load: impl Fn(&CStr) -> Option<PFN_vkVoidFunction>,
    ) -> core::result::Result<Self, LoadingError> {
        unsafe {
            Ok(Self {
                cmd_set_viewport_w_scaling_nv: transmute(
                    load(c"vkCmdSetViewportWScalingNV").ok_or(LoadingError)?,
                ),
            })
        }
    }
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
