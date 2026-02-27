#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{CStr, c_char, c_int, c_void};
use core::marker::PhantomData;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceCoverageReductionModeFeaturesNV<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub coverage_reduction_mode: Bool32,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for PhysicalDeviceCoverageReductionModeFeaturesNV<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::PHYSICAL_DEVICE_COVERAGE_REDUCTION_MODE_FEATURES_NV,
            p_next: core::ptr::null_mut(),
            coverage_reduction_mode: Default::default(),
            _marker: PhantomData,
        }
    }
}
impl<'a> PhysicalDeviceCoverageReductionModeFeaturesNV<'a> {
    pub fn coverage_reduction_mode(mut self, coverage_reduction_mode: Bool32) -> Self {
        self.coverage_reduction_mode = coverage_reduction_mode;
        self
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PipelineCoverageReductionStateCreateInfoNV<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: PipelineCoverageReductionStateCreateFlagsNV,
    pub coverage_reduction_mode: CoverageReductionModeNV,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for PipelineCoverageReductionStateCreateInfoNV<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::PIPELINE_COVERAGE_REDUCTION_STATE_CREATE_INFO_NV,
            p_next: core::ptr::null(),
            flags: Default::default(),
            coverage_reduction_mode: Default::default(),
            _marker: PhantomData,
        }
    }
}
impl<'a> PipelineCoverageReductionStateCreateInfoNV<'a> {
    pub fn flags(mut self, flags: PipelineCoverageReductionStateCreateFlagsNV) -> Self {
        self.flags = flags;
        self
    }
    pub fn coverage_reduction_mode(
        mut self,
        coverage_reduction_mode: CoverageReductionModeNV,
    ) -> Self {
        self.coverage_reduction_mode = coverage_reduction_mode;
        self
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct FramebufferMixedSamplesCombinationNV<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub coverage_reduction_mode: CoverageReductionModeNV,
    pub rasterization_samples: SampleCountFlagBits,
    pub depth_stencil_samples: SampleCountFlags,
    pub color_samples: SampleCountFlags,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for FramebufferMixedSamplesCombinationNV<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::FRAMEBUFFER_MIXED_SAMPLES_COMBINATION_NV,
            p_next: core::ptr::null_mut(),
            coverage_reduction_mode: Default::default(),
            rasterization_samples: Default::default(),
            depth_stencil_samples: Default::default(),
            color_samples: Default::default(),
            _marker: PhantomData,
        }
    }
}
impl<'a> FramebufferMixedSamplesCombinationNV<'a> {
    pub fn coverage_reduction_mode(
        mut self,
        coverage_reduction_mode: CoverageReductionModeNV,
    ) -> Self {
        self.coverage_reduction_mode = coverage_reduction_mode;
        self
    }
    pub fn rasterization_samples(mut self, rasterization_samples: SampleCountFlagBits) -> Self {
        self.rasterization_samples = rasterization_samples;
        self
    }
    pub fn depth_stencil_samples(mut self, depth_stencil_samples: SampleCountFlags) -> Self {
        self.depth_stencil_samples = depth_stencil_samples;
        self
    }
    pub fn color_samples(mut self, color_samples: SampleCountFlags) -> Self {
        self.color_samples = color_samples;
        self
    }
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
    #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct PipelineCoverageReductionStateCreateFlagsNV: Flags {
    }
}
pub type PFN_vkGetPhysicalDeviceSupportedFramebufferMixedSamplesCombinationsNV =
    unsafe extern "system" fn(
        physical_device: PhysicalDevice,
        p_combination_count: *mut u32,
        p_combinations: *mut FramebufferMixedSamplesCombinationNV<'_>,
    ) -> Result;
