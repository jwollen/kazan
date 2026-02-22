#![allow(unused_imports)]
use crate::*;
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
use kazan_sys::{vk::*, *};
pub struct DeviceFn {
    release_swapchain_images_ext: PFN_vkReleaseSwapchainImagesKHR,
}
impl DeviceFn {
    pub unsafe fn load(
        load: impl Fn(&CStr) -> Option<PFN_vkVoidFunction>,
    ) -> core::result::Result<Self, LoadingError> {
        unsafe {
            Ok(Self {
                release_swapchain_images_ext: transmute(
                    load(c"vkReleaseSwapchainImagesEXT").ok_or(LoadingError)?,
                ),
            })
        }
    }
}
impl DeviceFn {
    pub unsafe fn release_swapchain_images_ext(
        &self,
        device: Device,
        release_info: &ReleaseSwapchainImagesInfoKHR,
    ) -> crate::Result<()> {
        unsafe { result((self.release_swapchain_images_ext)(device, release_info)) }
    }
}
