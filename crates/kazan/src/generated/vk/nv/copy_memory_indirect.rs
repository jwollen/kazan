#![allow(unused_imports)]
use crate::*;
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
use kazan_sys::{vk::*, *};
pub struct DeviceFn {
    cmd_copy_memory_indirect_nv: PFN_vkCmdCopyMemoryIndirectNV,
    cmd_copy_memory_to_image_indirect_nv: PFN_vkCmdCopyMemoryToImageIndirectNV,
}
impl DeviceFn {
    pub unsafe fn load(
        load: impl Fn(&CStr) -> Option<PFN_vkVoidFunction>,
    ) -> core::result::Result<Self, LoadingError> {
        unsafe {
            Ok(Self {
                cmd_copy_memory_indirect_nv: transmute(
                    load(c"vkCmdCopyMemoryIndirectNV").ok_or(LoadingError)?,
                ),
                cmd_copy_memory_to_image_indirect_nv: transmute(
                    load(c"vkCmdCopyMemoryToImageIndirectNV").ok_or(LoadingError)?,
                ),
            })
        }
    }
}
impl DeviceFn {
    pub unsafe fn cmd_copy_memory_indirect_nv(
        &self,
        command_buffer: CommandBuffer,
        copy_buffer_address: DeviceAddress,
        copy_count: u32,
        stride: u32,
    ) {
        unsafe {
            (self.cmd_copy_memory_indirect_nv)(
                command_buffer,
                copy_buffer_address,
                copy_count,
                stride,
            )
        }
    }
    pub unsafe fn cmd_copy_memory_to_image_indirect_nv(
        &self,
        command_buffer: CommandBuffer,
        copy_buffer_address: DeviceAddress,
        stride: u32,
        dst_image: Image,
        dst_image_layout: ImageLayout,
        image_subresources: &[ImageSubresourceLayers],
    ) {
        unsafe {
            (self.cmd_copy_memory_to_image_indirect_nv)(
                command_buffer,
                copy_buffer_address,
                image_subresources.len().try_into().unwrap(),
                stride,
                dst_image,
                dst_image_layout,
                image_subresources.as_ptr() as _,
            )
        }
    }
}
