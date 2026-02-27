#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{CStr, c_char, c_int, c_void};
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
impl<'a> PhysicalDeviceSwapchainMaintenance1FeaturesKHR<'a> {
    pub fn swapchain_maintenance1(mut self, swapchain_maintenance1: Bool32) -> Self {
        self.swapchain_maintenance1 = swapchain_maintenance1;
        self
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
impl<'a> SwapchainPresentFenceInfoKHR<'a> {
    pub fn fences(mut self, fences: &'a [Fence]) -> Self {
        self.swapchain_count = fences.len().try_into().unwrap();
        self.p_fences = fences.as_ptr();
        self
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
impl<'a> SwapchainPresentModesCreateInfoKHR<'a> {
    pub fn present_modes(mut self, present_modes: &'a [PresentModeKHR]) -> Self {
        self.present_mode_count = present_modes.len().try_into().unwrap();
        self.p_present_modes = present_modes.as_ptr();
        self
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
impl<'a> SwapchainPresentModeInfoKHR<'a> {
    pub fn present_modes(mut self, present_modes: &'a [PresentModeKHR]) -> Self {
        self.swapchain_count = present_modes.len().try_into().unwrap();
        self.p_present_modes = present_modes.as_ptr();
        self
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
impl<'a> SwapchainPresentScalingCreateInfoKHR<'a> {
    pub fn scaling_behavior(mut self, scaling_behavior: PresentScalingFlagsKHR) -> Self {
        self.scaling_behavior = scaling_behavior;
        self
    }
    pub fn present_gravity_x(mut self, present_gravity_x: PresentGravityFlagsKHR) -> Self {
        self.present_gravity_x = present_gravity_x;
        self
    }
    pub fn present_gravity_y(mut self, present_gravity_y: PresentGravityFlagsKHR) -> Self {
        self.present_gravity_y = present_gravity_y;
        self
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
impl<'a> ReleaseSwapchainImagesInfoKHR<'a> {
    pub fn swapchain(mut self, swapchain: SwapchainKHR) -> Self {
        self.swapchain = swapchain;
        self
    }
    pub fn image_indices(mut self, image_indices: &'a [u32]) -> Self {
        self.image_index_count = image_indices.len().try_into().unwrap();
        self.p_image_indices = image_indices.as_ptr();
        self
    }
}
pub type PFN_vkReleaseSwapchainImagesKHR = unsafe extern "system" fn(
    device: Device,
    p_release_info: *const ReleaseSwapchainImagesInfoKHR<'_>,
) -> Result;
