#![allow(unused_imports)]
use crate::*;
use core::ffi::{c_char, c_int, c_void, CStr};
use kazan_sys::{vk::*, *};
pub struct InstanceFn {
    create_headless_surface_ext: PFN_vkCreateHeadlessSurfaceEXT,
}
impl InstanceFn {
    pub unsafe fn create_headless_surface_ext(
        &self,
        instance: Instance,
        create_info: &HeadlessSurfaceCreateInfoEXT,
        allocator: Option<&AllocationCallbacks>,
        surface: &mut SurfaceKHR,
    ) -> Result {
        unsafe {
            (self.create_headless_surface_ext)(
                instance,
                create_info,
                allocator.to_raw_ptr(),
                surface,
            )
        }
    }
}
