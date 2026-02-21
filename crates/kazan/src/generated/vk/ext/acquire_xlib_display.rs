#![allow(unused_imports)]
use crate::*;
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
use kazan_sys::{vk::*, *};
pub struct InstanceFn {
    acquire_xlib_display_ext: PFN_vkAcquireXlibDisplayEXT,
    get_rand_r_output_display_ext: PFN_vkGetRandROutputDisplayEXT,
}
impl InstanceFn {
    pub unsafe fn load(
        load: impl Fn(&CStr) -> Option<PFN_vkVoidFunction>,
    ) -> core::result::Result<Self, LoadingError> {
        unsafe {
            Ok(Self {
                acquire_xlib_display_ext: transmute(
                    load(c"vkAcquireXlibDisplayEXT").ok_or(LoadingError)?,
                ),
                get_rand_r_output_display_ext: transmute(
                    load(c"vkGetRandROutputDisplayEXT").ok_or(LoadingError)?,
                ),
            })
        }
    }
}
impl InstanceFn {
    pub unsafe fn acquire_xlib_display_ext(
        &self,
        physical_device: PhysicalDevice,
        dpy: &mut Display,
        display: DisplayKHR,
    ) -> Result {
        unsafe { (self.acquire_xlib_display_ext)(physical_device, dpy, display) }
    }
    pub unsafe fn get_rand_r_output_display_ext(
        &self,
        physical_device: PhysicalDevice,
        dpy: &mut Display,
        rr_output: RROutput,
        display: &mut DisplayKHR,
    ) -> Result {
        unsafe { (self.get_rand_r_output_display_ext)(physical_device, dpy, rr_output, display) }
    }
}
