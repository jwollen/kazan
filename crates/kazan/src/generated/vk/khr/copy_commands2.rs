#![allow(unused_imports)]
use crate::*;
use core::ffi::{c_char, c_int, c_void, CStr};
use kazan_sys::{vk::*, *};
pub struct DeviceFn {
    cmd_copy_buffer2: PFN_vkCmdCopyBuffer2,
    cmd_copy_image2: PFN_vkCmdCopyImage2,
    cmd_copy_buffer_to_image2: PFN_vkCmdCopyBufferToImage2,
    cmd_copy_image_to_buffer2: PFN_vkCmdCopyImageToBuffer2,
    cmd_blit_image2: PFN_vkCmdBlitImage2,
    cmd_resolve_image2: PFN_vkCmdResolveImage2,
}
impl DeviceFn {
    pub unsafe fn cmd_copy_buffer2(
        &self,
        command_buffer: CommandBuffer,
        copy_buffer_info: &CopyBufferInfo2,
    ) {
        unsafe { (self.cmd_copy_buffer2)(command_buffer, copy_buffer_info) }
    }
    pub unsafe fn cmd_copy_image2(
        &self,
        command_buffer: CommandBuffer,
        copy_image_info: &CopyImageInfo2,
    ) {
        unsafe { (self.cmd_copy_image2)(command_buffer, copy_image_info) }
    }
    pub unsafe fn cmd_copy_buffer_to_image2(
        &self,
        command_buffer: CommandBuffer,
        copy_buffer_to_image_info: &CopyBufferToImageInfo2,
    ) {
        unsafe { (self.cmd_copy_buffer_to_image2)(command_buffer, copy_buffer_to_image_info) }
    }
    pub unsafe fn cmd_copy_image_to_buffer2(
        &self,
        command_buffer: CommandBuffer,
        copy_image_to_buffer_info: &CopyImageToBufferInfo2,
    ) {
        unsafe { (self.cmd_copy_image_to_buffer2)(command_buffer, copy_image_to_buffer_info) }
    }
    pub unsafe fn cmd_blit_image2(
        &self,
        command_buffer: CommandBuffer,
        blit_image_info: &BlitImageInfo2,
    ) {
        unsafe { (self.cmd_blit_image2)(command_buffer, blit_image_info) }
    }
    pub unsafe fn cmd_resolve_image2(
        &self,
        command_buffer: CommandBuffer,
        resolve_image_info: &ResolveImageInfo2,
    ) {
        unsafe { (self.cmd_resolve_image2)(command_buffer, resolve_image_info) }
    }
}
