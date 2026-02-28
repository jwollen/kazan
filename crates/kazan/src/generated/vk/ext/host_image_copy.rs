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
    pub type PhysicalDeviceHostImageCopyFeaturesEXT<'a> = PhysicalDeviceHostImageCopyFeatures<'a>;
    pub type PhysicalDeviceHostImageCopyPropertiesEXT<'a> =
        PhysicalDeviceHostImageCopyProperties<'a>;
    pub type MemoryToImageCopyEXT<'a> = MemoryToImageCopy<'a>;
    pub type ImageToMemoryCopyEXT<'a> = ImageToMemoryCopy<'a>;
    pub type CopyMemoryToImageInfoEXT<'a> = CopyMemoryToImageInfo<'a>;
    pub type CopyImageToMemoryInfoEXT<'a> = CopyImageToMemoryInfo<'a>;
    pub type CopyImageToImageInfoEXT<'a> = CopyImageToImageInfo<'a>;
    pub type HostImageLayoutTransitionInfoEXT<'a> = HostImageLayoutTransitionInfo<'a>;
    pub type SubresourceHostMemcpySizeEXT<'a> = SubresourceHostMemcpySize<'a>;
    pub type HostImageCopyDevicePerformanceQueryEXT<'a> = HostImageCopyDevicePerformanceQuery<'a>;
    pub type ImageSubresource2EXT<'a> = ImageSubresource2<'a>;
    pub type SubresourceLayout2EXT<'a> = SubresourceLayout2<'a>;
    pub type HostImageCopyFlagsEXT = HostImageCopyFlags;
    pub type PFN_vkCopyMemoryToImageEXT = PFN_vkCopyMemoryToImage;
    pub type PFN_vkCopyImageToMemoryEXT = PFN_vkCopyImageToMemory;
    pub type PFN_vkCopyImageToImageEXT = PFN_vkCopyImageToImage;
    pub type PFN_vkTransitionImageLayoutEXT = PFN_vkTransitionImageLayout;
    pub type PFN_vkGetImageSubresourceLayout2EXT = PFN_vkGetImageSubresourceLayout2;
}
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
        copy_memory_to_image_info: &CopyMemoryToImageInfo<'_>,
    ) -> crate::Result<()> {
        unsafe {
            let result = (self.copy_memory_to_image_ext)(device, copy_memory_to_image_info);

            match result {
                VkResult::SUCCESS => Ok(()),
                err => Err(err),
            }
        }
    }
    pub unsafe fn copy_image_to_memory_ext(
        &self,
        device: Device,
        copy_image_to_memory_info: &CopyImageToMemoryInfo<'_>,
    ) -> crate::Result<()> {
        unsafe {
            let result = (self.copy_image_to_memory_ext)(device, copy_image_to_memory_info);

            match result {
                VkResult::SUCCESS => Ok(()),
                err => Err(err),
            }
        }
    }
    pub unsafe fn copy_image_to_image_ext(
        &self,
        device: Device,
        copy_image_to_image_info: &CopyImageToImageInfo<'_>,
    ) -> crate::Result<()> {
        unsafe {
            let result = (self.copy_image_to_image_ext)(device, copy_image_to_image_info);

            match result {
                VkResult::SUCCESS => Ok(()),
                err => Err(err),
            }
        }
    }
    pub unsafe fn transition_image_layout_ext(
        &self,
        device: Device,
        transitions: &[HostImageLayoutTransitionInfo<'_>],
    ) -> crate::Result<()> {
        unsafe {
            let result = (self.transition_image_layout_ext)(
                device,
                transitions.len().try_into().unwrap(),
                transitions.as_ptr() as _,
            );

            match result {
                VkResult::SUCCESS => Ok(()),
                err => Err(err),
            }
        }
    }
    pub unsafe fn get_image_subresource_layout2_ext(
        &self,
        device: Device,
        image: Image,
        subresource: &ImageSubresource2<'_>,
    ) -> SubresourceLayout2<'_> {
        unsafe {
            let mut layout = core::mem::MaybeUninit::uninit();
            (self.get_image_subresource_layout2_ext)(
                device,
                image,
                subresource,
                layout.as_mut_ptr(),
            );
            layout.assume_init()
        }
    }
}
