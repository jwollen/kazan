#![allow(unused_imports)]
use crate::*;
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
use kazan_sys::{vk::*, *};
pub struct InstanceFn {
    create_metal_surface_ext: PFN_vkCreateMetalSurfaceEXT,
}
impl InstanceFn {
    pub unsafe fn load(
        load: impl Fn(&CStr) -> Option<PFN_vkVoidFunction>,
    ) -> core::result::Result<Self, LoadingError> {
        unsafe {
            Ok(Self {
                create_metal_surface_ext: transmute(
                    load(c"vkCreateMetalSurfaceEXT").ok_or(LoadingError)?,
                ),
            })
        }
    }
}
impl InstanceFn {
    pub unsafe fn create_metal_surface_ext(
        &self,
        instance: Instance,
        create_info: &MetalSurfaceCreateInfoEXT,
        allocator: Option<&AllocationCallbacks>,
        surface: &mut SurfaceKHR,
    ) -> Result {
        unsafe {
            (self.create_metal_surface_ext)(instance, create_info, allocator.to_raw_ptr(), surface)
        }
    }
}
