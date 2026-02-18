#![allow(unused_imports)]
use crate::*;
use kazan_sys::{vk::*, *};
use std::ffi::{c_char, c_int, c_void, CStr};
pub struct DeviceFn {
    cmd_bind_tile_memory_qcom: PFN_vkCmdBindTileMemoryQCOM,
}
impl DeviceFn {
    pub unsafe fn cmd_bind_tile_memory_qcom(
        &self,
        command_buffer: CommandBuffer,
        tile_memory_bind_info: &TileMemoryBindInfoQCOM,
    ) {
        unsafe { (self.cmd_bind_tile_memory_qcom)(command_buffer, tile_memory_bind_info) }
    }
}
