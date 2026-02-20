#![allow(unused_imports)]
use crate::*;
use kazan_sys::{vk::*, *};
use std::ffi::{c_char, c_int, c_void, CStr};
pub struct InstanceFn {
    create_vi_surface_nn: PFN_vkCreateViSurfaceNN,
}
impl InstanceFn {
    pub unsafe fn create_vi_surface_nn(
        &self,
        instance: Instance,
        create_info: &ViSurfaceCreateInfoNN,
        allocator: Option<&AllocationCallbacks>,
        surface: &mut SurfaceKHR,
    ) -> Result {
        unsafe {
            (self.create_vi_surface_nn)(instance, create_info, allocator.to_raw_ptr(), surface)
        }
    }
}
