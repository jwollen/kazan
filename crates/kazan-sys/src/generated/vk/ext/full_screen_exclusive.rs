#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{c_char, c_int, c_void};
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SurfaceFullScreenExclusiveInfoEXT {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub full_screen_exclusive: FullScreenExclusiveEXT,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SurfaceFullScreenExclusiveWin32InfoEXT {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub hmonitor: HMONITOR,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SurfaceCapabilitiesFullScreenExclusiveEXT {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub full_screen_exclusive_supported: Bool32,
}
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FullScreenExclusiveEXT(i32);
impl FullScreenExclusiveEXT {
    pub const DEFAULT_EXT: Self = Self(0);
    pub const ALLOWED_EXT: Self = Self(1);
    pub const DISALLOWED_EXT: Self = Self(2);
    pub const APPLICATION_CONTROLLED_EXT: Self = Self(3);
}
pub type PFN_vkGetPhysicalDeviceSurfacePresentModes2EXT = unsafe extern "system" fn(
    physical_device: PhysicalDevice,
    p_surface_info: *const PhysicalDeviceSurfaceInfo2KHR,
    p_present_mode_count: *mut u32,
    p_present_modes: *mut PresentModeKHR,
) -> Result;
pub type PFN_vkGetDeviceGroupSurfacePresentModes2EXT = unsafe extern "system" fn(
    device: Device,
    p_surface_info: *const PhysicalDeviceSurfaceInfo2KHR,
    p_modes: *mut DeviceGroupPresentModeFlagsKHR,
) -> Result;
pub type PFN_vkAcquireFullScreenExclusiveModeEXT =
    unsafe extern "system" fn(device: Device, swapchain: SwapchainKHR) -> Result;
pub type PFN_vkReleaseFullScreenExclusiveModeEXT =
    unsafe extern "system" fn(device: Device, swapchain: SwapchainKHR) -> Result;
