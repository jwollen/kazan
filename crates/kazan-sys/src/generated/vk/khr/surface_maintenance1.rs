#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{CStr, c_char, c_int, c_void};
use core::marker::PhantomData;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SurfacePresentModeKHR<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub present_mode: PresentModeKHR,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for SurfacePresentModeKHR<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::SURFACE_PRESENT_MODE_KHR,
            p_next: core::ptr::null_mut(),
            present_mode: Default::default(),
            _marker: PhantomData,
        }
    }
}
impl<'a> SurfacePresentModeKHR<'a> {
    pub fn present_mode(mut self, present_mode: PresentModeKHR) -> Self {
        self.present_mode = present_mode;
        self
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SurfacePresentScalingCapabilitiesKHR<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub supported_present_scaling: PresentScalingFlagsKHR,
    pub supported_present_gravity_x: PresentGravityFlagsKHR,
    pub supported_present_gravity_y: PresentGravityFlagsKHR,
    pub min_scaled_image_extent: Extent2D,
    pub max_scaled_image_extent: Extent2D,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for SurfacePresentScalingCapabilitiesKHR<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::SURFACE_PRESENT_SCALING_CAPABILITIES_KHR,
            p_next: core::ptr::null_mut(),
            supported_present_scaling: Default::default(),
            supported_present_gravity_x: Default::default(),
            supported_present_gravity_y: Default::default(),
            min_scaled_image_extent: Default::default(),
            max_scaled_image_extent: Default::default(),
            _marker: PhantomData,
        }
    }
}
impl<'a> SurfacePresentScalingCapabilitiesKHR<'a> {
    pub fn supported_present_scaling(
        mut self,
        supported_present_scaling: PresentScalingFlagsKHR,
    ) -> Self {
        self.supported_present_scaling = supported_present_scaling;
        self
    }
    pub fn supported_present_gravity_x(
        mut self,
        supported_present_gravity_x: PresentGravityFlagsKHR,
    ) -> Self {
        self.supported_present_gravity_x = supported_present_gravity_x;
        self
    }
    pub fn supported_present_gravity_y(
        mut self,
        supported_present_gravity_y: PresentGravityFlagsKHR,
    ) -> Self {
        self.supported_present_gravity_y = supported_present_gravity_y;
        self
    }
    pub fn min_scaled_image_extent(mut self, min_scaled_image_extent: Extent2D) -> Self {
        self.min_scaled_image_extent = min_scaled_image_extent;
        self
    }
    pub fn max_scaled_image_extent(mut self, max_scaled_image_extent: Extent2D) -> Self {
        self.max_scaled_image_extent = max_scaled_image_extent;
        self
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SurfacePresentModeCompatibilityKHR<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub present_mode_count: u32,
    pub p_present_modes: *mut PresentModeKHR,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for SurfacePresentModeCompatibilityKHR<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::SURFACE_PRESENT_MODE_COMPATIBILITY_KHR,
            p_next: core::ptr::null_mut(),
            present_mode_count: Default::default(),
            p_present_modes: core::ptr::null_mut(),
            _marker: PhantomData,
        }
    }
}
impl<'a> SurfacePresentModeCompatibilityKHR<'a> {
    pub fn present_modes(mut self, present_modes: &'a mut [PresentModeKHR]) -> Self {
        self.present_mode_count = present_modes.len().try_into().unwrap();
        self.p_present_modes = present_modes.as_mut_ptr();
        self
    }
}
bitflags! {
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
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
#[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PresentScalingFlagBitsKHR(u32);
impl PresentScalingFlagBitsKHR {
    pub const ONE_TO_ONE_KHR: Self = Self(1 << 0);
    pub const ASPECT_RATIO_STRETCH_KHR: Self = Self(1 << 1);
    pub const STRETCH_KHR: Self = Self(1 << 2);
}
bitflags! {
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
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
#[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PresentGravityFlagBitsKHR(u32);
impl PresentGravityFlagBitsKHR {
    pub const MIN_KHR: Self = Self(1 << 0);
    pub const MAX_KHR: Self = Self(1 << 1);
    pub const CENTERED_KHR: Self = Self(1 << 2);
}
