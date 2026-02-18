#![allow(unused_imports)]
use crate::*;
use kazan_sys::{vk::*, *};
use std::ffi::{c_char, c_int, c_void, CStr};
pub struct InstanceFn {
    create_headless_surface_ext: PFN_vkCreateHeadlessSurfaceEXT,
}
impl InstanceFn {
    pub unsafe fn create_headless_surface_ext(
        &self,
        instance: Instance,
        create_info: &HeadlessSurfaceCreateInfoEXT,
        allocator: &AllocationCallbacks,
        surface: &mut SurfaceKHR,
    ) -> Result {
        unsafe { (self.create_headless_surface_ext)(instance, create_info, allocator, surface) }
    }
}
