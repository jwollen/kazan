#![allow(unused_imports)]
use crate::{vk::Result as VkResult, vk::*, *};
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;

pub const EXTENSION_NAME: &CStr = c"VK_EXT_conservative_rasterization";

pub(super) mod defs {
    #![allow(non_camel_case_types, unused_imports)]
    use crate::{vk::*, *};
    use core::ffi::{CStr, c_char, c_int, c_void};
    use core::fmt;
    use core::marker::PhantomData;

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPhysicalDeviceConservativeRasterizationPropertiesEXT.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct PhysicalDeviceConservativeRasterizationPropertiesEXT<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub primitive_overestimation_size: f32,
        pub max_extra_primitive_overestimation_size: f32,
        pub extra_primitive_overestimation_size_granularity: f32,
        pub primitive_underestimation: Bool32,
        pub conservative_point_and_line_rasterization: Bool32,
        pub degenerate_triangles_rasterized: Bool32,
        pub degenerate_lines_rasterized: Bool32,
        pub fully_covered_fragment_shader_input_variable: Bool32,
        pub conservative_rasterization_post_depth_coverage: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for PhysicalDeviceConservativeRasterizationPropertiesEXT<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("PhysicalDeviceConservativeRasterizationPropertiesEXT")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field(
                    "primitive_overestimation_size",
                    &self.primitive_overestimation_size,
                )
                .field(
                    "max_extra_primitive_overestimation_size",
                    &self.max_extra_primitive_overestimation_size,
                )
                .field(
                    "extra_primitive_overestimation_size_granularity",
                    &self.extra_primitive_overestimation_size_granularity,
                )
                .field("primitive_underestimation", &self.primitive_underestimation)
                .field(
                    "conservative_point_and_line_rasterization",
                    &self.conservative_point_and_line_rasterization,
                )
                .field(
                    "degenerate_triangles_rasterized",
                    &self.degenerate_triangles_rasterized,
                )
                .field(
                    "degenerate_lines_rasterized",
                    &self.degenerate_lines_rasterized,
                )
                .field(
                    "fully_covered_fragment_shader_input_variable",
                    &self.fully_covered_fragment_shader_input_variable,
                )
                .field(
                    "conservative_rasterization_post_depth_coverage",
                    &self.conservative_rasterization_post_depth_coverage,
                )
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for PhysicalDeviceConservativeRasterizationPropertiesEXT<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PHYSICAL_DEVICE_CONSERVATIVE_RASTERIZATION_PROPERTIES_EXT;
    }

    unsafe impl<'a> Extends<PhysicalDeviceProperties2<'a>>
        for PhysicalDeviceConservativeRasterizationPropertiesEXT<'a>
    {
    }

    impl Default for PhysicalDeviceConservativeRasterizationPropertiesEXT<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null_mut(),
                primitive_overestimation_size: Default::default(),
                max_extra_primitive_overestimation_size: Default::default(),
                extra_primitive_overestimation_size_granularity: Default::default(),
                primitive_underestimation: Default::default(),
                conservative_point_and_line_rasterization: Default::default(),
                degenerate_triangles_rasterized: Default::default(),
                degenerate_lines_rasterized: Default::default(),
                fully_covered_fragment_shader_input_variable: Default::default(),
                conservative_rasterization_post_depth_coverage: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> PhysicalDeviceConservativeRasterizationPropertiesEXT<'a> {
        pub fn primitive_overestimation_size(mut self, primitive_overestimation_size: f32) -> Self {
            self.primitive_overestimation_size = primitive_overestimation_size;
            self
        }

        pub fn max_extra_primitive_overestimation_size(
            mut self,
            max_extra_primitive_overestimation_size: f32,
        ) -> Self {
            self.max_extra_primitive_overestimation_size = max_extra_primitive_overestimation_size;
            self
        }

        pub fn extra_primitive_overestimation_size_granularity(
            mut self,
            extra_primitive_overestimation_size_granularity: f32,
        ) -> Self {
            self.extra_primitive_overestimation_size_granularity =
                extra_primitive_overestimation_size_granularity;
            self
        }

        pub fn primitive_underestimation(mut self, primitive_underestimation: bool) -> Self {
            self.primitive_underestimation = primitive_underestimation.into();
            self
        }

        pub fn conservative_point_and_line_rasterization(
            mut self,
            conservative_point_and_line_rasterization: bool,
        ) -> Self {
            self.conservative_point_and_line_rasterization =
                conservative_point_and_line_rasterization.into();
            self
        }

        pub fn degenerate_triangles_rasterized(
            mut self,
            degenerate_triangles_rasterized: bool,
        ) -> Self {
            self.degenerate_triangles_rasterized = degenerate_triangles_rasterized.into();
            self
        }

        pub fn degenerate_lines_rasterized(mut self, degenerate_lines_rasterized: bool) -> Self {
            self.degenerate_lines_rasterized = degenerate_lines_rasterized.into();
            self
        }

        pub fn fully_covered_fragment_shader_input_variable(
            mut self,
            fully_covered_fragment_shader_input_variable: bool,
        ) -> Self {
            self.fully_covered_fragment_shader_input_variable =
                fully_covered_fragment_shader_input_variable.into();
            self
        }

        pub fn conservative_rasterization_post_depth_coverage(
            mut self,
            conservative_rasterization_post_depth_coverage: bool,
        ) -> Self {
            self.conservative_rasterization_post_depth_coverage =
                conservative_rasterization_post_depth_coverage.into();
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPipelineRasterizationConservativeStateCreateInfoEXT.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct PipelineRasterizationConservativeStateCreateInfoEXT<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub flags: PipelineRasterizationConservativeStateCreateFlagsEXT,
        pub conservative_rasterization_mode: ConservativeRasterizationModeEXT,
        pub extra_primitive_overestimation_size: f32,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for PipelineRasterizationConservativeStateCreateInfoEXT<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("PipelineRasterizationConservativeStateCreateInfoEXT")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("flags", &self.flags)
                .field(
                    "conservative_rasterization_mode",
                    &self.conservative_rasterization_mode,
                )
                .field(
                    "extra_primitive_overestimation_size",
                    &self.extra_primitive_overestimation_size,
                )
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for PipelineRasterizationConservativeStateCreateInfoEXT<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PIPELINE_RASTERIZATION_CONSERVATIVE_STATE_CREATE_INFO_EXT;
    }

    unsafe impl<'a> Extends<PipelineRasterizationStateCreateInfo<'a>>
        for PipelineRasterizationConservativeStateCreateInfoEXT<'a>
    {
    }

    impl Default for PipelineRasterizationConservativeStateCreateInfoEXT<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null(),
                flags: Default::default(),
                conservative_rasterization_mode: Default::default(),
                extra_primitive_overestimation_size: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> PipelineRasterizationConservativeStateCreateInfoEXT<'a> {
        pub fn flags(
            mut self,
            flags: PipelineRasterizationConservativeStateCreateFlagsEXT,
        ) -> Self {
            self.flags = flags;
            self
        }

        pub fn conservative_rasterization_mode(
            mut self,
            conservative_rasterization_mode: ConservativeRasterizationModeEXT,
        ) -> Self {
            self.conservative_rasterization_mode = conservative_rasterization_mode;
            self
        }

        pub fn extra_primitive_overestimation_size(
            mut self,
            extra_primitive_overestimation_size: f32,
        ) -> Self {
            self.extra_primitive_overestimation_size = extra_primitive_overestimation_size;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkConservativeRasterizationModeEXT.html>
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct ConservativeRasterizationModeEXT(i32);

    impl ConservativeRasterizationModeEXT {
        pub const DISABLED_EXT: Self = Self(0);
        pub const OVERESTIMATE_EXT: Self = Self(1);
        pub const UNDERESTIMATE_EXT: Self = Self(2);
    }

    impl fmt::Debug for ConservativeRasterizationModeEXT {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let name = match *self {
                Self::DISABLED_EXT => Some("DISABLED_EXT"),
                Self::OVERESTIMATE_EXT => Some("OVERESTIMATE_EXT"),
                Self::UNDERESTIMATE_EXT => Some("UNDERESTIMATE_EXT"),
                _ => None,
            };
            if let Some(name) = name {
                f.write_str(name)
            } else {
                self.0.fmt(f)
            }
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPipelineRasterizationConservativeStateCreateFlagsEXT.html>
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Hash)]
    pub struct PipelineRasterizationConservativeStateCreateFlagsEXT(Flags);
    vk_bitflags_wrapped!(PipelineRasterizationConservativeStateCreateFlagsEXT, Flags);

    impl fmt::Debug for PipelineRasterizationConservativeStateCreateFlagsEXT {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            debug_flags(f, &[], self.0)
        }
    }
}
