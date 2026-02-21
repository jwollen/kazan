#![allow(unused_imports)]
use crate::*;
use core::ffi::{c_char, c_int, c_void, CStr};
use kazan_sys::{vk::*, *};
pub struct DeviceFn {
    cmd_bind_tile_memory_qcom: PFN_vkCmdBindTileMemoryQCOM,
}
impl DeviceFn {
    pub unsafe fn cmd_bind_tile_memory_qcom(
        &self,
        command_buffer: CommandBuffer,
        tile_memory_bind_info: Option<&TileMemoryBindInfoQCOM>,
    ) {
        unsafe {
            (self.cmd_bind_tile_memory_qcom)(command_buffer, tile_memory_bind_info.to_raw_ptr())
        }
    }
}
