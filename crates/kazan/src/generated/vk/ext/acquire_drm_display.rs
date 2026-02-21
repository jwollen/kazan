#![allow(unused_imports)]
use crate::*;
use core::ffi::{c_char, c_int, c_void, CStr};
use kazan_sys::{vk::*, *};
pub struct InstanceFn {
    acquire_drm_display_ext: PFN_vkAcquireDrmDisplayEXT,
    get_drm_display_ext: PFN_vkGetDrmDisplayEXT,
}
impl InstanceFn {
    pub unsafe fn acquire_drm_display_ext(
        &self,
        physical_device: PhysicalDevice,
        drm_fd: i32,
        display: DisplayKHR,
    ) -> Result {
        unsafe { (self.acquire_drm_display_ext)(physical_device, drm_fd, display) }
    }
    pub unsafe fn get_drm_display_ext(
        &self,
        physical_device: PhysicalDevice,
        drm_fd: i32,
        connector_id: u32,
        display: &mut DisplayKHR,
    ) -> Result {
        unsafe { (self.get_drm_display_ext)(physical_device, drm_fd, connector_id, display) }
    }
}
