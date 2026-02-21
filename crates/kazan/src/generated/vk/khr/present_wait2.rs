#![allow(unused_imports)]
use crate::*;
use core::ffi::{c_char, c_int, c_void, CStr};
use kazan_sys::{vk::*, *};
pub struct DeviceFn {
    wait_for_present2_khr: PFN_vkWaitForPresent2KHR,
}
impl DeviceFn {
    pub unsafe fn wait_for_present2_khr(
        &self,
        device: Device,
        swapchain: SwapchainKHR,
        present_wait2_info: &PresentWait2InfoKHR,
    ) -> Result {
        unsafe { (self.wait_for_present2_khr)(device, swapchain, present_wait2_info) }
    }
}
