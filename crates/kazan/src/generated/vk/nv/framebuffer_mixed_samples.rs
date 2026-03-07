#![allow(unused_imports)]
use crate::{vk::Result as VkResult, vk::*, *};
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;

pub const EXTENSION_NAME: &CStr = c"VK_NV_framebuffer_mixed_samples";

pub(super) mod defs {
    #![allow(non_camel_case_types, unused_imports)]
    use crate::{vk::*, *};
    use core::ffi::{CStr, c_char, c_int, c_void};
    use core::fmt;
    use core::marker::PhantomData;

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkAttachmentSampleCountInfoNV.html>
    pub type AttachmentSampleCountInfoNV<'a> = AttachmentSampleCountInfoAMD<'a>;

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPipelineCoverageModulationStateCreateInfoNV.html>
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

    impl fmt::Debug for PipelineCoverageModulationStateCreateInfoNV<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("PipelineCoverageModulationStateCreateInfoNV")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("flags", &self.flags)
                .field("coverage_modulation_mode", &self.coverage_modulation_mode)
                .field(
                    "coverage_modulation_table_enable",
                    &self.coverage_modulation_table_enable,
                )
                .field(
                    "coverage_modulation_table_count",
                    &self.coverage_modulation_table_count,
                )
                .field(
                    "p_coverage_modulation_table",
                    &self.p_coverage_modulation_table,
                )
                .finish()
        }
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

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkCoverageModulationModeNV.html>
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

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPipelineCoverageModulationStateCreateFlagsNV.html>
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Hash)]
    pub struct PipelineCoverageModulationStateCreateFlagsNV(Flags);
    vk_bitflags_wrapped!(PipelineCoverageModulationStateCreateFlagsNV, Flags);

    impl fmt::Debug for PipelineCoverageModulationStateCreateFlagsNV {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            debug_flags(f, &[], self.0)
        }
    }
}
