#![allow(unused_imports)]
use crate::*;
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
use kazan_sys::{vk::*, *};
pub struct DeviceFn {
    copy_memory_to_image_ext: PFN_vkCopyMemoryToImage,
    copy_image_to_memory_ext: PFN_vkCopyImageToMemory,
    copy_image_to_image_ext: PFN_vkCopyImageToImage,
    transition_image_layout_ext: PFN_vkTransitionImageLayout,
    get_image_subresource_layout2_ext: PFN_vkGetImageSubresourceLayout2,
}
impl DeviceFn {
    pub unsafe fn load(
        load: impl Fn(&CStr) -> Option<PFN_vkVoidFunction>,
    ) -> core::result::Result<Self, LoadingError> {
        unsafe {
            Ok(Self {
                copy_memory_to_image_ext: transmute(
                    load(c"vkCopyMemoryToImageEXT").ok_or(LoadingError)?,
                ),
                copy_image_to_memory_ext: transmute(
                    load(c"vkCopyImageToMemoryEXT").ok_or(LoadingError)?,
                ),
                copy_image_to_image_ext: transmute(
                    load(c"vkCopyImageToImageEXT").ok_or(LoadingError)?,
                ),
                transition_image_layout_ext: transmute(
                    load(c"vkTransitionImageLayoutEXT").ok_or(LoadingError)?,
                ),
                get_image_subresource_layout2_ext: transmute(
                    load(c"vkGetImageSubresourceLayout2EXT").ok_or(LoadingError)?,
                ),
            })
        }
    }
}
impl DeviceFn {
    pub unsafe fn copy_memory_to_image_ext(
        &self,
        device: Device,
        copy_memory_to_image_info: &CopyMemoryToImageInfo,
    ) -> crate::Result<()> {
        unsafe {
            result((self.copy_memory_to_image_ext)(
                device,
                copy_memory_to_image_info,
            ))
        }
    }
    pub unsafe fn copy_image_to_memory_ext(
        &self,
        device: Device,
        copy_image_to_memory_info: &CopyImageToMemoryInfo,
    ) -> crate::Result<()> {
        unsafe {
            result((self.copy_image_to_memory_ext)(
                device,
                copy_image_to_memory_info,
            ))
        }
    }
    pub unsafe fn copy_image_to_image_ext(
        &self,
        device: Device,
        copy_image_to_image_info: &CopyImageToImageInfo,
    ) -> crate::Result<()> {
        unsafe {
            result((self.copy_image_to_image_ext)(
                device,
                copy_image_to_image_info,
            ))
        }
    }
    pub unsafe fn transition_image_layout_ext(
        &self,
        device: Device,
        transitions: &[HostImageLayoutTransitionInfo],
    ) -> crate::Result<()> {
        unsafe {
            result((self.transition_image_layout_ext)(
                device,
                transitions.len().try_into().unwrap(),
                transitions.as_ptr() as _,
            ))
        }
    }
    pub unsafe fn get_image_subresource_layout2_ext(
        &self,
        device: Device,
        image: Image,
        subresource: &ImageSubresource2,
        layout: &mut SubresourceLayout2,
    ) {
        unsafe { (self.get_image_subresource_layout2_ext)(device, image, subresource, layout) }
    }
}
