#![allow(unused_imports)]
use crate::*;
use kazan_sys::{vk::*, *};
use std::ffi::{c_char, c_int, c_void, CStr};
pub struct DeviceFn {
    cmd_copy_memory_indirect_khr: PFN_vkCmdCopyMemoryIndirectKHR,
    cmd_copy_memory_to_image_indirect_khr: PFN_vkCmdCopyMemoryToImageIndirectKHR,
}
impl DeviceFn {
    pub unsafe fn cmd_copy_memory_indirect_khr(
        &self,
        command_buffer: CommandBuffer,
        copy_memory_indirect_info: &CopyMemoryIndirectInfoKHR,
    ) {
        unsafe { (self.cmd_copy_memory_indirect_khr)(command_buffer, copy_memory_indirect_info) }
    }
    pub unsafe fn cmd_copy_memory_to_image_indirect_khr(
        &self,
        command_buffer: CommandBuffer,
        copy_memory_to_image_indirect_info: &CopyMemoryToImageIndirectInfoKHR,
    ) {
        unsafe {
            (self.cmd_copy_memory_to_image_indirect_khr)(
                command_buffer,
                copy_memory_to_image_indirect_info,
            )
        }
    }
}
