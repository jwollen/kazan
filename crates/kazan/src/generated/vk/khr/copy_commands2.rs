#![allow(unused_imports)]
use crate::{vk::Result as VkResult, vk::*, *};
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
pub(super) mod defs {
    #![allow(non_camel_case_types, unused_imports)]
    use crate::{vk::*, *};
    use bitflags::bitflags;
    use core::ffi::{CStr, c_char, c_int, c_void};
    use core::marker::PhantomData;
    pub type BufferCopy2KHR<'a> = BufferCopy2<'a>;
    pub type ImageCopy2KHR<'a> = ImageCopy2<'a>;
    pub type ImageBlit2KHR<'a> = ImageBlit2<'a>;
    pub type BufferImageCopy2KHR<'a> = BufferImageCopy2<'a>;
    pub type ImageResolve2KHR<'a> = ImageResolve2<'a>;
    pub type CopyBufferInfo2KHR<'a> = CopyBufferInfo2<'a>;
    pub type CopyImageInfo2KHR<'a> = CopyImageInfo2<'a>;
    pub type BlitImageInfo2KHR<'a> = BlitImageInfo2<'a>;
    pub type CopyBufferToImageInfo2KHR<'a> = CopyBufferToImageInfo2<'a>;
    pub type CopyImageToBufferInfo2KHR<'a> = CopyImageToBufferInfo2<'a>;
    pub type ResolveImageInfo2KHR<'a> = ResolveImageInfo2<'a>;
    pub type PFN_vkCmdCopyBuffer2KHR = PFN_vkCmdCopyBuffer2;
    pub type PFN_vkCmdCopyImage2KHR = PFN_vkCmdCopyImage2;
    pub type PFN_vkCmdBlitImage2KHR = PFN_vkCmdBlitImage2;
    pub type PFN_vkCmdCopyBufferToImage2KHR = PFN_vkCmdCopyBufferToImage2;
    pub type PFN_vkCmdCopyImageToBuffer2KHR = PFN_vkCmdCopyImageToBuffer2;
    pub type PFN_vkCmdResolveImage2KHR = PFN_vkCmdResolveImage2;
}
pub struct DeviceFn {
    cmd_copy_buffer2_khr: PFN_vkCmdCopyBuffer2,
    cmd_copy_image2_khr: PFN_vkCmdCopyImage2,
    cmd_copy_buffer_to_image2_khr: PFN_vkCmdCopyBufferToImage2,
    cmd_copy_image_to_buffer2_khr: PFN_vkCmdCopyImageToBuffer2,
    cmd_blit_image2_khr: PFN_vkCmdBlitImage2,
    cmd_resolve_image2_khr: PFN_vkCmdResolveImage2,
}
impl DeviceFn {
    pub unsafe fn load(
        load: impl Fn(&CStr) -> Option<PFN_vkVoidFunction>,
    ) -> core::result::Result<Self, LoadingError> {
        unsafe {
            Ok(Self {
                cmd_copy_buffer2_khr: transmute(load(c"vkCmdCopyBuffer2KHR").ok_or(LoadingError)?),
                cmd_copy_image2_khr: transmute(load(c"vkCmdCopyImage2KHR").ok_or(LoadingError)?),
                cmd_copy_buffer_to_image2_khr: transmute(
                    load(c"vkCmdCopyBufferToImage2KHR").ok_or(LoadingError)?,
                ),
                cmd_copy_image_to_buffer2_khr: transmute(
                    load(c"vkCmdCopyImageToBuffer2KHR").ok_or(LoadingError)?,
                ),
                cmd_blit_image2_khr: transmute(load(c"vkCmdBlitImage2KHR").ok_or(LoadingError)?),
                cmd_resolve_image2_khr: transmute(
                    load(c"vkCmdResolveImage2KHR").ok_or(LoadingError)?,
                ),
            })
        }
    }
}
impl DeviceFn {
    pub unsafe fn cmd_copy_buffer2_khr(
        &self,
        command_buffer: CommandBuffer,
        copy_buffer_info: &CopyBufferInfo2<'_>,
    ) {
        unsafe { (self.cmd_copy_buffer2_khr)(command_buffer, copy_buffer_info) }
    }
    pub unsafe fn cmd_copy_image2_khr(
        &self,
        command_buffer: CommandBuffer,
        copy_image_info: &CopyImageInfo2<'_>,
    ) {
        unsafe { (self.cmd_copy_image2_khr)(command_buffer, copy_image_info) }
    }
    pub unsafe fn cmd_copy_buffer_to_image2_khr(
        &self,
        command_buffer: CommandBuffer,
        copy_buffer_to_image_info: &CopyBufferToImageInfo2<'_>,
    ) {
        unsafe { (self.cmd_copy_buffer_to_image2_khr)(command_buffer, copy_buffer_to_image_info) }
    }
    pub unsafe fn cmd_copy_image_to_buffer2_khr(
        &self,
        command_buffer: CommandBuffer,
        copy_image_to_buffer_info: &CopyImageToBufferInfo2<'_>,
    ) {
        unsafe { (self.cmd_copy_image_to_buffer2_khr)(command_buffer, copy_image_to_buffer_info) }
    }
    pub unsafe fn cmd_blit_image2_khr(
        &self,
        command_buffer: CommandBuffer,
        blit_image_info: &BlitImageInfo2<'_>,
    ) {
        unsafe { (self.cmd_blit_image2_khr)(command_buffer, blit_image_info) }
    }
    pub unsafe fn cmd_resolve_image2_khr(
        &self,
        command_buffer: CommandBuffer,
        resolve_image_info: &ResolveImageInfo2<'_>,
    ) {
        unsafe { (self.cmd_resolve_image2_khr)(command_buffer, resolve_image_info) }
    }
}
