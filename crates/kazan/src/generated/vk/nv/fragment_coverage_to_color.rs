#![allow(unused_imports)]
use crate::{vk::Result as VkResult, vk::*, *};
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;

pub const EXTENSION_NAME: &CStr = c"VK_NV_fragment_coverage_to_color";

pub(super) mod defs {
    #![allow(non_camel_case_types, unused_imports)]
    use crate::{vk::*, *};
    use core::ffi::{CStr, c_char, c_int, c_void};
    use core::fmt;
    use core::marker::PhantomData;

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPipelineCoverageToColorStateCreateInfoNV.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct PipelineCoverageToColorStateCreateInfoNV<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub flags: PipelineCoverageToColorStateCreateFlagsNV,
        pub coverage_to_color_enable: Bool32,
        pub coverage_to_color_location: u32,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for PipelineCoverageToColorStateCreateInfoNV<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("PipelineCoverageToColorStateCreateInfoNV")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("flags", &self.flags)
                .field("coverage_to_color_enable", &self.coverage_to_color_enable)
                .field(
                    "coverage_to_color_location",
                    &self.coverage_to_color_location,
                )
                .finish()
        }
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
        #[inline]
        pub fn flags(mut self, flags: PipelineCoverageToColorStateCreateFlagsNV) -> Self {
            self.flags = flags;
            self
        }

        #[inline]
        pub fn coverage_to_color_enable(mut self, coverage_to_color_enable: bool) -> Self {
            self.coverage_to_color_enable = coverage_to_color_enable.into();
            self
        }

        #[inline]
        pub fn coverage_to_color_location(mut self, coverage_to_color_location: u32) -> Self {
            self.coverage_to_color_location = coverage_to_color_location;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPipelineCoverageToColorStateCreateFlagsNV.html>
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Hash)]
    pub struct PipelineCoverageToColorStateCreateFlagsNV(Flags);
    vk_bitflags_wrapped!(PipelineCoverageToColorStateCreateFlagsNV, Flags);

    impl fmt::Debug for PipelineCoverageToColorStateCreateFlagsNV {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            debug_flags(f, &[], self.0)
        }
    }
}
