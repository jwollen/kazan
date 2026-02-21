#![allow(unused_imports)]
use crate::*;
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
use kazan_sys::{vk::*, *};
pub struct InstanceFn {
    create_vi_surface_nn: PFN_vkCreateViSurfaceNN,
}
impl InstanceFn {
    pub unsafe fn load(
        load: impl Fn(&CStr) -> Option<PFN_vkVoidFunction>,
    ) -> core::result::Result<Self, LoadingError> {
        unsafe {
            Ok(Self {
                create_vi_surface_nn: transmute(load(c"vkCreateViSurfaceNN").ok_or(LoadingError)?),
            })
        }
    }
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
