#![allow(unused_imports)]
use crate::*;
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
use kazan_sys::{vk::Result as VkResult, vk::*, *};
pub struct DeviceFn {
    get_image_view_handle_nvx: PFN_vkGetImageViewHandleNVX,
    get_image_view_handle64_nvx: PFN_vkGetImageViewHandle64NVX,
    get_image_view_address_nvx: PFN_vkGetImageViewAddressNVX,
    get_device_combined_image_sampler_index_nvx: PFN_vkGetDeviceCombinedImageSamplerIndexNVX,
}
impl DeviceFn {
    pub unsafe fn load(
        load: impl Fn(&CStr) -> Option<PFN_vkVoidFunction>,
    ) -> core::result::Result<Self, LoadingError> {
        unsafe {
            Ok(Self {
                get_image_view_handle_nvx: transmute(
                    load(c"vkGetImageViewHandleNVX").ok_or(LoadingError)?,
                ),
                get_image_view_handle64_nvx: transmute(
                    load(c"vkGetImageViewHandle64NVX").ok_or(LoadingError)?,
                ),
                get_image_view_address_nvx: transmute(
                    load(c"vkGetImageViewAddressNVX").ok_or(LoadingError)?,
                ),
                get_device_combined_image_sampler_index_nvx: transmute(
                    load(c"vkGetDeviceCombinedImageSamplerIndexNVX").ok_or(LoadingError)?,
                ),
            })
        }
    }
}
impl DeviceFn {
    pub unsafe fn get_image_view_handle_nvx(
        &self,
        device: Device,
        info: &ImageViewHandleInfoNVX,
    ) -> u32 {
        unsafe { (self.get_image_view_handle_nvx)(device, info) }
    }
    pub unsafe fn get_image_view_handle64_nvx(
        &self,
        device: Device,
        info: &ImageViewHandleInfoNVX,
    ) -> u64 {
        unsafe { (self.get_image_view_handle64_nvx)(device, info) }
    }
    pub unsafe fn get_image_view_address_nvx(
        &self,
        device: Device,
        image_view: ImageView,
    ) -> crate::Result<ImageViewAddressPropertiesNVX> {
        unsafe {
            let mut properties = core::mem::MaybeUninit::uninit();
            let result =
                (self.get_image_view_address_nvx)(device, image_view, properties.as_mut_ptr());

            match result {
                VkResult::SUCCESS => Ok(properties.assume_init()),
                err => Err(err),
            }
        }
    }
    pub unsafe fn get_device_combined_image_sampler_index_nvx(
        &self,
        device: Device,
        image_view_index: u64,
        sampler_index: u64,
    ) -> u64 {
        unsafe {
            (self.get_device_combined_image_sampler_index_nvx)(
                device,
                image_view_index,
                sampler_index,
            )
        }
    }
}
