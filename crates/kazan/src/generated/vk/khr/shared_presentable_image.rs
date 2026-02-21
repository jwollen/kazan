#![allow(unused_imports)]
use crate::*;
use core::ffi::{c_char, c_int, c_void, CStr};
use kazan_sys::{vk::*, *};
pub struct DeviceFn {
    get_swapchain_status_khr: PFN_vkGetSwapchainStatusKHR,
}
impl DeviceFn {
    pub unsafe fn get_swapchain_status_khr(
        &self,
        device: Device,
        swapchain: SwapchainKHR,
    ) -> Result {
        unsafe { (self.get_swapchain_status_khr)(device, swapchain) }
    }
}
