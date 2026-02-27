#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{CStr, c_char, c_int, c_void};
use core::marker::PhantomData;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SurfaceFullScreenExclusiveInfoEXT<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub full_screen_exclusive: FullScreenExclusiveEXT,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for SurfaceFullScreenExclusiveInfoEXT<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::SURFACE_FULL_SCREEN_EXCLUSIVE_INFO_EXT,
            p_next: core::ptr::null_mut(),
            full_screen_exclusive: Default::default(),
            _marker: PhantomData,
        }
    }
}
impl<'a> SurfaceFullScreenExclusiveInfoEXT<'a> {
    pub fn full_screen_exclusive(mut self, full_screen_exclusive: FullScreenExclusiveEXT) -> Self {
        self.full_screen_exclusive = full_screen_exclusive;
        self
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SurfaceFullScreenExclusiveWin32InfoEXT<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub hmonitor: HMONITOR,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for SurfaceFullScreenExclusiveWin32InfoEXT<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::SURFACE_FULL_SCREEN_EXCLUSIVE_WIN32_INFO_EXT,
            p_next: core::ptr::null(),
            hmonitor: Default::default(),
            _marker: PhantomData,
        }
    }
}
impl<'a> SurfaceFullScreenExclusiveWin32InfoEXT<'a> {
    pub fn hmonitor(mut self, hmonitor: HMONITOR) -> Self {
        self.hmonitor = hmonitor;
        self
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SurfaceCapabilitiesFullScreenExclusiveEXT<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub full_screen_exclusive_supported: Bool32,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for SurfaceCapabilitiesFullScreenExclusiveEXT<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::SURFACE_CAPABILITIES_FULL_SCREEN_EXCLUSIVE_EXT,
            p_next: core::ptr::null_mut(),
            full_screen_exclusive_supported: Default::default(),
            _marker: PhantomData,
        }
    }
}
impl<'a> SurfaceCapabilitiesFullScreenExclusiveEXT<'a> {
    pub fn full_screen_exclusive_supported(
        mut self,
        full_screen_exclusive_supported: Bool32,
    ) -> Self {
        self.full_screen_exclusive_supported = full_screen_exclusive_supported;
        self
    }
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
