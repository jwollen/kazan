#![allow(unused_imports)]
use crate::*;
use kazan_sys::{vk::*, *};
use std::ffi::{c_char, c_int, c_void, CStr};
pub struct InstanceFn {
    release_display_ext: PFN_vkReleaseDisplayEXT,
}
impl InstanceFn {
    pub unsafe fn release_display_ext(
        &self,
        physical_device: PhysicalDevice,
        display: DisplayKHR,
    ) -> Result {
        unsafe { (self.release_display_ext)(physical_device, display) }
    }
}
