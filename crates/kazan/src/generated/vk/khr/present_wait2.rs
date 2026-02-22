#![allow(unused_imports)]
use crate::*;
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
use kazan_sys::{vk::Result as VkResult, vk::*, *};
pub struct DeviceFn {
    wait_for_present2_khr: PFN_vkWaitForPresent2KHR,
}
impl DeviceFn {
    pub unsafe fn load(
        load: impl Fn(&CStr) -> Option<PFN_vkVoidFunction>,
    ) -> core::result::Result<Self, LoadingError> {
        unsafe {
            Ok(Self {
                wait_for_present2_khr: transmute(
                    load(c"vkWaitForPresent2KHR").ok_or(LoadingError)?,
                ),
            })
        }
    }
}
impl DeviceFn {
    pub unsafe fn wait_for_present2_khr(
        &self,
        device: Device,
        swapchain: SwapchainKHR,
        present_wait2_info: &PresentWait2InfoKHR,
    ) -> crate::Result<()> {
        unsafe {
            let result = (self.wait_for_present2_khr)(device, swapchain, present_wait2_info);

            match result {
                VkResult::SUCCESS => Ok(()),
                VkResult::TIMEOUT => Ok(()),
                VkResult::SUBOPTIMAL_KHR => Ok(()),
                err => Err(err),
            }
        }
    }
}
