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
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct PipelineCoverageToColorStateCreateInfoNV<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub flags: PipelineCoverageToColorStateCreateFlagsNV,
        pub coverage_to_color_enable: Bool32,
        pub coverage_to_color_location: u32,
        pub _marker: PhantomData<&'a ()>,
    }
    unsafe impl<'a> TaggedStructure<'a> for PipelineCoverageToColorStateCreateInfoNV<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PIPELINE_COVERAGE_TO_COLOR_STATE_CREATE_INFO_NV;
    }
    unsafe impl<'a> Extends<PipelineMultisampleStateCreateInfo<'a>>
        for PipelineCoverageToColorStateCreateInfoNV<'a>
    {
    }
    impl Default for PipelineCoverageToColorStateCreateInfoNV<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null(),
                flags: Default::default(),
                coverage_to_color_enable: Default::default(),
                coverage_to_color_location: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> PipelineCoverageToColorStateCreateInfoNV<'a> {
        pub fn flags(mut self, flags: PipelineCoverageToColorStateCreateFlagsNV) -> Self {
            self.flags = flags;
            self
        }
        pub fn coverage_to_color_enable(mut self, coverage_to_color_enable: bool) -> Self {
            self.coverage_to_color_enable = coverage_to_color_enable.into();
            self
        }
        pub fn coverage_to_color_location(mut self, coverage_to_color_location: u32) -> Self {
            self.coverage_to_color_location = coverage_to_color_location;
            self
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Hash)]
    pub struct PipelineCoverageToColorStateCreateFlagsNV(Flags);
    vk_bitflags_wrapped!(PipelineCoverageToColorStateCreateFlagsNV, Flags);
}
