#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{c_char, c_int, c_void};
#[repr(C)]
pub struct PhysicalDeviceSwapchainMaintenance1FeaturesKHR {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub swapchain_maintenance1: Bool32,
}
#[repr(C)]
pub struct SwapchainPresentFenceInfoKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub swapchain_count: u32,
    pub p_fences: *const Fence,
}
#[repr(C)]
pub struct SwapchainPresentModesCreateInfoKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub present_mode_count: u32,
    pub p_present_modes: *const PresentModeKHR,
}
#[repr(C)]
pub struct SwapchainPresentModeInfoKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub swapchain_count: u32,
    pub p_present_modes: *const PresentModeKHR,
}
#[repr(C)]
pub struct SwapchainPresentScalingCreateInfoKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub scaling_behavior: PresentScalingFlagsKHR,
    pub present_gravity_x: PresentGravityFlagsKHR,
    pub present_gravity_y: PresentGravityFlagsKHR,
}
#[repr(C)]
pub struct ReleaseSwapchainImagesInfoKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub swapchain: SwapchainKHR,
    pub image_index_count: u32,
    pub p_image_indices: *const u32,
}
pub type PFN_vkReleaseSwapchainImagesKHR = unsafe extern "system" fn(
    device: Device,
    p_release_info: *const ReleaseSwapchainImagesInfoKHR,
) -> Result;
