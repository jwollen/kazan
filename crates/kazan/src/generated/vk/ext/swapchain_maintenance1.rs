#![allow(unused_imports)]
use crate::*;
use core::ffi::{c_char, c_int, c_void, CStr};
use kazan_sys::{vk::*, *};
pub struct DeviceFn {
    release_swapchain_images_khr: PFN_vkReleaseSwapchainImagesKHR,
}
impl DeviceFn {
    pub unsafe fn release_swapchain_images_ext(
        &self,
        device: Device,
        release_info: &ReleaseSwapchainImagesInfoKHR,
    ) -> Result {
        unsafe { (self.release_swapchain_images_khr)(device, release_info) }
    }
}
