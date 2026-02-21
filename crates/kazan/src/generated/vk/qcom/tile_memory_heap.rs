#![allow(unused_imports)]
use crate::*;
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
use kazan_sys::{vk::*, *};
pub struct DeviceFn {
    cmd_bind_tile_memory_qcom: PFN_vkCmdBindTileMemoryQCOM,
}
impl DeviceFn {
    pub unsafe fn load(
        load: impl Fn(&CStr) -> Option<PFN_vkVoidFunction>,
    ) -> core::result::Result<Self, LoadingError> {
        unsafe {
            Ok(Self {
                cmd_bind_tile_memory_qcom: transmute(
                    load(c"vkCmdBindTileMemoryQCOM").ok_or(LoadingError)?,
                ),
            })
        }
    }
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
