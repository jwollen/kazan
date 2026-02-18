#![allow(unused_imports)]
use crate::*;
use kazan_sys::{vk::*, *};
use std::ffi::{c_char, c_int, c_void, CStr};
pub struct InstanceFn {
    get_physical_device_surface_capabilities2_khr: PFN_vkGetPhysicalDeviceSurfaceCapabilities2KHR,
    get_physical_device_surface_formats2_khr: PFN_vkGetPhysicalDeviceSurfaceFormats2KHR,
}
impl InstanceFn {
    pub unsafe fn get_physical_device_surface_capabilities2_khr(
        &self,
        physical_device: PhysicalDevice,
        surface_info: &PhysicalDeviceSurfaceInfo2KHR,
        surface_capabilities: &mut SurfaceCapabilities2KHR,
    ) -> Result {
        unsafe {
            (self.get_physical_device_surface_capabilities2_khr)(
                physical_device,
                surface_info,
                surface_capabilities,
            )
        }
    }
    pub unsafe fn get_physical_device_surface_formats2_khr(
        &self,
        physical_device: PhysicalDevice,
        surface_info: &PhysicalDeviceSurfaceInfo2KHR,
        surface_formats: impl ExtendUninit<SurfaceFormat2KHR>,
    ) -> Result {
        unsafe {
            try_extend_uninit(surface_formats, |surface_format_count, surface_formats| {
                (self.get_physical_device_surface_formats2_khr)(
                    physical_device,
                    surface_info,
                    surface_format_count,
                    surface_formats as _,
                )
            })
        }
    }
}
