#![allow(unused_imports)]
use crate::*;
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
use kazan_sys::{vk::Result as VkResult, vk::*, *};
pub struct DeviceFn {
    cmd_dispatch_tile_qcom: PFN_vkCmdDispatchTileQCOM,
    cmd_begin_per_tile_execution_qcom: PFN_vkCmdBeginPerTileExecutionQCOM,
    cmd_end_per_tile_execution_qcom: PFN_vkCmdEndPerTileExecutionQCOM,
}
impl DeviceFn {
    pub unsafe fn load(
        load: impl Fn(&CStr) -> Option<PFN_vkVoidFunction>,
    ) -> core::result::Result<Self, LoadingError> {
        unsafe {
            Ok(Self {
                cmd_dispatch_tile_qcom: transmute(
                    load(c"vkCmdDispatchTileQCOM").ok_or(LoadingError)?,
                ),
                cmd_begin_per_tile_execution_qcom: transmute(
                    load(c"vkCmdBeginPerTileExecutionQCOM").ok_or(LoadingError)?,
                ),
                cmd_end_per_tile_execution_qcom: transmute(
                    load(c"vkCmdEndPerTileExecutionQCOM").ok_or(LoadingError)?,
                ),
            })
        }
    }
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
