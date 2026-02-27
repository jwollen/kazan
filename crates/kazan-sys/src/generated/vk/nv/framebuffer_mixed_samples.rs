#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{CStr, c_char, c_int, c_void};
use core::marker::PhantomData;
pub type AttachmentSampleCountInfoNV<'a> = AttachmentSampleCountInfoAMD<'a>;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PipelineCoverageModulationStateCreateInfoNV<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: PipelineCoverageModulationStateCreateFlagsNV,
    pub coverage_modulation_mode: CoverageModulationModeNV,
    pub coverage_modulation_table_enable: Bool32,
    pub coverage_modulation_table_count: u32,
    pub p_coverage_modulation_table: *const f32,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for PipelineCoverageModulationStateCreateInfoNV<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::PIPELINE_COVERAGE_MODULATION_STATE_CREATE_INFO_NV,
            p_next: core::ptr::null(),
            flags: Default::default(),
            coverage_modulation_mode: Default::default(),
            coverage_modulation_table_enable: Default::default(),
            coverage_modulation_table_count: Default::default(),
            p_coverage_modulation_table: core::ptr::null(),
            _marker: PhantomData,
        }
    }
}
impl<'a> PipelineCoverageModulationStateCreateInfoNV<'a> {
    pub fn flags(mut self, flags: PipelineCoverageModulationStateCreateFlagsNV) -> Self {
        self.flags = flags;
        self
    }
    pub fn coverage_modulation_mode(
        mut self,
        coverage_modulation_mode: CoverageModulationModeNV,
    ) -> Self {
        self.coverage_modulation_mode = coverage_modulation_mode;
        self
    }
    pub fn coverage_modulation_table_enable(
        mut self,
        coverage_modulation_table_enable: Bool32,
    ) -> Self {
        self.coverage_modulation_table_enable = coverage_modulation_table_enable;
        self
    }
    pub fn coverage_modulation_table(mut self, coverage_modulation_table: &'a [f32]) -> Self {
        self.coverage_modulation_table_count = coverage_modulation_table.len().try_into().unwrap();
        self.p_coverage_modulation_table = coverage_modulation_table.as_ptr();
        self
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CoverageModulationModeNV(i32);
impl CoverageModulationModeNV {
    pub const NONE_NV: Self = Self(0);
    pub const RGB_NV: Self = Self(1);
    pub const ALPHA_NV: Self = Self(2);
    pub const RGBA_NV: Self = Self(3);
}
bitflags! {
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct PipelineCoverageModulationStateCreateFlagsNV: Flags {
    }
}
