#![allow(unused_imports)]
use crate::*;
use core::ffi::{c_char, c_int, c_void, CStr};
use kazan_sys::{vk::*, *};
pub struct InstanceFn {
    acquire_xlib_display_ext: PFN_vkAcquireXlibDisplayEXT,
    get_rand_r_output_display_ext: PFN_vkGetRandROutputDisplayEXT,
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
