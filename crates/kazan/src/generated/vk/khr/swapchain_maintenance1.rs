#![allow(unused_imports)]
use crate::*;
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
use kazan_sys::{vk::Result as VkResult, vk::*, *};
pub struct DeviceFn {
    release_swapchain_images_khr: PFN_vkReleaseSwapchainImagesKHR,
}
impl DeviceFn {
    pub unsafe fn load(
        load: impl Fn(&CStr) -> Option<PFN_vkVoidFunction>,
    ) -> core::result::Result<Self, LoadingError> {
        unsafe {
            Ok(Self {
                release_swapchain_images_khr: transmute(
                    load(c"vkReleaseSwapchainImagesKHR").ok_or(LoadingError)?,
                ),
            })
        }
    }
}
impl DeviceFn {
    pub unsafe fn release_swapchain_images_khr(
        &self,
        device: Device,
        release_info: &ReleaseSwapchainImagesInfoKHR,
    ) -> crate::Result<()> {
        unsafe {
            let result = (self.release_swapchain_images_khr)(device, release_info);

            match result {
                VkResult::SUCCESS => Ok(()),
                err => Err(err),
            }
        }
    }
}
