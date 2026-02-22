#![allow(unused_imports)]
use crate::*;
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
use kazan_sys::{vk::*, *};
pub struct InstanceFn {
    get_physical_device_surface_capabilities2_khr: PFN_vkGetPhysicalDeviceSurfaceCapabilities2KHR,
    get_physical_device_surface_formats2_khr: PFN_vkGetPhysicalDeviceSurfaceFormats2KHR,
}
impl InstanceFn {
    pub unsafe fn load(
        load: impl Fn(&CStr) -> Option<PFN_vkVoidFunction>,
    ) -> core::result::Result<Self, LoadingError> {
        unsafe {
            Ok(Self {
                get_physical_device_surface_capabilities2_khr: transmute(
                    load(c"vkGetPhysicalDeviceSurfaceCapabilities2KHR").ok_or(LoadingError)?,
                ),
                get_physical_device_surface_formats2_khr: transmute(
                    load(c"vkGetPhysicalDeviceSurfaceFormats2KHR").ok_or(LoadingError)?,
                ),
            })
        }
    }
}
impl InstanceFn {
    pub unsafe fn get_physical_device_surface_capabilities2_khr(
        &self,
        physical_device: PhysicalDevice,
        surface_info: &PhysicalDeviceSurfaceInfo2KHR,
        surface_capabilities: &mut SurfaceCapabilities2KHR,
    ) -> crate::Result<()> {
        unsafe {
            result((self.get_physical_device_surface_capabilities2_khr)(
                physical_device,
                surface_info,
                surface_capabilities,
            ))
        }
    }
    pub unsafe fn get_physical_device_surface_formats2_khr(
        &self,
        physical_device: PhysicalDevice,
        surface_info: &PhysicalDeviceSurfaceInfo2KHR,
        surface_formats: impl ExtendUninit<SurfaceFormat2KHR>,
    ) -> crate::Result<()> {
        unsafe {
            try_extend_uninit(surface_formats, |surface_format_count, surface_formats| {
                result((self.get_physical_device_surface_formats2_khr)(
                    physical_device,
                    surface_info,
                    surface_format_count,
                    surface_formats as _,
                ))
            })
        }
    }
}
