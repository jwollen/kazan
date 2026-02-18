#![allow(unused_imports)]
use crate::*;
use kazan_sys::{vk::*, *};
use std::ffi::{c_char, c_int, c_void, CStr};
pub struct InstanceFn {
    create_android_surface_khr: PFN_vkCreateAndroidSurfaceKHR,
}
impl InstanceFn {
    pub unsafe fn create_android_surface_khr(
        &self,
        instance: Instance,
        create_info: &AndroidSurfaceCreateInfoKHR,
        allocator: &AllocationCallbacks,
        surface: &mut SurfaceKHR,
    ) -> Result {
        unsafe { (self.create_android_surface_khr)(instance, create_info, allocator, surface) }
    }
}
