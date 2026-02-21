#![allow(unused_imports)]
use crate::*;
use core::ffi::{c_char, c_int, c_void, CStr};
use kazan_sys::{vk::*, *};
pub struct InstanceFn {
    get_physical_device_display_properties2_khr: PFN_vkGetPhysicalDeviceDisplayProperties2KHR,
    get_physical_device_display_plane_properties2_khr:
        PFN_vkGetPhysicalDeviceDisplayPlaneProperties2KHR,
    get_display_mode_properties2_khr: PFN_vkGetDisplayModeProperties2KHR,
    get_display_plane_capabilities2_khr: PFN_vkGetDisplayPlaneCapabilities2KHR,
}
impl InstanceFn {
    pub unsafe fn get_physical_device_display_properties2_khr(
        &self,
        physical_device: PhysicalDevice,
        properties: impl ExtendUninit<DisplayProperties2KHR>,
    ) -> Result {
        unsafe {
            try_extend_uninit(properties, |property_count, properties| {
                (self.get_physical_device_display_properties2_khr)(
                    physical_device,
                    property_count,
                    properties as _,
                )
            })
        }
    }
    pub unsafe fn get_physical_device_display_plane_properties2_khr(
        &self,
        physical_device: PhysicalDevice,
        properties: impl ExtendUninit<DisplayPlaneProperties2KHR>,
    ) -> Result {
        unsafe {
            try_extend_uninit(properties, |property_count, properties| {
                (self.get_physical_device_display_plane_properties2_khr)(
                    physical_device,
                    property_count,
                    properties as _,
                )
            })
        }
    }
    pub unsafe fn get_display_mode_properties2_khr(
        &self,
        physical_device: PhysicalDevice,
        display: DisplayKHR,
        properties: impl ExtendUninit<DisplayModeProperties2KHR>,
    ) -> Result {
        unsafe {
            try_extend_uninit(properties, |property_count, properties| {
                (self.get_display_mode_properties2_khr)(
                    physical_device,
                    display,
                    property_count,
                    properties as _,
                )
            })
        }
    }
    pub unsafe fn get_display_plane_capabilities2_khr(
        &self,
        physical_device: PhysicalDevice,
        display_plane_info: &DisplayPlaneInfo2KHR,
        capabilities: &mut DisplayPlaneCapabilities2KHR,
    ) -> Result {
        unsafe {
            (self.get_display_plane_capabilities2_khr)(
                physical_device,
                display_plane_info,
                capabilities,
            )
        }
    }
}
