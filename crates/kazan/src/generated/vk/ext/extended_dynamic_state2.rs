#![allow(unused_imports)]
use crate::*;
use core::ffi::{c_char, c_int, c_void, CStr};
use kazan_sys::{vk::*, *};
pub struct DeviceFn {
    cmd_set_patch_control_points_ext: PFN_vkCmdSetPatchControlPointsEXT,
    cmd_set_logic_op_ext: PFN_vkCmdSetLogicOpEXT,
}
impl DeviceFn {
    pub unsafe fn cmd_set_patch_control_points_ext(
        &self,
        command_buffer: CommandBuffer,
        patch_control_points: u32,
    ) {
        unsafe { (self.cmd_set_patch_control_points_ext)(command_buffer, patch_control_points) }
    }
    pub unsafe fn cmd_set_logic_op_ext(&self, command_buffer: CommandBuffer, logic_op: LogicOp) {
        unsafe { (self.cmd_set_logic_op_ext)(command_buffer, logic_op) }
    }
}
