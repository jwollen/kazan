#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{CStr, c_char, c_int, c_void};
use core::marker::PhantomData;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceSurfaceInfo2KHR<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub surface: SurfaceKHR,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for PhysicalDeviceSurfaceInfo2KHR<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::PHYSICAL_DEVICE_SURFACE_INFO_2_KHR,
            p_next: core::ptr::null(),
            surface: Default::default(),
            _marker: PhantomData,
        }
    }
}
impl<'a> PhysicalDeviceSurfaceInfo2KHR<'a> {
    pub fn surface(mut self, surface: SurfaceKHR) -> Self {
        self.surface = surface;
        self
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SurfaceCapabilities2KHR<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub surface_capabilities: SurfaceCapabilitiesKHR,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for SurfaceCapabilities2KHR<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::SURFACE_CAPABILITIES_2_KHR,
            p_next: core::ptr::null_mut(),
            surface_capabilities: Default::default(),
            _marker: PhantomData,
        }
    }
}
impl<'a> SurfaceCapabilities2KHR<'a> {
    pub fn surface_capabilities(mut self, surface_capabilities: SurfaceCapabilitiesKHR) -> Self {
        self.surface_capabilities = surface_capabilities;
        self
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SurfaceFormat2KHR<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub surface_format: SurfaceFormatKHR,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for SurfaceFormat2KHR<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::SURFACE_FORMAT_2_KHR,
            p_next: core::ptr::null_mut(),
            surface_format: Default::default(),
            _marker: PhantomData,
        }
    }
}
impl<'a> SurfaceFormat2KHR<'a> {
    pub fn surface_format(mut self, surface_format: SurfaceFormatKHR) -> Self {
        self.surface_format = surface_format;
        self
    }
}
pub type PFN_vkGetPhysicalDeviceSurfaceCapabilities2KHR = unsafe extern "system" fn(
    physical_device: PhysicalDevice,
    p_surface_info: *const PhysicalDeviceSurfaceInfo2KHR<'_>,
    p_surface_capabilities: *mut SurfaceCapabilities2KHR<'_>,
) -> Result;
pub type PFN_vkGetPhysicalDeviceSurfaceFormats2KHR = unsafe extern "system" fn(
    physical_device: PhysicalDevice,
    p_surface_info: *const PhysicalDeviceSurfaceInfo2KHR<'_>,
    p_surface_format_count: *mut u32,
    p_surface_formats: *mut SurfaceFormat2KHR<'_>,
) -> Result;
