#![allow(unused_imports)]
use crate::*;
use kazan_sys::{vk::*, *};
use std::ffi::{c_char, c_int, c_void, CStr};
pub struct InstanceFn {
    create_metal_surface_ext: PFN_vkCreateMetalSurfaceEXT,
}
impl InstanceFn {
    pub unsafe fn create_metal_surface_ext(
        &self,
        instance: Instance,
        create_info: &MetalSurfaceCreateInfoEXT,
        allocator: &AllocationCallbacks,
        surface: &mut SurfaceKHR,
    ) -> Result {
        unsafe { (self.create_metal_surface_ext)(instance, create_info, allocator, surface) }
    }
}
