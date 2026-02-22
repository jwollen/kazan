#![allow(unused_imports)]
use crate::*;
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
use kazan_sys::{vk::Result as VkResult, vk::*, *};
pub struct DeviceFn {
    cmd_copy_memory_indirect_khr: PFN_vkCmdCopyMemoryIndirectKHR,
    cmd_copy_memory_to_image_indirect_khr: PFN_vkCmdCopyMemoryToImageIndirectKHR,
}
impl DeviceFn {
    pub unsafe fn load(
        load: impl Fn(&CStr) -> Option<PFN_vkVoidFunction>,
    ) -> core::result::Result<Self, LoadingError> {
        unsafe {
            Ok(Self {
                cmd_copy_memory_indirect_khr: transmute(
                    load(c"vkCmdCopyMemoryIndirectKHR").ok_or(LoadingError)?,
                ),
                cmd_copy_memory_to_image_indirect_khr: transmute(
                    load(c"vkCmdCopyMemoryToImageIndirectKHR").ok_or(LoadingError)?,
                ),
            })
        }
    }
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
