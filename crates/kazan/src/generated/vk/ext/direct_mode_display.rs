#![allow(unused_imports)]
use crate::*;
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
use kazan_sys::{vk::Result as VkResult, vk::*, *};
pub struct InstanceFn {
    release_display_ext: PFN_vkReleaseDisplayEXT,
}
impl InstanceFn {
    pub unsafe fn load(
        load: impl Fn(&CStr) -> Option<PFN_vkVoidFunction>,
    ) -> core::result::Result<Self, LoadingError> {
        unsafe {
            Ok(Self {
                release_display_ext: transmute(load(c"vkReleaseDisplayEXT").ok_or(LoadingError)?),
            })
        }
    }
}
impl InstanceFn {
    pub unsafe fn release_display_ext(
        &self,
        physical_device: PhysicalDevice,
        display: DisplayKHR,
    ) -> crate::Result<()> {
        unsafe {
            let result = (self.release_display_ext)(physical_device, display);

            match result {
                VkResult::SUCCESS => Ok(()),
                err => Err(err),
            }
        }
    }
}
