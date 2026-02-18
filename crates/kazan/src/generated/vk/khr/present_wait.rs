#![allow(unused_imports)]
use crate::*;
use kazan_sys::{vk::*, *};
use std::ffi::{c_char, c_int, c_void, CStr};
pub struct DeviceFn {
    wait_for_present_khr: PFN_vkWaitForPresentKHR,
}
impl DeviceFn {
    pub unsafe fn wait_for_present_khr(
        &self,
        device: Device,
        swapchain: SwapchainKHR,
        present_id: u64,
        timeout: u64,
    ) -> Result {
        unsafe { (self.wait_for_present_khr)(device, swapchain, present_id, timeout) }
    }
}
