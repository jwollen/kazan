#![allow(unused_imports)]
use crate::*;
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
use kazan_sys::{vk::Result as VkResult, vk::*, *};
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
    pub unsafe fn load(
        load: impl Fn(&CStr) -> Option<PFN_vkVoidFunction>,
    ) -> core::result::Result<Self, LoadingError> {
        unsafe {
            Ok(Self {
                get_physical_device_display_properties_khr: transmute(
                    load(c"vkGetPhysicalDeviceDisplayPropertiesKHR").ok_or(LoadingError)?,
                ),
                get_physical_device_display_plane_properties_khr: transmute(
                    load(c"vkGetPhysicalDeviceDisplayPlanePropertiesKHR").ok_or(LoadingError)?,
                ),
                get_display_plane_supported_displays_khr: transmute(
                    load(c"vkGetDisplayPlaneSupportedDisplaysKHR").ok_or(LoadingError)?,
                ),
                get_display_mode_properties_khr: transmute(
                    load(c"vkGetDisplayModePropertiesKHR").ok_or(LoadingError)?,
                ),
                create_display_mode_khr: transmute(
                    load(c"vkCreateDisplayModeKHR").ok_or(LoadingError)?,
                ),
                get_display_plane_capabilities_khr: transmute(
                    load(c"vkGetDisplayPlaneCapabilitiesKHR").ok_or(LoadingError)?,
                ),
                create_display_plane_surface_khr: transmute(
                    load(c"vkCreateDisplayPlaneSurfaceKHR").ok_or(LoadingError)?,
                ),
            })
        }
    }
}
impl InstanceFn {
    pub unsafe fn get_physical_device_display_properties_khr(
        &self,
        physical_device: PhysicalDevice,
        properties: impl ExtendUninit<DisplayPropertiesKHR>,
    ) -> crate::Result<()> {
        unsafe {
            try_extend_uninit(properties, |property_count, properties| {
                let result = (self.get_physical_device_display_properties_khr)(
                    physical_device,
                    property_count,
                    properties as _,
                );

                match result {
                    VkResult::SUCCESS => Ok(()),
                    VkResult::INCOMPLETE => Ok(()),
                    err => Err(err),
                }
            })
        }
    }
    pub unsafe fn get_physical_device_display_plane_properties_khr(
        &self,
        physical_device: PhysicalDevice,
        properties: impl ExtendUninit<DisplayPlanePropertiesKHR>,
    ) -> crate::Result<()> {
        unsafe {
            try_extend_uninit(properties, |property_count, properties| {
                let result = (self.get_physical_device_display_plane_properties_khr)(
                    physical_device,
                    property_count,
                    properties as _,
                );

                match result {
                    VkResult::SUCCESS => Ok(()),
                    VkResult::INCOMPLETE => Ok(()),
                    err => Err(err),
                }
            })
        }
    }
    pub unsafe fn get_display_plane_supported_displays_khr(
        &self,
        physical_device: PhysicalDevice,
        plane_index: u32,
        displays: impl ExtendUninit<DisplayKHR>,
    ) -> crate::Result<()> {
        unsafe {
            try_extend_uninit(displays, |display_count, displays| {
                let result = (self.get_display_plane_supported_displays_khr)(
                    physical_device,
                    plane_index,
                    display_count,
                    displays as _,
                );

                match result {
                    VkResult::SUCCESS => Ok(()),
                    VkResult::INCOMPLETE => Ok(()),
                    err => Err(err),
                }
            })
        }
    }
    pub unsafe fn get_display_mode_properties_khr(
        &self,
        physical_device: PhysicalDevice,
        display: DisplayKHR,
        properties: impl ExtendUninit<DisplayModePropertiesKHR>,
    ) -> crate::Result<()> {
        unsafe {
            try_extend_uninit(properties, |property_count, properties| {
                let result = (self.get_display_mode_properties_khr)(
                    physical_device,
                    display,
                    property_count,
                    properties as _,
                );

                match result {
                    VkResult::SUCCESS => Ok(()),
                    VkResult::INCOMPLETE => Ok(()),
                    err => Err(err),
                }
            })
        }
    }
    pub unsafe fn create_display_mode_khr(
        &self,
        physical_device: PhysicalDevice,
        display: DisplayKHR,
        create_info: &DisplayModeCreateInfoKHR,
        allocator: Option<&AllocationCallbacks>,
    ) -> crate::Result<DisplayModeKHR> {
        unsafe {
            let mut mode = core::mem::MaybeUninit::uninit();
            let result = (self.create_display_mode_khr)(
                physical_device,
                display,
                create_info,
                allocator.to_raw_ptr(),
                mode.as_mut_ptr(),
            );

            match result {
                VkResult::SUCCESS => Ok(mode.assume_init()),
                err => Err(err),
            }
        }
    }
    pub unsafe fn get_display_plane_capabilities_khr(
        &self,
        physical_device: PhysicalDevice,
        mode: DisplayModeKHR,
        plane_index: u32,
    ) -> crate::Result<DisplayPlaneCapabilitiesKHR> {
        unsafe {
            let mut capabilities = core::mem::MaybeUninit::uninit();
            let result = (self.get_display_plane_capabilities_khr)(
                physical_device,
                mode,
                plane_index,
                capabilities.as_mut_ptr(),
            );

            match result {
                VkResult::SUCCESS => Ok(capabilities.assume_init()),
                err => Err(err),
            }
        }
    }
    pub unsafe fn create_display_plane_surface_khr(
        &self,
        instance: Instance,
        create_info: &DisplaySurfaceCreateInfoKHR,
        allocator: Option<&AllocationCallbacks>,
    ) -> crate::Result<SurfaceKHR> {
        unsafe {
            let mut surface = core::mem::MaybeUninit::uninit();
            let result = (self.create_display_plane_surface_khr)(
                instance,
                create_info,
                allocator.to_raw_ptr(),
                surface.as_mut_ptr(),
            );

            match result {
                VkResult::SUCCESS => Ok(surface.assume_init()),
                err => Err(err),
            }
        }
    }
}
