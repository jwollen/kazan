#![allow(unused_imports)]
use crate::*;
use core::ffi::{c_char, c_int, c_void, CStr};
use kazan_sys::{vk::*, *};
pub struct InstanceFn {
    create_stream_descriptor_surface_ggp: PFN_vkCreateStreamDescriptorSurfaceGGP,
}
impl InstanceFn {
    pub unsafe fn create_stream_descriptor_surface_ggp(
        &self,
        instance: Instance,
        create_info: &StreamDescriptorSurfaceCreateInfoGGP,
        allocator: Option<&AllocationCallbacks>,
        surface: &mut SurfaceKHR,
    ) -> Result {
        unsafe {
            (self.create_stream_descriptor_surface_ggp)(
                instance,
                create_info,
                allocator.to_raw_ptr(),
                surface,
            )
        }
    }
}
