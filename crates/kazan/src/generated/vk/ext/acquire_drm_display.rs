#![allow(unused_imports)]
use crate::*;
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
use kazan_sys::{vk::*, *};
pub struct InstanceFn {
    acquire_drm_display_ext: PFN_vkAcquireDrmDisplayEXT,
    get_drm_display_ext: PFN_vkGetDrmDisplayEXT,
}
impl InstanceFn {
    pub unsafe fn load(
        load: impl Fn(&CStr) -> Option<PFN_vkVoidFunction>,
    ) -> core::result::Result<Self, LoadingError> {
        unsafe {
            Ok(Self {
                acquire_drm_display_ext: transmute(
                    load(c"vkAcquireDrmDisplayEXT").ok_or(LoadingError)?,
                ),
                get_drm_display_ext: transmute(load(c"vkGetDrmDisplayEXT").ok_or(LoadingError)?),
            })
        }
    }
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
