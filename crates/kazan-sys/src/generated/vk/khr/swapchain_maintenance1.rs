#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{c_char, c_int, c_void};
use core::marker::PhantomData;
#[repr(C)]
pub struct PhysicalDeviceSwapchainMaintenance1FeaturesKHR<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub swapchain_maintenance1: Bool32,
    pub _marker: PhantomData<&'a ()>,
}
#[repr(C)]
pub struct SwapchainPresentFenceInfoKHR<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub swapchain_count: u32,
    pub p_fences: *const Fence,
    pub _marker: PhantomData<&'a ()>,
}
#[repr(C)]
pub struct SwapchainPresentModesCreateInfoKHR<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub present_mode_count: u32,
    pub p_present_modes: *const PresentModeKHR,
    pub _marker: PhantomData<&'a ()>,
}
#[repr(C)]
pub struct SwapchainPresentModeInfoKHR<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub swapchain_count: u32,
    pub p_present_modes: *const PresentModeKHR,
    pub _marker: PhantomData<&'a ()>,
}
#[repr(C)]
pub struct SwapchainPresentScalingCreateInfoKHR<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub scaling_behavior: PresentScalingFlagsKHR,
    pub present_gravity_x: PresentGravityFlagsKHR,
    pub present_gravity_y: PresentGravityFlagsKHR,
    pub _marker: PhantomData<&'a ()>,
}
#[repr(C)]
pub struct ReleaseSwapchainImagesInfoKHR<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub swapchain: SwapchainKHR,
    pub image_index_count: u32,
    pub p_image_indices: *const u32,
    pub _marker: PhantomData<&'a ()>,
}
pub type PFN_vkReleaseSwapchainImagesKHR = unsafe extern "system" fn(
    device: Device,
    p_release_info: *const ReleaseSwapchainImagesInfoKHR<'_>,
) -> Result;
