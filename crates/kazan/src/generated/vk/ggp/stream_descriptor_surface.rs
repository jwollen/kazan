#![allow(unused_imports)]
use crate::*;
use kazan_sys::{vk::*, *};
use std::ffi::{c_char, c_int, c_void, CStr};
pub struct InstanceFn {
    create_stream_descriptor_surface_ggp: PFN_vkCreateStreamDescriptorSurfaceGGP,
}
impl InstanceFn {
    pub unsafe fn create_stream_descriptor_surface_ggp(
        &self,
        instance: Instance,
        create_info: &StreamDescriptorSurfaceCreateInfoGGP,
        allocator: &AllocationCallbacks,
        surface: &mut SurfaceKHR,
    ) -> Result {
        unsafe {
            (self.create_stream_descriptor_surface_ggp)(instance, create_info, allocator, surface)
        }
    }
}
