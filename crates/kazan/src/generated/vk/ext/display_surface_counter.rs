#![allow(unused_imports)]
use crate::*;
use kazan_sys::{vk::*, *};
use std::ffi::{c_char, c_int, c_void, CStr};
pub struct InstanceFn {
    get_physical_device_surface_capabilities2_ext: PFN_vkGetPhysicalDeviceSurfaceCapabilities2EXT,
}
impl InstanceFn {
    pub unsafe fn get_physical_device_surface_capabilities2_ext(
        &self,
        physical_device: PhysicalDevice,
        surface: SurfaceKHR,
        surface_capabilities: &mut SurfaceCapabilities2EXT,
    ) -> Result {
        unsafe {
            (self.get_physical_device_surface_capabilities2_ext)(
                physical_device,
                surface,
                surface_capabilities,
            )
        }
    }
}
