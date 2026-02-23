#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{c_char, c_int, c_void};
#[repr(C)]
pub struct DisplayProperties2KHR {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub display_properties: DisplayPropertiesKHR,
}
#[repr(C)]
pub struct DisplayPlaneProperties2KHR {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub display_plane_properties: DisplayPlanePropertiesKHR,
}
#[repr(C)]
pub struct DisplayModeProperties2KHR {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub display_mode_properties: DisplayModePropertiesKHR,
}
#[repr(C)]
pub struct DisplayPlaneInfo2KHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub mode: DisplayModeKHR,
    pub plane_index: u32,
}
#[repr(C)]
pub struct DisplayPlaneCapabilities2KHR {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub capabilities: DisplayPlaneCapabilitiesKHR,
}
pub type PFN_vkGetPhysicalDeviceDisplayProperties2KHR = unsafe extern "system" fn(
    physical_device: PhysicalDevice,
    p_property_count: *mut u32,
    p_properties: *mut DisplayProperties2KHR,
) -> Result;
pub type PFN_vkGetPhysicalDeviceDisplayPlaneProperties2KHR = unsafe extern "system" fn(
    physical_device: PhysicalDevice,
    p_property_count: *mut u32,
    p_properties: *mut DisplayPlaneProperties2KHR,
) -> Result;
pub type PFN_vkGetDisplayModeProperties2KHR = unsafe extern "system" fn(
    physical_device: PhysicalDevice,
    display: DisplayKHR,
    p_property_count: *mut u32,
    p_properties: *mut DisplayModeProperties2KHR,
) -> Result;
pub type PFN_vkGetDisplayPlaneCapabilities2KHR = unsafe extern "system" fn(
    physical_device: PhysicalDevice,
    p_display_plane_info: *const DisplayPlaneInfo2KHR,
    p_capabilities: *mut DisplayPlaneCapabilities2KHR,
) -> Result;
