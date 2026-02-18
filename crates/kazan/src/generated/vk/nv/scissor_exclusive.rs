#![allow(unused_imports)]
use crate::*;
use kazan_sys::{vk::*, *};
use std::ffi::{c_char, c_int, c_void, CStr};
pub struct DeviceFn {
    cmd_set_exclusive_scissor_nv: PFN_vkCmdSetExclusiveScissorNV,
    cmd_set_exclusive_scissor_enable_nv: PFN_vkCmdSetExclusiveScissorEnableNV,
}
impl DeviceFn {
    pub unsafe fn cmd_set_exclusive_scissor_nv(
        &self,
        command_buffer: CommandBuffer,
        first_exclusive_scissor: u32,
        exclusive_scissors: &[Rect2D],
    ) {
        unsafe {
            (self.cmd_set_exclusive_scissor_nv)(
                command_buffer,
                first_exclusive_scissor,
                exclusive_scissors.len().try_into().unwrap(),
                exclusive_scissors.as_ptr() as _,
            )
        }
    }
    pub unsafe fn cmd_set_exclusive_scissor_enable_nv(
        &self,
        command_buffer: CommandBuffer,
        first_exclusive_scissor: u32,
        exclusive_scissor_enables: &[Bool32],
    ) {
        unsafe {
            (self.cmd_set_exclusive_scissor_enable_nv)(
                command_buffer,
                first_exclusive_scissor,
                exclusive_scissor_enables.len().try_into().unwrap(),
                exclusive_scissor_enables.as_ptr() as _,
            )
        }
    }
}
