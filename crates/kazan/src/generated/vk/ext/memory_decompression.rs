#![allow(unused_imports)]
use crate::*;
use core::ffi::{c_char, c_int, c_void, CStr};
use kazan_sys::{vk::*, *};
pub struct DeviceFn {
    cmd_decompress_memory_ext: PFN_vkCmdDecompressMemoryEXT,
    cmd_decompress_memory_indirect_count_ext: PFN_vkCmdDecompressMemoryIndirectCountEXT,
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
