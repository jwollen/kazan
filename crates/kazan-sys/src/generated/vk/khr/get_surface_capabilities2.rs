#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{c_char, c_int, c_void};
use core::marker::PhantomData;
#[repr(C)]
pub struct PhysicalDeviceSurfaceInfo2KHR<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub surface: SurfaceKHR,
    pub _marker: PhantomData<&'a ()>,
}
#[repr(C)]
pub struct SurfaceCapabilities2KHR<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub surface_capabilities: SurfaceCapabilitiesKHR,
    pub _marker: PhantomData<&'a ()>,
}
#[repr(C)]
pub struct SurfaceFormat2KHR<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub surface_format: SurfaceFormatKHR,
    pub _marker: PhantomData<&'a ()>,
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
