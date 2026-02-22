#![allow(unused_imports)]
use crate::*;
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
use kazan_sys::{vk::Result as VkResult, vk::*, *};
pub struct DeviceFn {
    cmd_decompress_memory_nv: PFN_vkCmdDecompressMemoryNV,
    cmd_decompress_memory_indirect_count_nv: PFN_vkCmdDecompressMemoryIndirectCountNV,
}
impl DeviceFn {
    pub unsafe fn load(
        load: impl Fn(&CStr) -> Option<PFN_vkVoidFunction>,
    ) -> core::result::Result<Self, LoadingError> {
        unsafe {
            Ok(Self {
                cmd_decompress_memory_nv: transmute(
                    load(c"vkCmdDecompressMemoryNV").ok_or(LoadingError)?,
                ),
                cmd_decompress_memory_indirect_count_nv: transmute(
                    load(c"vkCmdDecompressMemoryIndirectCountNV").ok_or(LoadingError)?,
                ),
            })
        }
    }
}
impl DeviceFn {
    pub unsafe fn cmd_decompress_memory_nv(
        &self,
        command_buffer: CommandBuffer,
        decompress_memory_regions: &[DecompressMemoryRegionNV],
    ) {
        unsafe {
            (self.cmd_decompress_memory_nv)(
                command_buffer,
                decompress_memory_regions.len().try_into().unwrap(),
                decompress_memory_regions.as_ptr() as _,
            )
        }
    }
    pub unsafe fn cmd_decompress_memory_indirect_count_nv(
        &self,
        command_buffer: CommandBuffer,
        indirect_commands_address: DeviceAddress,
        indirect_commands_count_address: DeviceAddress,
        stride: u32,
    ) {
        unsafe {
            (self.cmd_decompress_memory_indirect_count_nv)(
                command_buffer,
                indirect_commands_address,
                indirect_commands_count_address,
                stride,
            )
        }
    }
}
