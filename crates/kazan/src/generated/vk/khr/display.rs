#![allow(unused_imports)]
use crate::*;
use core::ffi::{c_char, c_int, c_void, CStr};
use kazan_sys::{vk::*, *};
pub struct InstanceFn {
    get_physical_device_display_properties_khr: PFN_vkGetPhysicalDeviceDisplayPropertiesKHR,
    get_physical_device_display_plane_properties_khr:
        PFN_vkGetPhysicalDeviceDisplayPlanePropertiesKHR,
    get_display_plane_supported_displays_khr: PFN_vkGetDisplayPlaneSupportedDisplaysKHR,
    get_display_mode_properties_khr: PFN_vkGetDisplayModePropertiesKHR,
    create_display_mode_khr: PFN_vkCreateDisplayModeKHR,
    get_display_plane_capabilities_khr: PFN_vkGetDisplayPlaneCapabilitiesKHR,
    create_display_plane_surface_khr: PFN_vkCreateDisplayPlaneSurfaceKHR,
}
impl InstanceFn {
    pub unsafe fn get_physical_device_display_properties_khr(
        &self,
        physical_device: PhysicalDevice,
        properties: impl ExtendUninit<DisplayPropertiesKHR>,
    ) -> Result {
        unsafe {
            try_extend_uninit(properties, |property_count, properties| {
                (self.get_physical_device_display_properties_khr)(
                    physical_device,
                    property_count,
                    properties as _,
                )
            })
        }
    }
    pub unsafe fn get_physical_device_display_plane_properties_khr(
        &self,
        physical_device: PhysicalDevice,
        properties: impl ExtendUninit<DisplayPlanePropertiesKHR>,
    ) -> Result {
        unsafe {
            try_extend_uninit(properties, |property_count, properties| {
                (self.get_physical_device_display_plane_properties_khr)(
                    physical_device,
                    property_count,
                    properties as _,
                )
            })
        }
    }
    pub unsafe fn get_display_plane_supported_displays_khr(
        &self,
        physical_device: PhysicalDevice,
        plane_index: u32,
        displays: impl ExtendUninit<DisplayKHR>,
    ) -> Result {
        unsafe {
            try_extend_uninit(displays, |display_count, displays| {
                (self.get_display_plane_supported_displays_khr)(
                    physical_device,
                    plane_index,
                    display_count,
                    displays as _,
                )
            })
        }
    }
    pub unsafe fn get_display_mode_properties_khr(
        &self,
        physical_device: PhysicalDevice,
        display: DisplayKHR,
        properties: impl ExtendUninit<DisplayModePropertiesKHR>,
    ) -> Result {
        unsafe {
            try_extend_uninit(properties, |property_count, properties| {
                (self.get_display_mode_properties_khr)(
                    physical_device,
                    display,
                    property_count,
                    properties as _,
                )
            })
        }
    }
    pub unsafe fn create_display_mode_khr(
        &self,
        physical_device: PhysicalDevice,
        display: DisplayKHR,
        create_info: &DisplayModeCreateInfoKHR,
        allocator: Option<&AllocationCallbacks>,
        mode: &mut DisplayModeKHR,
    ) -> Result {
        unsafe {
            (self.create_display_mode_khr)(
                physical_device,
                display,
                create_info,
                allocator.to_raw_ptr(),
                mode,
            )
        }
    }
    pub unsafe fn get_display_plane_capabilities_khr(
        &self,
        physical_device: PhysicalDevice,
        mode: DisplayModeKHR,
        plane_index: u32,
        capabilities: &mut DisplayPlaneCapabilitiesKHR,
    ) -> Result {
        unsafe {
            (self.get_display_plane_capabilities_khr)(
                physical_device,
                mode,
                plane_index,
                capabilities,
            )
        }
    }
    pub unsafe fn create_display_plane_surface_khr(
        &self,
        instance: Instance,
        create_info: &DisplaySurfaceCreateInfoKHR,
        allocator: Option<&AllocationCallbacks>,
        surface: &mut SurfaceKHR,
    ) -> Result {
        unsafe {
            (self.create_display_plane_surface_khr)(
                instance,
                create_info,
                allocator.to_raw_ptr(),
                surface,
            )
        }
    }
}
