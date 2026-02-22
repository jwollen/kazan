#![allow(unused_imports)]
use crate::*;
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
use kazan_sys::{vk::Result as VkResult, vk::*, *};
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
    ) -> crate::Result<Bool32> {
        unsafe {
            let mut supported = core::mem::MaybeUninit::uninit();
            let result = (self.get_physical_device_surface_support_khr)(
                physical_device,
                queue_family_index,
                surface,
                supported.as_mut_ptr(),
            );

            match result {
                VkResult::SUCCESS => Ok(supported.assume_init()),
                err => Err(err),
            }
        }
    }
    pub unsafe fn get_physical_device_surface_capabilities_khr(
        &self,
        physical_device: PhysicalDevice,
        surface: SurfaceKHR,
    ) -> crate::Result<SurfaceCapabilitiesKHR> {
        unsafe {
            let mut surface_capabilities = core::mem::MaybeUninit::uninit();
            let result = (self.get_physical_device_surface_capabilities_khr)(
                physical_device,
                surface,
                surface_capabilities.as_mut_ptr(),
            );

            match result {
                VkResult::SUCCESS => Ok(surface_capabilities.assume_init()),
                err => Err(err),
            }
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
                let result = (self.get_physical_device_surface_formats_khr)(
                    physical_device,
                    surface,
                    surface_format_count,
                    surface_formats as _,
                );

                match result {
                    VkResult::SUCCESS => Ok(()),
                    VkResult::INCOMPLETE => Ok(()),
                    err => Err(err),
                }
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
                let result = (self.get_physical_device_surface_present_modes_khr)(
                    physical_device,
                    surface,
                    present_mode_count,
                    present_modes as _,
                );

                match result {
                    VkResult::SUCCESS => Ok(()),
                    VkResult::INCOMPLETE => Ok(()),
                    err => Err(err),
                }
            })
        }
    }
}
