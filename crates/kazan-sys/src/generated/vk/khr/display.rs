#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use std::ffi::{c_char, c_int, c_void};
#[repr(C)]
#[derive(Copy, Clone)]
pub struct DisplayKHR(u64);
#[repr(C)]
#[derive(Copy, Clone)]
pub struct DisplayModeKHR(u64);
#[repr(C)]
#[derive(Copy, Clone)]
pub struct DisplayPropertiesKHR {
    pub display: DisplayKHR,
    pub display_name: *const c_char,
    pub physical_dimensions: Extent2D,
    pub physical_resolution: Extent2D,
    pub supported_transforms: SurfaceTransformFlagsKHR,
    pub plane_reorder_possible: Bool32,
    pub persistent_content: Bool32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct DisplayPlanePropertiesKHR {
    pub current_display: DisplayKHR,
    pub current_stack_index: u32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct DisplayModeParametersKHR {
    pub visible_region: Extent2D,
    pub refresh_rate: u32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct DisplayModePropertiesKHR {
    pub display_mode: DisplayModeKHR,
    pub parameters: DisplayModeParametersKHR,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct DisplayModeCreateInfoKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: DisplayModeCreateFlagsKHR,
    pub parameters: DisplayModeParametersKHR,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct DisplayPlaneCapabilitiesKHR {
    pub supported_alpha: DisplayPlaneAlphaFlagsKHR,
    pub min_src_position: Offset2D,
    pub max_src_position: Offset2D,
    pub min_src_extent: Extent2D,
    pub max_src_extent: Extent2D,
    pub min_dst_position: Offset2D,
    pub max_dst_position: Offset2D,
    pub min_dst_extent: Extent2D,
    pub max_dst_extent: Extent2D,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct DisplaySurfaceCreateInfoKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: DisplaySurfaceCreateFlagsKHR,
    pub display_mode: DisplayModeKHR,
    pub plane_index: u32,
    pub plane_stack_index: u32,
    pub transform: SurfaceTransformFlagsKHR,
    pub global_alpha: f32,
    pub alpha_mode: DisplayPlaneAlphaFlagsKHR,
    pub image_extent: Extent2D,
}
bitflags! {
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Default)]
    pub struct DisplayPlaneAlphaFlagsKHR: Flags {
        const OPAQUE_KHR = 1 << 0;
        const GLOBAL_KHR = 1 << 1;
        const PER_PIXEL_KHR = 1 << 2;
        const PER_PIXEL_PREMULTIPLIED_KHR = 1 << 3;
    }
}
bitflags! {
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Default)]
    pub struct SurfaceTransformFlagsKHR: Flags {
        const IDENTITY_KHR = 1 << 0;
        const ROTATE_90_KHR = 1 << 1;
        const ROTATE_180_KHR = 1 << 2;
        const ROTATE_270_KHR = 1 << 3;
        const HORIZONTAL_MIRROR_KHR = 1 << 4;
        const HORIZONTAL_MIRROR_ROTATE_90_KHR = 1 << 5;
        const HORIZONTAL_MIRROR_ROTATE_180_KHR = 1 << 6;
        const HORIZONTAL_MIRROR_ROTATE_270_KHR = 1 << 7;
        const INHERIT_KHR = 1 << 8;
    }
}
bitflags! {
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Default)]
    pub struct DisplayModeCreateFlagsKHR: Flags {
    }
}
bitflags! {
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Default)]
    pub struct DisplaySurfaceCreateFlagsKHR: Flags {
    }
}
pub type PFN_vkGetPhysicalDeviceDisplayPropertiesKHR = unsafe extern "system" fn(
    physical_device: PhysicalDevice,
    p_property_count: *mut u32,
    p_properties: *mut DisplayPropertiesKHR,
) -> Result;
pub type PFN_vkGetPhysicalDeviceDisplayPlanePropertiesKHR = unsafe extern "system" fn(
    physical_device: PhysicalDevice,
    p_property_count: *mut u32,
    p_properties: *mut DisplayPlanePropertiesKHR,
) -> Result;
pub type PFN_vkGetDisplayPlaneSupportedDisplaysKHR = unsafe extern "system" fn(
    physical_device: PhysicalDevice,
    plane_index: u32,
    p_display_count: *mut u32,
    p_displays: *mut DisplayKHR,
) -> Result;
pub type PFN_vkGetDisplayModePropertiesKHR = unsafe extern "system" fn(
    physical_device: PhysicalDevice,
    display: DisplayKHR,
    p_property_count: *mut u32,
    p_properties: *mut DisplayModePropertiesKHR,
) -> Result;
pub type PFN_vkCreateDisplayModeKHR = unsafe extern "system" fn(
    physical_device: PhysicalDevice,
    display: DisplayKHR,
    p_create_info: *const DisplayModeCreateInfoKHR,
    p_allocator: *const AllocationCallbacks,
    p_mode: *mut DisplayModeKHR,
) -> Result;
pub type PFN_vkGetDisplayPlaneCapabilitiesKHR = unsafe extern "system" fn(
    physical_device: PhysicalDevice,
    mode: DisplayModeKHR,
    plane_index: u32,
    p_capabilities: *mut DisplayPlaneCapabilitiesKHR,
) -> Result;
pub type PFN_vkCreateDisplayPlaneSurfaceKHR = unsafe extern "system" fn(
    instance: Instance,
    p_create_info: *const DisplaySurfaceCreateInfoKHR,
    p_allocator: *const AllocationCallbacks,
    p_surface: *mut SurfaceKHR,
) -> Result;
