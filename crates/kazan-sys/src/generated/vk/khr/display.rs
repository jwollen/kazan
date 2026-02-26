#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{c_char, c_int, c_void};
use core::marker::PhantomData;
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct DisplayKHR(u64);
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct DisplayModeKHR(u64);
#[repr(C)]
#[derive(Copy, Clone)]
pub struct DisplayPropertiesKHR<'a> {
    pub display: DisplayKHR,
    pub display_name: *const c_char,
    pub physical_dimensions: Extent2D,
    pub physical_resolution: Extent2D,
    pub supported_transforms: SurfaceTransformFlagsKHR,
    pub plane_reorder_possible: Bool32,
    pub persistent_content: Bool32,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for DisplayPropertiesKHR<'_> {
    fn default() -> Self {
        Self {
            display: Default::default(),
            display_name: core::ptr::null(),
            physical_dimensions: Default::default(),
            physical_resolution: Default::default(),
            supported_transforms: Default::default(),
            plane_reorder_possible: Default::default(),
            persistent_content: Default::default(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct DisplayPlanePropertiesKHR {
    pub current_display: DisplayKHR,
    pub current_stack_index: u32,
}
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct DisplayModeParametersKHR {
    pub visible_region: Extent2D,
    pub refresh_rate: u32,
}
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct DisplayModePropertiesKHR {
    pub display_mode: DisplayModeKHR,
    pub parameters: DisplayModeParametersKHR,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct DisplayModeCreateInfoKHR<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: DisplayModeCreateFlagsKHR,
    pub parameters: DisplayModeParametersKHR,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for DisplayModeCreateInfoKHR<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::DISPLAY_MODE_CREATE_INFO_KHR,
            p_next: core::ptr::null(),
            flags: Default::default(),
            parameters: Default::default(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Default)]
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
pub struct DisplaySurfaceCreateInfoKHR<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: DisplaySurfaceCreateFlagsKHR,
    pub display_mode: DisplayModeKHR,
    pub plane_index: u32,
    pub plane_stack_index: u32,
    pub transform: SurfaceTransformFlagBitsKHR,
    pub global_alpha: f32,
    pub alpha_mode: DisplayPlaneAlphaFlagBitsKHR,
    pub image_extent: Extent2D,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for DisplaySurfaceCreateInfoKHR<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::DISPLAY_SURFACE_CREATE_INFO_KHR,
            p_next: core::ptr::null(),
            flags: Default::default(),
            display_mode: Default::default(),
            plane_index: Default::default(),
            plane_stack_index: Default::default(),
            transform: Default::default(),
            global_alpha: Default::default(),
            alpha_mode: Default::default(),
            image_extent: Default::default(),
            _marker: PhantomData,
        }
    }
}
bitflags! {
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct DisplayPlaneAlphaFlagsKHR: Flags {
        const OPAQUE_KHR = DisplayPlaneAlphaFlagBitsKHR::OPAQUE_KHR.0;
        const GLOBAL_KHR = DisplayPlaneAlphaFlagBitsKHR::GLOBAL_KHR.0;
        const PER_PIXEL_KHR = DisplayPlaneAlphaFlagBitsKHR::PER_PIXEL_KHR.0;
        const PER_PIXEL_PREMULTIPLIED_KHR = DisplayPlaneAlphaFlagBitsKHR::PER_PIXEL_PREMULTIPLIED_KHR.0;
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DisplayPlaneAlphaFlagBitsKHR(u32);
impl DisplayPlaneAlphaFlagBitsKHR {
    pub const OPAQUE_KHR: Self = Self(1 << 0);
    pub const GLOBAL_KHR: Self = Self(1 << 1);
    pub const PER_PIXEL_KHR: Self = Self(1 << 2);
    pub const PER_PIXEL_PREMULTIPLIED_KHR: Self = Self(1 << 3);
}
bitflags! {
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct SurfaceTransformFlagsKHR: Flags {
        const IDENTITY_KHR = SurfaceTransformFlagBitsKHR::IDENTITY_KHR.0;
        const ROTATE_90_KHR = SurfaceTransformFlagBitsKHR::ROTATE_90_KHR.0;
        const ROTATE_180_KHR = SurfaceTransformFlagBitsKHR::ROTATE_180_KHR.0;
        const ROTATE_270_KHR = SurfaceTransformFlagBitsKHR::ROTATE_270_KHR.0;
        const HORIZONTAL_MIRROR_KHR = SurfaceTransformFlagBitsKHR::HORIZONTAL_MIRROR_KHR.0;
        const HORIZONTAL_MIRROR_ROTATE_90_KHR = SurfaceTransformFlagBitsKHR::HORIZONTAL_MIRROR_ROTATE_90_KHR.0;
        const HORIZONTAL_MIRROR_ROTATE_180_KHR = SurfaceTransformFlagBitsKHR::HORIZONTAL_MIRROR_ROTATE_180_KHR.0;
        const HORIZONTAL_MIRROR_ROTATE_270_KHR = SurfaceTransformFlagBitsKHR::HORIZONTAL_MIRROR_ROTATE_270_KHR.0;
        const INHERIT_KHR = SurfaceTransformFlagBitsKHR::INHERIT_KHR.0;
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SurfaceTransformFlagBitsKHR(u32);
impl SurfaceTransformFlagBitsKHR {
    pub const IDENTITY_KHR: Self = Self(1 << 0);
    pub const ROTATE_90_KHR: Self = Self(1 << 1);
    pub const ROTATE_180_KHR: Self = Self(1 << 2);
    pub const ROTATE_270_KHR: Self = Self(1 << 3);
    pub const HORIZONTAL_MIRROR_KHR: Self = Self(1 << 4);
    pub const HORIZONTAL_MIRROR_ROTATE_90_KHR: Self = Self(1 << 5);
    pub const HORIZONTAL_MIRROR_ROTATE_180_KHR: Self = Self(1 << 6);
    pub const HORIZONTAL_MIRROR_ROTATE_270_KHR: Self = Self(1 << 7);
    pub const INHERIT_KHR: Self = Self(1 << 8);
}
bitflags! {
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct DisplayModeCreateFlagsKHR: Flags {
    }
}
bitflags! {
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct DisplaySurfaceCreateFlagsKHR: Flags {
    }
}
pub type PFN_vkGetPhysicalDeviceDisplayPropertiesKHR = unsafe extern "system" fn(
    physical_device: PhysicalDevice,
    p_property_count: *mut u32,
    p_properties: *mut DisplayPropertiesKHR<'_>,
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
    p_create_info: *const DisplayModeCreateInfoKHR<'_>,
    p_allocator: *const AllocationCallbacks<'_>,
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
    p_create_info: *const DisplaySurfaceCreateInfoKHR<'_>,
    p_allocator: *const AllocationCallbacks<'_>,
    p_surface: *mut SurfaceKHR,
) -> Result;
