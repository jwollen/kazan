#![allow(unused_imports)]
use crate::*;
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
use kazan_sys::{vk::Result as VkResult, vk::*, *};
pub struct InstanceFn {
    get_physical_device_display_properties2_khr: PFN_vkGetPhysicalDeviceDisplayProperties2KHR,
    get_physical_device_display_plane_properties2_khr:
        PFN_vkGetPhysicalDeviceDisplayPlaneProperties2KHR,
    get_display_mode_properties2_khr: PFN_vkGetDisplayModeProperties2KHR,
    get_display_plane_capabilities2_khr: PFN_vkGetDisplayPlaneCapabilities2KHR,
}
impl InstanceFn {
    pub unsafe fn load(
        load: impl Fn(&CStr) -> Option<PFN_vkVoidFunction>,
    ) -> core::result::Result<Self, LoadingError> {
        unsafe {
            Ok(Self {
                get_physical_device_display_properties2_khr: transmute(
                    load(c"vkGetPhysicalDeviceDisplayProperties2KHR").ok_or(LoadingError)?,
                ),
                get_physical_device_display_plane_properties2_khr: transmute(
                    load(c"vkGetPhysicalDeviceDisplayPlaneProperties2KHR").ok_or(LoadingError)?,
                ),
                get_display_mode_properties2_khr: transmute(
                    load(c"vkGetDisplayModeProperties2KHR").ok_or(LoadingError)?,
                ),
                get_display_plane_capabilities2_khr: transmute(
                    load(c"vkGetDisplayPlaneCapabilities2KHR").ok_or(LoadingError)?,
                ),
            })
        }
    }
}
impl InstanceFn {
    pub unsafe fn get_physical_device_display_properties2_khr<'a>(
        &self,
        physical_device: PhysicalDevice,
        properties: impl ExtendUninit<DisplayProperties2KHR<'a>>,
    ) -> crate::Result<()> {
        unsafe {
            try_extend_uninit(properties, |property_count, properties| {
                let result = (self.get_physical_device_display_properties2_khr)(
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
    pub unsafe fn get_physical_device_display_plane_properties2_khr<'a>(
        &self,
        physical_device: PhysicalDevice,
        properties: impl ExtendUninit<DisplayPlaneProperties2KHR<'a>>,
    ) -> crate::Result<()> {
        unsafe {
            try_extend_uninit(properties, |property_count, properties| {
                let result = (self.get_physical_device_display_plane_properties2_khr)(
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
    pub unsafe fn get_display_mode_properties2_khr<'a>(
        &self,
        physical_device: PhysicalDevice,
        display: DisplayKHR,
        properties: impl ExtendUninit<DisplayModeProperties2KHR<'a>>,
    ) -> crate::Result<()> {
        unsafe {
            try_extend_uninit(properties, |property_count, properties| {
                let result = (self.get_display_mode_properties2_khr)(
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
    pub unsafe fn get_display_plane_capabilities2_khr(
        &self,
        physical_device: PhysicalDevice,
        display_plane_info: &DisplayPlaneInfo2KHR<'_>,
    ) -> crate::Result<DisplayPlaneCapabilities2KHR<'_>> {
        unsafe {
            let mut capabilities = core::mem::MaybeUninit::uninit();
            let result = (self.get_display_plane_capabilities2_khr)(
                physical_device,
                display_plane_info,
                capabilities.as_mut_ptr(),
            );

            match result {
                VkResult::SUCCESS => Ok(capabilities.assume_init()),
                err => Err(err),
            }
        }
    }
}
