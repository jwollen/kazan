#![allow(unused_imports)]
use crate::{vk::Result as VkResult, vk::*, *};
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
pub(super) mod defs {
    #![allow(non_camel_case_types, unused_imports)]
    use crate::{vk::*, *};
    use core::ffi::{CStr, c_char, c_int, c_void};
    use core::fmt;
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
    unsafe impl<'a> TaggedStructure<'a> for PipelineCoverageModulationStateCreateInfoNV<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PIPELINE_COVERAGE_MODULATION_STATE_CREATE_INFO_NV;
    }
    unsafe impl<'a> Extends<PipelineMultisampleStateCreateInfo<'a>>
        for PipelineCoverageModulationStateCreateInfoNV<'a>
    {
    }
    impl Default for PipelineCoverageModulationStateCreateInfoNV<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
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
            coverage_modulation_table_enable: bool,
        ) -> Self {
            self.coverage_modulation_table_enable = coverage_modulation_table_enable.into();
            self
        }
        pub fn coverage_modulation_table(mut self, coverage_modulation_table: &'a [f32]) -> Self {
            self.coverage_modulation_table_count =
                coverage_modulation_table.len().try_into().unwrap();
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
    impl fmt::Debug for CoverageModulationModeNV {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let name = match *self {
                Self::NONE_NV => Some("NONE_NV"),
                Self::RGB_NV => Some("RGB_NV"),
                Self::ALPHA_NV => Some("ALPHA_NV"),
                Self::RGBA_NV => Some("RGBA_NV"),
                _ => None,
            };
            if let Some(name) = name {
                f.write_str(name)
            } else {
                self.0.fmt(f)
            }
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Hash)]
    pub struct PipelineCoverageModulationStateCreateFlagsNV(Flags);
    vk_bitflags_wrapped!(PipelineCoverageModulationStateCreateFlagsNV, Flags);
    impl PipelineCoverageModulationStateCreateFlagsNV {}
}
