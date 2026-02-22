#![allow(unused_imports)]
use crate::*;
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
use kazan_sys::{vk::Result as VkResult, vk::*, *};
pub struct DeviceFn {
    get_swapchain_status_khr: PFN_vkGetSwapchainStatusKHR,
}
impl DeviceFn {
    pub unsafe fn load(
        load: impl Fn(&CStr) -> Option<PFN_vkVoidFunction>,
    ) -> core::result::Result<Self, LoadingError> {
        unsafe {
            Ok(Self {
                get_swapchain_status_khr: transmute(
                    load(c"vkGetSwapchainStatusKHR").ok_or(LoadingError)?,
                ),
            })
        }
    }
}
impl DeviceFn {
    pub unsafe fn get_swapchain_status_khr(
        &self,
        device: Device,
        swapchain: SwapchainKHR,
    ) -> crate::Result<()> {
        unsafe {
            let result = (self.get_swapchain_status_khr)(device, swapchain);

            match result {
                VkResult::SUCCESS => Ok(()),
                VkResult::SUBOPTIMAL_KHR => Ok(()),
                err => Err(err),
            }
        }
    }
}
