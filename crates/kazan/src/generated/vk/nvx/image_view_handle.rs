#![allow(unused_imports)]
use crate::*;
use kazan_sys::{vk::*, *};
use std::ffi::{c_char, c_int, c_void, CStr};
pub struct DeviceFn {
    get_image_view_handle_nvx: PFN_vkGetImageViewHandleNVX,
    get_image_view_handle64_nvx: PFN_vkGetImageViewHandle64NVX,
    get_image_view_address_nvx: PFN_vkGetImageViewAddressNVX,
    get_device_combined_image_sampler_index_nvx: PFN_vkGetDeviceCombinedImageSamplerIndexNVX,
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
        properties: &mut ImageViewAddressPropertiesNVX,
    ) -> Result {
        unsafe { (self.get_image_view_address_nvx)(device, image_view, properties) }
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
