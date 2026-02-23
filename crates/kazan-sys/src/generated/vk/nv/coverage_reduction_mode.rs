#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{c_char, c_int, c_void};
#[repr(C)]
pub struct PhysicalDeviceCoverageReductionModeFeaturesNV {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub coverage_reduction_mode: Bool32,
}
#[repr(C)]
pub struct PipelineCoverageReductionStateCreateInfoNV {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: PipelineCoverageReductionStateCreateFlagsNV,
    pub coverage_reduction_mode: CoverageReductionModeNV,
}
#[repr(C)]
pub struct FramebufferMixedSamplesCombinationNV {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub coverage_reduction_mode: CoverageReductionModeNV,
    pub rasterization_samples: SampleCountFlagBits,
    pub depth_stencil_samples: SampleCountFlags,
    pub color_samples: SampleCountFlags,
}
#[repr(transparent)]
#[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CoverageReductionModeNV(i32);
impl CoverageReductionModeNV {
    pub const MERGE_NV: Self = Self(0);
    pub const TRUNCATE_NV: Self = Self(1);
}
bitflags! {
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Default)]
    pub struct PipelineCoverageReductionStateCreateFlagsNV: Flags {
    }
}
pub type PFN_vkGetPhysicalDeviceSupportedFramebufferMixedSamplesCombinationsNV =
    unsafe extern "system" fn(
        physical_device: PhysicalDevice,
        p_combination_count: *mut u32,
        p_combinations: *mut FramebufferMixedSamplesCombinationNV,
    ) -> Result;
