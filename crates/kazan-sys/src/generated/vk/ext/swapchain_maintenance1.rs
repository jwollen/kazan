#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{CStr, c_char, c_int, c_void};
use core::marker::PhantomData;
pub type PhysicalDeviceSwapchainMaintenance1FeaturesEXT<'a> =
    PhysicalDeviceSwapchainMaintenance1FeaturesKHR<'a>;
pub type SwapchainPresentFenceInfoEXT<'a> = SwapchainPresentFenceInfoKHR<'a>;
pub type SwapchainPresentModesCreateInfoEXT<'a> = SwapchainPresentModesCreateInfoKHR<'a>;
pub type SwapchainPresentModeInfoEXT<'a> = SwapchainPresentModeInfoKHR<'a>;
pub type SwapchainPresentScalingCreateInfoEXT<'a> = SwapchainPresentScalingCreateInfoKHR<'a>;
pub type ReleaseSwapchainImagesInfoEXT<'a> = ReleaseSwapchainImagesInfoKHR<'a>;
pub type PFN_vkReleaseSwapchainImagesEXT = PFN_vkReleaseSwapchainImagesKHR;
