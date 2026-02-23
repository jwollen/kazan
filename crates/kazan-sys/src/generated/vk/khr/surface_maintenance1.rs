#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{c_char, c_int, c_void};
#[repr(C)]
pub struct SurfacePresentModeKHR {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub present_mode: PresentModeKHR,
}
#[repr(C)]
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
        const ONE_TO_ONE_KHR = PresentScalingFlagBitsKHR::ONE_TO_ONE_KHR.0;
        const ASPECT_RATIO_STRETCH_KHR = PresentScalingFlagBitsKHR::ASPECT_RATIO_STRETCH_KHR.0;
        const STRETCH_KHR = PresentScalingFlagBitsKHR::STRETCH_KHR.0;
        const ONE_TO_ONE_EXT = Self::ONE_TO_ONE_KHR.bits();
        const ASPECT_RATIO_STRETCH_EXT = Self::ASPECT_RATIO_STRETCH_KHR.bits();
        const STRETCH_EXT = Self::STRETCH_KHR.bits();
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PresentScalingFlagBitsKHR(u32);
impl PresentScalingFlagBitsKHR {
    pub const ONE_TO_ONE_KHR: Self = Self(1 << 0);
    pub const ASPECT_RATIO_STRETCH_KHR: Self = Self(1 << 1);
    pub const STRETCH_KHR: Self = Self(1 << 2);
}
bitflags! {
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Default)]
    pub struct PresentGravityFlagsKHR: Flags {
        const MIN_KHR = PresentGravityFlagBitsKHR::MIN_KHR.0;
        const MAX_KHR = PresentGravityFlagBitsKHR::MAX_KHR.0;
        const CENTERED_KHR = PresentGravityFlagBitsKHR::CENTERED_KHR.0;
        const MIN_EXT = Self::MIN_KHR.bits();
        const MAX_EXT = Self::MAX_KHR.bits();
        const CENTERED_EXT = Self::CENTERED_KHR.bits();
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PresentGravityFlagBitsKHR(u32);
impl PresentGravityFlagBitsKHR {
    pub const MIN_KHR: Self = Self(1 << 0);
    pub const MAX_KHR: Self = Self(1 << 1);
    pub const CENTERED_KHR: Self = Self(1 << 2);
}
