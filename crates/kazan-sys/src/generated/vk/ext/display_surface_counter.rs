#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{c_char, c_int, c_void};
#[repr(C)]
pub struct SurfaceCapabilities2EXT {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
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
    pub supported_surface_counters: SurfaceCounterFlagsEXT,
}
bitflags! {
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Default)]
    pub struct SurfaceCounterFlagsEXT: Flags {
        const VBLANK_EXT = SurfaceCounterFlagBitsEXT::VBLANK_EXT.0;
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SurfaceCounterFlagBitsEXT(u32);
impl SurfaceCounterFlagBitsEXT {
    pub const VBLANK_EXT: Self = Self(1 << 0);
}
pub type PFN_vkGetPhysicalDeviceSurfaceCapabilities2EXT = unsafe extern "system" fn(
    physical_device: PhysicalDevice,
    surface: SurfaceKHR,
    p_surface_capabilities: *mut SurfaceCapabilities2EXT,
) -> Result;
