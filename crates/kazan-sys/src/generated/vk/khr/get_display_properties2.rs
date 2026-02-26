#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{c_char, c_int, c_void};
use core::marker::PhantomData;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct DisplayProperties2KHR<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub display_properties: DisplayPropertiesKHR<'a>,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for DisplayProperties2KHR<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::DISPLAY_PROPERTIES_2_KHR,
            p_next: core::ptr::null_mut(),
            display_properties: Default::default(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct DisplayPlaneProperties2KHR<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub display_plane_properties: DisplayPlanePropertiesKHR,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for DisplayPlaneProperties2KHR<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::DISPLAY_PLANE_PROPERTIES_2_KHR,
            p_next: core::ptr::null_mut(),
            display_plane_properties: Default::default(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct DisplayModeProperties2KHR<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub display_mode_properties: DisplayModePropertiesKHR,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for DisplayModeProperties2KHR<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::DISPLAY_MODE_PROPERTIES_2_KHR,
            p_next: core::ptr::null_mut(),
            display_mode_properties: Default::default(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct DisplayPlaneInfo2KHR<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub mode: DisplayModeKHR,
    pub plane_index: u32,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for DisplayPlaneInfo2KHR<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::DISPLAY_PLANE_INFO_2_KHR,
            p_next: core::ptr::null(),
            mode: Default::default(),
            plane_index: Default::default(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct DisplayPlaneCapabilities2KHR<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub capabilities: DisplayPlaneCapabilitiesKHR,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for DisplayPlaneCapabilities2KHR<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::DISPLAY_PLANE_CAPABILITIES_2_KHR,
            p_next: core::ptr::null_mut(),
            capabilities: Default::default(),
            _marker: PhantomData,
        }
    }
}
pub type PFN_vkGetPhysicalDeviceDisplayProperties2KHR = unsafe extern "system" fn(
    physical_device: PhysicalDevice,
    p_property_count: *mut u32,
    p_properties: *mut DisplayProperties2KHR<'_>,
) -> Result;
pub type PFN_vkGetPhysicalDeviceDisplayPlaneProperties2KHR = unsafe extern "system" fn(
    physical_device: PhysicalDevice,
    p_property_count: *mut u32,
    p_properties: *mut DisplayPlaneProperties2KHR<'_>,
) -> Result;
pub type PFN_vkGetDisplayModeProperties2KHR = unsafe extern "system" fn(
    physical_device: PhysicalDevice,
    display: DisplayKHR,
    p_property_count: *mut u32,
    p_properties: *mut DisplayModeProperties2KHR<'_>,
) -> Result;
pub type PFN_vkGetDisplayPlaneCapabilities2KHR = unsafe extern "system" fn(
    physical_device: PhysicalDevice,
    p_display_plane_info: *const DisplayPlaneInfo2KHR<'_>,
    p_capabilities: *mut DisplayPlaneCapabilities2KHR<'_>,
) -> Result;
