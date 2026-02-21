#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{c_char, c_int, c_void};
pub type AttachmentSampleCountInfoNV = AttachmentSampleCountInfoAMD;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PipelineCoverageModulationStateCreateInfoNV {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: PipelineCoverageModulationStateCreateFlagsNV,
    pub coverage_modulation_mode: CoverageModulationModeNV,
    pub coverage_modulation_table_enable: Bool32,
    pub coverage_modulation_table_count: u32,
    pub p_coverage_modulation_table: *const f32,
}
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CoverageModulationModeNV(i32);
impl CoverageModulationModeNV {
    pub const NONE_NV: Self = Self(0);
    pub const RGB_NV: Self = Self(1);
    pub const ALPHA_NV: Self = Self(2);
    pub const RGBA_NV: Self = Self(3);
}
bitflags! {
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Default)]
    pub struct PipelineCoverageModulationStateCreateFlagsNV: Flags {
    }
}
