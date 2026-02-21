#![allow(unused_imports)]
use crate::*;
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
use kazan_sys::{vk::*, *};
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
    ) -> Result {
        unsafe { (self.acquire_winrt_display_nv)(physical_device, display) }
    }
    pub unsafe fn get_winrt_display_nv(
        &self,
        physical_device: PhysicalDevice,
        device_relative_id: u32,
        display: &mut DisplayKHR,
    ) -> Result {
        unsafe { (self.get_winrt_display_nv)(physical_device, device_relative_id, display) }
    }
}
