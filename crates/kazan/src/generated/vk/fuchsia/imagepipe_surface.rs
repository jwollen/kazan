#![allow(unused_imports)]
use crate::*;
use core::ffi::{c_char, c_int, c_void, CStr};
use kazan_sys::{vk::*, *};
pub struct InstanceFn {
    create_image_pipe_surface_fuchsia: PFN_vkCreateImagePipeSurfaceFUCHSIA,
}
impl InstanceFn {
    pub unsafe fn create_image_pipe_surface_fuchsia(
        &self,
        instance: Instance,
        create_info: &ImagePipeSurfaceCreateInfoFUCHSIA,
        allocator: Option<&AllocationCallbacks>,
        surface: &mut SurfaceKHR,
    ) -> Result {
        unsafe {
            (self.create_image_pipe_surface_fuchsia)(
                instance,
                create_info,
                allocator.to_raw_ptr(),
                surface,
            )
        }
    }
}
