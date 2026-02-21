#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{c_char, c_int, c_void};
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SurfacePresentModeKHR {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub present_mode: PresentModeKHR,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SurfacePresentScalingCapabilitiesKHR {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub supported_present_scaling: PresentScalingFlagsKHR,
    pub supported_present_gravity_x: PresentGravityFlagsKHR,
    pub supported_present_gravity_y: PresentGravityFlagsKHR,
    pub min_scaled_image_extent: Extent2D,
    pub max_scaled_image_extent: Extent2D,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SurfacePresentModeCompatibilityKHR {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub present_mode_count: u32,
    pub p_present_modes: *mut PresentModeKHR,
}
bitflags! {
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Default)]
    pub struct PresentScalingFlagsKHR: Flags {
        const ONE_TO_ONE_KHR = 1 << 0;
        const ASPECT_RATIO_STRETCH_KHR = 1 << 1;
        const STRETCH_KHR = 1 << 2;
    }
}
bitflags! {
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Default)]
    pub struct PresentGravityFlagsKHR: Flags {
        const MIN_KHR = 1 << 0;
        const MAX_KHR = 1 << 1;
        const CENTERED_KHR = 1 << 2;
    }
}
