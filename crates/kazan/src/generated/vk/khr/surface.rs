#![allow(unused_imports)]
use crate::*;
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
use kazan_sys::{vk::*, *};
pub struct InstanceFn {
    destroy_surface_khr: PFN_vkDestroySurfaceKHR,
    get_physical_device_surface_support_khr: PFN_vkGetPhysicalDeviceSurfaceSupportKHR,
    get_physical_device_surface_capabilities_khr: PFN_vkGetPhysicalDeviceSurfaceCapabilitiesKHR,
    get_physical_device_surface_formats_khr: PFN_vkGetPhysicalDeviceSurfaceFormatsKHR,
    get_physical_device_surface_present_modes_khr: PFN_vkGetPhysicalDeviceSurfacePresentModesKHR,
}
impl InstanceFn {
    pub unsafe fn load(
        load: impl Fn(&CStr) -> Option<PFN_vkVoidFunction>,
    ) -> core::result::Result<Self, LoadingError> {
        unsafe {
            Ok(Self {
                destroy_surface_khr: transmute(load(c"vkDestroySurfaceKHR").ok_or(LoadingError)?),
                get_physical_device_surface_support_khr: transmute(
                    load(c"vkGetPhysicalDeviceSurfaceSupportKHR").ok_or(LoadingError)?,
                ),
                get_physical_device_surface_capabilities_khr: transmute(
                    load(c"vkGetPhysicalDeviceSurfaceCapabilitiesKHR").ok_or(LoadingError)?,
                ),
                get_physical_device_surface_formats_khr: transmute(
                    load(c"vkGetPhysicalDeviceSurfaceFormatsKHR").ok_or(LoadingError)?,
                ),
                get_physical_device_surface_present_modes_khr: transmute(
                    load(c"vkGetPhysicalDeviceSurfacePresentModesKHR").ok_or(LoadingError)?,
                ),
            })
        }
    }
}
impl InstanceFn {
    pub unsafe fn destroy_surface_khr(
        &self,
        instance: Instance,
        surface: SurfaceKHR,
        allocator: Option<&AllocationCallbacks>,
    ) {
        unsafe { (self.destroy_surface_khr)(instance, surface, allocator.to_raw_ptr()) }
    }
    pub unsafe fn get_physical_device_surface_support_khr(
        &self,
        physical_device: PhysicalDevice,
        queue_family_index: u32,
        surface: SurfaceKHR,
        supported: &mut Bool32,
    ) -> crate::Result<()> {
        unsafe {
            result((self.get_physical_device_surface_support_khr)(
                physical_device,
                queue_family_index,
                surface,
                supported,
            ))
        }
    }
    pub unsafe fn get_physical_device_surface_capabilities_khr(
        &self,
        physical_device: PhysicalDevice,
        surface: SurfaceKHR,
        surface_capabilities: &mut SurfaceCapabilitiesKHR,
    ) -> crate::Result<()> {
        unsafe {
            result((self.get_physical_device_surface_capabilities_khr)(
                physical_device,
                surface,
                surface_capabilities,
            ))
        }
    }
    pub unsafe fn get_physical_device_surface_formats_khr(
        &self,
        physical_device: PhysicalDevice,
        surface: SurfaceKHR,
        surface_formats: impl ExtendUninit<SurfaceFormatKHR>,
    ) -> crate::Result<()> {
        unsafe {
            try_extend_uninit(surface_formats, |surface_format_count, surface_formats| {
                result((self.get_physical_device_surface_formats_khr)(
                    physical_device,
                    surface,
                    surface_format_count,
                    surface_formats as _,
                ))
            })
        }
    }
    pub unsafe fn get_physical_device_surface_present_modes_khr(
        &self,
        physical_device: PhysicalDevice,
        surface: SurfaceKHR,
        present_modes: impl ExtendUninit<PresentModeKHR>,
    ) -> crate::Result<()> {
        unsafe {
            try_extend_uninit(present_modes, |present_mode_count, present_modes| {
                result((self.get_physical_device_surface_present_modes_khr)(
                    physical_device,
                    surface,
                    present_mode_count,
                    present_modes as _,
                ))
            })
        }
    }
}
