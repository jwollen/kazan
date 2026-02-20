#![allow(unused_imports)]
use crate::*;
use kazan_sys::{vk::*, *};
use std::ffi::{c_char, c_int, c_void, CStr};
pub struct InstanceFn {
    create_surface_ohos: PFN_vkCreateSurfaceOHOS,
}
impl InstanceFn {
    pub unsafe fn create_surface_ohos(
        &self,
        instance: Instance,
        create_info: &SurfaceCreateInfoOHOS,
        allocator: Option<&AllocationCallbacks>,
        surface: &mut SurfaceKHR,
    ) -> Result {
        unsafe {
            (self.create_surface_ohos)(instance, create_info, allocator.to_raw_ptr(), surface)
        }
    }
}
