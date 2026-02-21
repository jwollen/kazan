#![allow(unused_imports)]
use crate::*;
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
use kazan_sys::{vk::*, *};
pub struct DeviceFn {
    cmd_set_patch_control_points_ext: PFN_vkCmdSetPatchControlPointsEXT,
    cmd_set_rasterizer_discard_enable: PFN_vkCmdSetRasterizerDiscardEnable,
    cmd_set_depth_bias_enable: PFN_vkCmdSetDepthBiasEnable,
    cmd_set_logic_op_ext: PFN_vkCmdSetLogicOpEXT,
    cmd_set_primitive_restart_enable: PFN_vkCmdSetPrimitiveRestartEnable,
}
impl DeviceFn {
    pub unsafe fn load(
        load: impl Fn(&CStr) -> Option<PFN_vkVoidFunction>,
    ) -> core::result::Result<Self, LoadingError> {
        unsafe {
            Ok(Self {
                cmd_set_patch_control_points_ext: transmute(
                    load(c"vkCmdSetPatchControlPointsEXT").ok_or(LoadingError)?,
                ),
                cmd_set_rasterizer_discard_enable: transmute(
                    load(c"vkCmdSetRasterizerDiscardEnableEXT").ok_or(LoadingError)?,
                ),
                cmd_set_depth_bias_enable: transmute(
                    load(c"vkCmdSetDepthBiasEnableEXT").ok_or(LoadingError)?,
                ),
                cmd_set_logic_op_ext: transmute(load(c"vkCmdSetLogicOpEXT").ok_or(LoadingError)?),
                cmd_set_primitive_restart_enable: transmute(
                    load(c"vkCmdSetPrimitiveRestartEnableEXT").ok_or(LoadingError)?,
                ),
            })
        }
    }
}
impl DeviceFn {
    pub unsafe fn cmd_set_patch_control_points_ext(
        &self,
        command_buffer: CommandBuffer,
        patch_control_points: u32,
    ) {
        unsafe { (self.cmd_set_patch_control_points_ext)(command_buffer, patch_control_points) }
    }
    pub unsafe fn cmd_set_rasterizer_discard_enable_ext(
        &self,
        command_buffer: CommandBuffer,
        rasterizer_discard_enable: Bool32,
    ) {
        unsafe {
            (self.cmd_set_rasterizer_discard_enable)(command_buffer, rasterizer_discard_enable)
        }
    }
    pub unsafe fn cmd_set_depth_bias_enable_ext(
        &self,
        command_buffer: CommandBuffer,
        depth_bias_enable: Bool32,
    ) {
        unsafe { (self.cmd_set_depth_bias_enable)(command_buffer, depth_bias_enable) }
    }
    pub unsafe fn cmd_set_logic_op_ext(&self, command_buffer: CommandBuffer, logic_op: LogicOp) {
        unsafe { (self.cmd_set_logic_op_ext)(command_buffer, logic_op) }
    }
    pub unsafe fn cmd_set_primitive_restart_enable_ext(
        &self,
        command_buffer: CommandBuffer,
        primitive_restart_enable: Bool32,
    ) {
        unsafe { (self.cmd_set_primitive_restart_enable)(command_buffer, primitive_restart_enable) }
    }
}
