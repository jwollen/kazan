#![allow(unused_imports)]
use crate::*;
use core::ffi::{c_char, c_int, c_void, CStr};
use kazan_sys::{vk::*, *};
pub struct InstanceFn {
    create_android_surface_khr: PFN_vkCreateAndroidSurfaceKHR,
}
impl InstanceFn {
    pub unsafe fn create_android_surface_khr(
        &self,
        instance: Instance,
        create_info: &AndroidSurfaceCreateInfoKHR,
        allocator: Option<&AllocationCallbacks>,
        surface: &mut SurfaceKHR,
    ) -> Result {
        unsafe {
            (self.create_android_surface_khr)(
                instance,
                create_info,
                allocator.to_raw_ptr(),
                surface,
            )
        }
    }
}
