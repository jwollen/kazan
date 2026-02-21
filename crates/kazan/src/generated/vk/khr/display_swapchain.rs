#![allow(unused_imports)]
use crate::*;
use core::ffi::{c_char, c_int, c_void, CStr};
use kazan_sys::{vk::*, *};
pub struct DeviceFn {
    create_shared_swapchains_khr: PFN_vkCreateSharedSwapchainsKHR,
}
impl DeviceFn {
    pub unsafe fn create_shared_swapchains_khr(
        &self,
        device: Device,
        create_infos: &[SwapchainCreateInfoKHR],
        allocator: Option<&AllocationCallbacks>,
        swapchains: &mut [SwapchainKHR],
    ) -> Result {
        unsafe {
            (self.create_shared_swapchains_khr)(
                device,
                create_infos.len().try_into().unwrap(),
                create_infos.as_ptr() as _,
                allocator.to_raw_ptr(),
                swapchains.as_mut_ptr() as _,
            )
        }
    }
}
