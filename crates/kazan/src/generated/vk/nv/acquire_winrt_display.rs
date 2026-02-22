#![allow(unused_imports)]
use crate::*;
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
use kazan_sys::{vk::Result as VkResult, vk::*, *};
pub struct InstanceFn {
    acquire_winrt_display_nv: PFN_vkAcquireWinrtDisplayNV,
    get_winrt_display_nv: PFN_vkGetWinrtDisplayNV,
}
impl InstanceFn {
    pub unsafe fn load(
        load: impl Fn(&CStr) -> Option<PFN_vkVoidFunction>,
    ) -> core::result::Result<Self, LoadingError> {
        unsafe {
            Ok(Self {
                acquire_winrt_display_nv: transmute(
                    load(c"vkAcquireWinrtDisplayNV").ok_or(LoadingError)?,
                ),
                get_winrt_display_nv: transmute(load(c"vkGetWinrtDisplayNV").ok_or(LoadingError)?),
            })
        }
    }
}
impl InstanceFn {
    pub unsafe fn acquire_winrt_display_nv(
        &self,
        physical_device: PhysicalDevice,
        display: DisplayKHR,
    ) -> crate::Result<()> {
        unsafe {
            let result = (self.acquire_winrt_display_nv)(physical_device, display);

            match result {
                VkResult::SUCCESS => Ok(()),
                err => Err(err),
            }
        }
    }
    pub unsafe fn get_winrt_display_nv(
        &self,
        physical_device: PhysicalDevice,
        device_relative_id: u32,
    ) -> crate::Result<DisplayKHR> {
        unsafe {
            let mut display = core::mem::MaybeUninit::uninit();
            let result = (self.get_winrt_display_nv)(
                physical_device,
                device_relative_id,
                display.as_mut_ptr(),
            );

            match result {
                VkResult::SUCCESS => Ok(display.assume_init()),
                err => Err(err),
            }
        }
    }
}
