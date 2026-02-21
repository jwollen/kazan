#![allow(unused_imports)]
use crate::*;
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
use kazan_sys::{vk::*, *};
pub struct DeviceFn {
    cmd_decompress_memory_ext: PFN_vkCmdDecompressMemoryEXT,
    cmd_decompress_memory_indirect_count_ext: PFN_vkCmdDecompressMemoryIndirectCountEXT,
}
impl DeviceFn {
    pub unsafe fn load(
        load: impl Fn(&CStr) -> Option<PFN_vkVoidFunction>,
    ) -> core::result::Result<Self, LoadingError> {
        unsafe {
            Ok(Self {
                cmd_decompress_memory_ext: transmute(
                    load(c"vkCmdDecompressMemoryEXT").ok_or(LoadingError)?,
                ),
                cmd_decompress_memory_indirect_count_ext: transmute(
                    load(c"vkCmdDecompressMemoryIndirectCountEXT").ok_or(LoadingError)?,
                ),
            })
        }
    }
}
impl DeviceFn {
    pub unsafe fn cmd_decompress_memory_ext(
        &self,
        command_buffer: CommandBuffer,
        decompress_memory_info_ext: &DecompressMemoryInfoEXT,
    ) {
        unsafe { (self.cmd_decompress_memory_ext)(command_buffer, decompress_memory_info_ext) }
    }
    pub unsafe fn cmd_decompress_memory_indirect_count_ext(
        &self,
        command_buffer: CommandBuffer,
        decompression_method: MemoryDecompressionMethodFlagsEXT,
        indirect_commands_address: DeviceAddress,
        indirect_commands_count_address: DeviceAddress,
        max_decompression_count: u32,
        stride: u32,
    ) {
        unsafe {
            (self.cmd_decompress_memory_indirect_count_ext)(
                command_buffer,
                decompression_method,
                indirect_commands_address,
                indirect_commands_count_address,
                max_decompression_count,
                stride,
            )
        }
    }
}
