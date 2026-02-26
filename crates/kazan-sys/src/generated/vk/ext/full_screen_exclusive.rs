#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{c_char, c_int, c_void};
use core::marker::PhantomData;
#[repr(C)]
pub struct SurfaceFullScreenExclusiveInfoEXT<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub full_screen_exclusive: FullScreenExclusiveEXT,
    pub _marker: PhantomData<&'a ()>,
}
#[repr(C)]
pub struct SurfaceFullScreenExclusiveWin32InfoEXT<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub hmonitor: HMONITOR,
    pub _marker: PhantomData<&'a ()>,
}
#[repr(C)]
pub struct SurfaceCapabilitiesFullScreenExclusiveEXT<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub full_screen_exclusive_supported: Bool32,
    pub _marker: PhantomData<&'a ()>,
}
#[repr(transparent)]
#[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FullScreenExclusiveEXT(i32);
impl FullScreenExclusiveEXT {
    pub const DEFAULT_EXT: Self = Self(0);
    pub const ALLOWED_EXT: Self = Self(1);
    pub const DISALLOWED_EXT: Self = Self(2);
    pub const APPLICATION_CONTROLLED_EXT: Self = Self(3);
}
pub type PFN_vkGetPhysicalDeviceSurfacePresentModes2EXT = unsafe extern "system" fn(
    physical_device: PhysicalDevice,
    p_surface_info: *const PhysicalDeviceSurfaceInfo2KHR<'_>,
    p_present_mode_count: *mut u32,
    p_present_modes: *mut PresentModeKHR,
) -> Result;
pub type PFN_vkGetDeviceGroupSurfacePresentModes2EXT = unsafe extern "system" fn(
    device: Device,
    p_surface_info: *const PhysicalDeviceSurfaceInfo2KHR<'_>,
    p_modes: *mut DeviceGroupPresentModeFlagsKHR,
) -> Result;
pub type PFN_vkAcquireFullScreenExclusiveModeEXT =
    unsafe extern "system" fn(device: Device, swapchain: SwapchainKHR) -> Result;
pub type PFN_vkReleaseFullScreenExclusiveModeEXT =
    unsafe extern "system" fn(device: Device, swapchain: SwapchainKHR) -> Result;
