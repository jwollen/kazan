#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{c_char, c_int, c_void};
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SurfaceKHR(u64);
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SurfaceCapabilitiesKHR {
    pub min_image_count: u32,
    pub max_image_count: u32,
    pub current_extent: Extent2D,
    pub min_image_extent: Extent2D,
    pub max_image_extent: Extent2D,
    pub max_image_array_layers: u32,
    pub supported_transforms: SurfaceTransformFlagsKHR,
    pub current_transform: SurfaceTransformFlagBitsKHR,
    pub supported_composite_alpha: CompositeAlphaFlagsKHR,
    pub supported_usage_flags: ImageUsageFlags,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SurfaceFormatKHR {
    pub format: Format,
    pub color_space: ColorSpaceKHR,
}
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PresentModeKHR(i32);
impl PresentModeKHR {
    pub const IMMEDIATE_KHR: Self = Self(0);
    pub const MAILBOX_KHR: Self = Self(1);
    pub const FIFO_KHR: Self = Self(2);
    pub const FIFO_RELAXED_KHR: Self = Self(3);
    pub const FIFO_LATEST_READY_KHR: Self = Self(1000361000);
    pub const SHARED_CONTINUOUS_REFRESH_KHR: Self = Self(1000111001);
    pub const SHARED_DEMAND_REFRESH_KHR: Self = Self(1000111000);
}
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ColorSpaceKHR(i32);
impl ColorSpaceKHR {
    pub const SRGB_NONLINEAR_KHR: Self = Self(0);
    pub const ADOBERGB_LINEAR_EXT: Self = Self(1000104011);
    pub const ADOBERGB_NONLINEAR_EXT: Self = Self(1000104012);
    pub const BT2020_LINEAR_EXT: Self = Self(1000104007);
    pub const BT709_LINEAR_EXT: Self = Self(1000104005);
    pub const BT709_NONLINEAR_EXT: Self = Self(1000104006);
    pub const DCI_P3_NONLINEAR_EXT: Self = Self(1000104004);
    pub const DISPLAY_NATIVE_AMD: Self = Self(1000213000);
    pub const DISPLAY_P3_LINEAR_EXT: Self = Self(1000104003);
    pub const DISPLAY_P3_NONLINEAR_EXT: Self = Self(1000104001);
    pub const DOLBYVISION_EXT: Self = Self(1000104009);
    pub const EXTENDED_SRGB_LINEAR_EXT: Self = Self(1000104002);
    pub const EXTENDED_SRGB_NONLINEAR_EXT: Self = Self(1000104014);
    pub const HDR10_HLG_EXT: Self = Self(1000104010);
    pub const HDR10_ST2084_EXT: Self = Self(1000104008);
    pub const PASS_THROUGH_EXT: Self = Self(1000104013);
}
bitflags! {
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Default)]
    pub struct CompositeAlphaFlagsKHR: Flags {
        const OPAQUE_KHR = CompositeAlphaFlagBitsKHR::OPAQUE_KHR.0;
        const PRE_MULTIPLIED_KHR = CompositeAlphaFlagBitsKHR::PRE_MULTIPLIED_KHR.0;
        const POST_MULTIPLIED_KHR = CompositeAlphaFlagBitsKHR::POST_MULTIPLIED_KHR.0;
        const INHERIT_KHR = CompositeAlphaFlagBitsKHR::INHERIT_KHR.0;
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CompositeAlphaFlagBitsKHR(u32);
impl CompositeAlphaFlagBitsKHR {
    pub const OPAQUE_KHR: Self = Self(1 << 0);
    pub const PRE_MULTIPLIED_KHR: Self = Self(1 << 1);
    pub const POST_MULTIPLIED_KHR: Self = Self(1 << 2);
    pub const INHERIT_KHR: Self = Self(1 << 3);
}
pub type PFN_vkDestroySurfaceKHR = unsafe extern "system" fn(
    instance: Instance,
    surface: SurfaceKHR,
    p_allocator: *const AllocationCallbacks,
);
pub type PFN_vkGetPhysicalDeviceSurfaceSupportKHR = unsafe extern "system" fn(
    physical_device: PhysicalDevice,
    queue_family_index: u32,
    surface: SurfaceKHR,
    p_supported: *mut Bool32,
) -> Result;
pub type PFN_vkGetPhysicalDeviceSurfaceCapabilitiesKHR = unsafe extern "system" fn(
    physical_device: PhysicalDevice,
    surface: SurfaceKHR,
    p_surface_capabilities: *mut SurfaceCapabilitiesKHR,
) -> Result;
pub type PFN_vkGetPhysicalDeviceSurfaceFormatsKHR = unsafe extern "system" fn(
    physical_device: PhysicalDevice,
    surface: SurfaceKHR,
    p_surface_format_count: *mut u32,
    p_surface_formats: *mut SurfaceFormatKHR,
) -> Result;
pub type PFN_vkGetPhysicalDeviceSurfacePresentModesKHR = unsafe extern "system" fn(
    physical_device: PhysicalDevice,
    surface: SurfaceKHR,
    p_present_mode_count: *mut u32,
    p_present_modes: *mut PresentModeKHR,
) -> Result;
