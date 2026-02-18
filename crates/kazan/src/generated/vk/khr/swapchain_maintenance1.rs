#![allow(unused_imports)]
use crate::*;
use kazan_sys::{vk::*, *};
use std::ffi::{c_char, c_int, c_void, CStr};
pub struct DeviceFn {
    release_swapchain_images_khr: PFN_vkReleaseSwapchainImagesKHR,
}
impl DeviceFn {
    pub unsafe fn release_swapchain_images_khr(
        &self,
        device: Device,
        release_info: &ReleaseSwapchainImagesInfoKHR,
    ) -> Result {
        unsafe { (self.release_swapchain_images_khr)(device, release_info) }
    }
}
