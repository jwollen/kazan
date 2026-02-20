#![allow(unused_imports)]
use crate::*;
use kazan_sys::{vk::*, *};
use std::ffi::{c_char, c_int, c_void, CStr};
pub struct InstanceFn {
    create_mac_os_surface_mvk: PFN_vkCreateMacOSSurfaceMVK,
}
impl InstanceFn {
    pub unsafe fn create_mac_os_surface_mvk(
        &self,
        instance: Instance,
        create_info: &MacOSSurfaceCreateInfoMVK,
        allocator: Option<&AllocationCallbacks>,
        surface: &mut SurfaceKHR,
    ) -> Result {
        unsafe {
            (self.create_mac_os_surface_mvk)(instance, create_info, allocator.to_raw_ptr(), surface)
        }
    }
}
