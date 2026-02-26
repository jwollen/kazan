#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{c_char, c_int, c_void};
use core::marker::PhantomData;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceSwapchainMaintenance1FeaturesKHR<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub swapchain_maintenance1: Bool32,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for PhysicalDeviceSwapchainMaintenance1FeaturesKHR<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::PHYSICAL_DEVICE_SWAPCHAIN_MAINTENANCE_1_FEATURES_KHR,
            p_next: core::ptr::null_mut(),
            swapchain_maintenance1: Default::default(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SwapchainPresentFenceInfoKHR<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub swapchain_count: u32,
    pub p_fences: *const Fence,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for SwapchainPresentFenceInfoKHR<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::SWAPCHAIN_PRESENT_FENCE_INFO_KHR,
            p_next: core::ptr::null(),
            swapchain_count: Default::default(),
            p_fences: core::ptr::null(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SwapchainPresentModesCreateInfoKHR<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub present_mode_count: u32,
    pub p_present_modes: *const PresentModeKHR,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for SwapchainPresentModesCreateInfoKHR<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::SWAPCHAIN_PRESENT_MODES_CREATE_INFO_KHR,
            p_next: core::ptr::null(),
            present_mode_count: Default::default(),
            p_present_modes: core::ptr::null(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SwapchainPresentModeInfoKHR<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub swapchain_count: u32,
    pub p_present_modes: *const PresentModeKHR,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for SwapchainPresentModeInfoKHR<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::SWAPCHAIN_PRESENT_MODE_INFO_KHR,
            p_next: core::ptr::null(),
            swapchain_count: Default::default(),
            p_present_modes: core::ptr::null(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SwapchainPresentScalingCreateInfoKHR<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub scaling_behavior: PresentScalingFlagsKHR,
    pub present_gravity_x: PresentGravityFlagsKHR,
    pub present_gravity_y: PresentGravityFlagsKHR,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for SwapchainPresentScalingCreateInfoKHR<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::SWAPCHAIN_PRESENT_SCALING_CREATE_INFO_KHR,
            p_next: core::ptr::null(),
            scaling_behavior: Default::default(),
            present_gravity_x: Default::default(),
            present_gravity_y: Default::default(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ReleaseSwapchainImagesInfoKHR<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub swapchain: SwapchainKHR,
    pub image_index_count: u32,
    pub p_image_indices: *const u32,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for ReleaseSwapchainImagesInfoKHR<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::RELEASE_SWAPCHAIN_IMAGES_INFO_KHR,
            p_next: core::ptr::null(),
            swapchain: Default::default(),
            image_index_count: Default::default(),
            p_image_indices: core::ptr::null(),
            _marker: PhantomData,
        }
    }
}
pub type PFN_vkReleaseSwapchainImagesKHR = unsafe extern "system" fn(
    device: Device,
    p_release_info: *const ReleaseSwapchainImagesInfoKHR<'_>,
) -> Result;
