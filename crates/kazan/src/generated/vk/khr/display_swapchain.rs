#![allow(unused_imports)]
use crate::*;
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
use kazan_sys::{vk::Result as VkResult, vk::*, *};
pub struct DeviceFn {
    create_shared_swapchains_khr: PFN_vkCreateSharedSwapchainsKHR,
}
impl DeviceFn {
    pub unsafe fn load(
        load: impl Fn(&CStr) -> Option<PFN_vkVoidFunction>,
    ) -> core::result::Result<Self, LoadingError> {
        unsafe {
            Ok(Self {
                create_shared_swapchains_khr: transmute(
                    load(c"vkCreateSharedSwapchainsKHR").ok_or(LoadingError)?,
                ),
            })
        }
    }
}
impl DeviceFn {
    pub unsafe fn create_shared_swapchains_khr(
        &self,
        device: Device,
        create_infos: &[SwapchainCreateInfoKHR],
        allocator: Option<&AllocationCallbacks>,
        swapchains: &mut [SwapchainKHR],
    ) -> crate::Result<()> {
        unsafe {
            let result = (self.create_shared_swapchains_khr)(
                device,
                create_infos.len().try_into().unwrap(),
                create_infos.as_ptr() as _,
                allocator.to_raw_ptr(),
                swapchains.as_mut_ptr() as _,
            );

            match result {
                VkResult::SUCCESS => Ok(()),
                err => Err(err),
            }
        }
    }
}
