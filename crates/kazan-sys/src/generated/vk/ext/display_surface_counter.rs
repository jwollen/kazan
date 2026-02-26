#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{c_char, c_int, c_void};
use core::marker::PhantomData;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SurfaceCapabilities2EXT<'a> {
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
    pub _marker: PhantomData<&'a ()>,
}
impl Default for SurfaceCapabilities2EXT<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::SURFACE_CAPABILITIES_2_EXT,
            p_next: core::ptr::null_mut(),
            min_image_count: Default::default(),
            max_image_count: Default::default(),
            current_extent: Default::default(),
            min_image_extent: Default::default(),
            max_image_extent: Default::default(),
            max_image_array_layers: Default::default(),
            supported_transforms: Default::default(),
            current_transform: Default::default(),
            supported_composite_alpha: Default::default(),
            supported_usage_flags: Default::default(),
            supported_surface_counters: Default::default(),
            _marker: PhantomData,
        }
    }
}
bitflags! {
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct SurfaceCounterFlagsEXT: Flags {
        const VBLANK_EXT = SurfaceCounterFlagBitsEXT::VBLANK_EXT.0;
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SurfaceCounterFlagBitsEXT(u32);
impl SurfaceCounterFlagBitsEXT {
    pub const VBLANK_EXT: Self = Self(1 << 0);
}
pub type PFN_vkGetPhysicalDeviceSurfaceCapabilities2EXT = unsafe extern "system" fn(
    physical_device: PhysicalDevice,
    surface: SurfaceKHR,
    p_surface_capabilities: *mut SurfaceCapabilities2EXT<'_>,
) -> Result;
