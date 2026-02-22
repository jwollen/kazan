#![allow(unused_imports)]
use crate::*;
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
use kazan_sys::{vk::*, *};
pub struct DeviceFn {
    wait_for_present_khr: PFN_vkWaitForPresentKHR,
}
impl DeviceFn {
    pub unsafe fn load(
        load: impl Fn(&CStr) -> Option<PFN_vkVoidFunction>,
    ) -> core::result::Result<Self, LoadingError> {
        unsafe {
            Ok(Self {
                wait_for_present_khr: transmute(load(c"vkWaitForPresentKHR").ok_or(LoadingError)?),
            })
        }
    }
}
impl DeviceFn {
    pub unsafe fn wait_for_present_khr(
        &self,
        device: Device,
        swapchain: SwapchainKHR,
        present_id: u64,
        timeout: u64,
    ) -> crate::Result<()> {
        unsafe {
            result((self.wait_for_present_khr)(
                device, swapchain, present_id, timeout,
            ))
        }
    }
}
