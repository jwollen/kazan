#![allow(unused_imports)]
use crate::*;
use kazan_sys::{vk::*, *};
use std::ffi::{c_char, c_int, c_void, CStr};
pub struct DeviceFn {
    cmd_dispatch_tile_qcom: PFN_vkCmdDispatchTileQCOM,
    cmd_begin_per_tile_execution_qcom: PFN_vkCmdBeginPerTileExecutionQCOM,
    cmd_end_per_tile_execution_qcom: PFN_vkCmdEndPerTileExecutionQCOM,
}
impl DeviceFn {
    pub unsafe fn cmd_dispatch_tile_qcom(
        &self,
        command_buffer: CommandBuffer,
        dispatch_tile_info: &DispatchTileInfoQCOM,
    ) {
        unsafe { (self.cmd_dispatch_tile_qcom)(command_buffer, dispatch_tile_info) }
    }
    pub unsafe fn cmd_begin_per_tile_execution_qcom(
        &self,
        command_buffer: CommandBuffer,
        per_tile_begin_info: &PerTileBeginInfoQCOM,
    ) {
        unsafe { (self.cmd_begin_per_tile_execution_qcom)(command_buffer, per_tile_begin_info) }
    }
    pub unsafe fn cmd_end_per_tile_execution_qcom(
        &self,
        command_buffer: CommandBuffer,
        per_tile_end_info: &PerTileEndInfoQCOM,
    ) {
        unsafe { (self.cmd_end_per_tile_execution_qcom)(command_buffer, per_tile_end_info) }
    }
}
