//! <https://registry.khronos.org/vulkan/specs/latest/man/html/VK_EXT_extended_dynamic_state3.html>
#![allow(unused_imports)]
use crate::{vk::Result as VkResult, vk::*, *};
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
use core::ptr;

pub const EXTENSION_NAME: &CStr = c"VK_EXT_extended_dynamic_state3";

pub(super) mod defs {
    #![allow(non_camel_case_types, unused_imports)]
    use crate::{vk::*, *};
    use core::ffi::{CStr, c_char, c_int, c_void};
    use core::fmt;
    use core::marker::PhantomData;
    use core::ptr;

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPhysicalDeviceExtendedDynamicState3FeaturesEXT.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct PhysicalDeviceExtendedDynamicState3FeaturesEXT<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub extended_dynamic_state3_tessellation_domain_origin: Bool32,
        pub extended_dynamic_state3_depth_clamp_enable: Bool32,
        pub extended_dynamic_state3_polygon_mode: Bool32,
        pub extended_dynamic_state3_rasterization_samples: Bool32,
        pub extended_dynamic_state3_sample_mask: Bool32,
        pub extended_dynamic_state3_alpha_to_coverage_enable: Bool32,
        pub extended_dynamic_state3_alpha_to_one_enable: Bool32,
        pub extended_dynamic_state3_logic_op_enable: Bool32,
        pub extended_dynamic_state3_color_blend_enable: Bool32,
        pub extended_dynamic_state3_color_blend_equation: Bool32,
        pub extended_dynamic_state3_color_write_mask: Bool32,
        pub extended_dynamic_state3_rasterization_stream: Bool32,
        pub extended_dynamic_state3_conservative_rasterization_mode: Bool32,
        pub extended_dynamic_state3_extra_primitive_overestimation_size: Bool32,
        pub extended_dynamic_state3_depth_clip_enable: Bool32,
        pub extended_dynamic_state3_sample_locations_enable: Bool32,
        pub extended_dynamic_state3_color_blend_advanced: Bool32,
        pub extended_dynamic_state3_provoking_vertex_mode: Bool32,
        pub extended_dynamic_state3_line_rasterization_mode: Bool32,
        pub extended_dynamic_state3_line_stipple_enable: Bool32,
        pub extended_dynamic_state3_depth_clip_negative_one_to_one: Bool32,
        pub extended_dynamic_state3_viewport_w_scaling_enable: Bool32,
        pub extended_dynamic_state3_viewport_swizzle: Bool32,
        pub extended_dynamic_state3_coverage_to_color_enable: Bool32,
        pub extended_dynamic_state3_coverage_to_color_location: Bool32,
        pub extended_dynamic_state3_coverage_modulation_mode: Bool32,
        pub extended_dynamic_state3_coverage_modulation_table_enable: Bool32,
        pub extended_dynamic_state3_coverage_modulation_table: Bool32,
        pub extended_dynamic_state3_coverage_reduction_mode: Bool32,
        pub extended_dynamic_state3_representative_fragment_test_enable: Bool32,
        pub extended_dynamic_state3_shading_rate_image_enable: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for PhysicalDeviceExtendedDynamicState3FeaturesEXT<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("PhysicalDeviceExtendedDynamicState3FeaturesEXT")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field(
                    "extended_dynamic_state3_tessellation_domain_origin",
                    &self.extended_dynamic_state3_tessellation_domain_origin,
                )
                .field(
                    "extended_dynamic_state3_depth_clamp_enable",
                    &self.extended_dynamic_state3_depth_clamp_enable,
                )
                .field(
                    "extended_dynamic_state3_polygon_mode",
                    &self.extended_dynamic_state3_polygon_mode,
                )
                .field(
                    "extended_dynamic_state3_rasterization_samples",
                    &self.extended_dynamic_state3_rasterization_samples,
                )
                .field(
                    "extended_dynamic_state3_sample_mask",
                    &self.extended_dynamic_state3_sample_mask,
                )
                .field(
                    "extended_dynamic_state3_alpha_to_coverage_enable",
                    &self.extended_dynamic_state3_alpha_to_coverage_enable,
                )
                .field(
                    "extended_dynamic_state3_alpha_to_one_enable",
                    &self.extended_dynamic_state3_alpha_to_one_enable,
                )
                .field(
                    "extended_dynamic_state3_logic_op_enable",
                    &self.extended_dynamic_state3_logic_op_enable,
                )
                .field(
                    "extended_dynamic_state3_color_blend_enable",
                    &self.extended_dynamic_state3_color_blend_enable,
                )
                .field(
                    "extended_dynamic_state3_color_blend_equation",
                    &self.extended_dynamic_state3_color_blend_equation,
                )
                .field(
                    "extended_dynamic_state3_color_write_mask",
                    &self.extended_dynamic_state3_color_write_mask,
                )
                .field(
                    "extended_dynamic_state3_rasterization_stream",
                    &self.extended_dynamic_state3_rasterization_stream,
                )
                .field(
                    "extended_dynamic_state3_conservative_rasterization_mode",
                    &self.extended_dynamic_state3_conservative_rasterization_mode,
                )
                .field(
                    "extended_dynamic_state3_extra_primitive_overestimation_size",
                    &self.extended_dynamic_state3_extra_primitive_overestimation_size,
                )
                .field(
                    "extended_dynamic_state3_depth_clip_enable",
                    &self.extended_dynamic_state3_depth_clip_enable,
                )
                .field(
                    "extended_dynamic_state3_sample_locations_enable",
                    &self.extended_dynamic_state3_sample_locations_enable,
                )
                .field(
                    "extended_dynamic_state3_color_blend_advanced",
                    &self.extended_dynamic_state3_color_blend_advanced,
                )
                .field(
                    "extended_dynamic_state3_provoking_vertex_mode",
                    &self.extended_dynamic_state3_provoking_vertex_mode,
                )
                .field(
                    "extended_dynamic_state3_line_rasterization_mode",
                    &self.extended_dynamic_state3_line_rasterization_mode,
                )
                .field(
                    "extended_dynamic_state3_line_stipple_enable",
                    &self.extended_dynamic_state3_line_stipple_enable,
                )
                .field(
                    "extended_dynamic_state3_depth_clip_negative_one_to_one",
                    &self.extended_dynamic_state3_depth_clip_negative_one_to_one,
                )
                .field(
                    "extended_dynamic_state3_viewport_w_scaling_enable",
                    &self.extended_dynamic_state3_viewport_w_scaling_enable,
                )
                .field(
                    "extended_dynamic_state3_viewport_swizzle",
                    &self.extended_dynamic_state3_viewport_swizzle,
                )
                .field(
                    "extended_dynamic_state3_coverage_to_color_enable",
                    &self.extended_dynamic_state3_coverage_to_color_enable,
                )
                .field(
                    "extended_dynamic_state3_coverage_to_color_location",
                    &self.extended_dynamic_state3_coverage_to_color_location,
                )
                .field(
                    "extended_dynamic_state3_coverage_modulation_mode",
                    &self.extended_dynamic_state3_coverage_modulation_mode,
                )
                .field(
                    "extended_dynamic_state3_coverage_modulation_table_enable",
                    &self.extended_dynamic_state3_coverage_modulation_table_enable,
                )
                .field(
                    "extended_dynamic_state3_coverage_modulation_table",
                    &self.extended_dynamic_state3_coverage_modulation_table,
                )
                .field(
                    "extended_dynamic_state3_coverage_reduction_mode",
                    &self.extended_dynamic_state3_coverage_reduction_mode,
                )
                .field(
                    "extended_dynamic_state3_representative_fragment_test_enable",
                    &self.extended_dynamic_state3_representative_fragment_test_enable,
                )
                .field(
                    "extended_dynamic_state3_shading_rate_image_enable",
                    &self.extended_dynamic_state3_shading_rate_image_enable,
                )
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for PhysicalDeviceExtendedDynamicState3FeaturesEXT<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PHYSICAL_DEVICE_EXTENDED_DYNAMIC_STATE_3_FEATURES_EXT;
    }

    unsafe impl<'a> Extends<PhysicalDeviceFeatures2<'a>>
        for PhysicalDeviceExtendedDynamicState3FeaturesEXT<'a>
    {
    }
    unsafe impl<'a> Extends<DeviceCreateInfo<'a>>
        for PhysicalDeviceExtendedDynamicState3FeaturesEXT<'a>
    {
    }

    impl Default for PhysicalDeviceExtendedDynamicState3FeaturesEXT<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null_mut(),
                extended_dynamic_state3_tessellation_domain_origin: Default::default(),
                extended_dynamic_state3_depth_clamp_enable: Default::default(),
                extended_dynamic_state3_polygon_mode: Default::default(),
                extended_dynamic_state3_rasterization_samples: Default::default(),
                extended_dynamic_state3_sample_mask: Default::default(),
                extended_dynamic_state3_alpha_to_coverage_enable: Default::default(),
                extended_dynamic_state3_alpha_to_one_enable: Default::default(),
                extended_dynamic_state3_logic_op_enable: Default::default(),
                extended_dynamic_state3_color_blend_enable: Default::default(),
                extended_dynamic_state3_color_blend_equation: Default::default(),
                extended_dynamic_state3_color_write_mask: Default::default(),
                extended_dynamic_state3_rasterization_stream: Default::default(),
                extended_dynamic_state3_conservative_rasterization_mode: Default::default(),
                extended_dynamic_state3_extra_primitive_overestimation_size: Default::default(),
                extended_dynamic_state3_depth_clip_enable: Default::default(),
                extended_dynamic_state3_sample_locations_enable: Default::default(),
                extended_dynamic_state3_color_blend_advanced: Default::default(),
                extended_dynamic_state3_provoking_vertex_mode: Default::default(),
                extended_dynamic_state3_line_rasterization_mode: Default::default(),
                extended_dynamic_state3_line_stipple_enable: Default::default(),
                extended_dynamic_state3_depth_clip_negative_one_to_one: Default::default(),
                extended_dynamic_state3_viewport_w_scaling_enable: Default::default(),
                extended_dynamic_state3_viewport_swizzle: Default::default(),
                extended_dynamic_state3_coverage_to_color_enable: Default::default(),
                extended_dynamic_state3_coverage_to_color_location: Default::default(),
                extended_dynamic_state3_coverage_modulation_mode: Default::default(),
                extended_dynamic_state3_coverage_modulation_table_enable: Default::default(),
                extended_dynamic_state3_coverage_modulation_table: Default::default(),
                extended_dynamic_state3_coverage_reduction_mode: Default::default(),
                extended_dynamic_state3_representative_fragment_test_enable: Default::default(),
                extended_dynamic_state3_shading_rate_image_enable: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> PhysicalDeviceExtendedDynamicState3FeaturesEXT<'a> {
        #[inline]
        pub fn extended_dynamic_state3_tessellation_domain_origin(
            mut self,
            extended_dynamic_state3_tessellation_domain_origin: bool,
        ) -> Self {
            self.extended_dynamic_state3_tessellation_domain_origin =
                extended_dynamic_state3_tessellation_domain_origin.into();
            self
        }

        #[inline]
        pub fn extended_dynamic_state3_depth_clamp_enable(
            mut self,
            extended_dynamic_state3_depth_clamp_enable: bool,
        ) -> Self {
            self.extended_dynamic_state3_depth_clamp_enable =
                extended_dynamic_state3_depth_clamp_enable.into();
            self
        }

        #[inline]
        pub fn extended_dynamic_state3_polygon_mode(
            mut self,
            extended_dynamic_state3_polygon_mode: bool,
        ) -> Self {
            self.extended_dynamic_state3_polygon_mode = extended_dynamic_state3_polygon_mode.into();
            self
        }

        #[inline]
        pub fn extended_dynamic_state3_rasterization_samples(
            mut self,
            extended_dynamic_state3_rasterization_samples: bool,
        ) -> Self {
            self.extended_dynamic_state3_rasterization_samples =
                extended_dynamic_state3_rasterization_samples.into();
            self
        }

        #[inline]
        pub fn extended_dynamic_state3_sample_mask(
            mut self,
            extended_dynamic_state3_sample_mask: bool,
        ) -> Self {
            self.extended_dynamic_state3_sample_mask = extended_dynamic_state3_sample_mask.into();
            self
        }

        #[inline]
        pub fn extended_dynamic_state3_alpha_to_coverage_enable(
            mut self,
            extended_dynamic_state3_alpha_to_coverage_enable: bool,
        ) -> Self {
            self.extended_dynamic_state3_alpha_to_coverage_enable =
                extended_dynamic_state3_alpha_to_coverage_enable.into();
            self
        }

        #[inline]
        pub fn extended_dynamic_state3_alpha_to_one_enable(
            mut self,
            extended_dynamic_state3_alpha_to_one_enable: bool,
        ) -> Self {
            self.extended_dynamic_state3_alpha_to_one_enable =
                extended_dynamic_state3_alpha_to_one_enable.into();
            self
        }

        #[inline]
        pub fn extended_dynamic_state3_logic_op_enable(
            mut self,
            extended_dynamic_state3_logic_op_enable: bool,
        ) -> Self {
            self.extended_dynamic_state3_logic_op_enable =
                extended_dynamic_state3_logic_op_enable.into();
            self
        }

        #[inline]
        pub fn extended_dynamic_state3_color_blend_enable(
            mut self,
            extended_dynamic_state3_color_blend_enable: bool,
        ) -> Self {
            self.extended_dynamic_state3_color_blend_enable =
                extended_dynamic_state3_color_blend_enable.into();
            self
        }

        #[inline]
        pub fn extended_dynamic_state3_color_blend_equation(
            mut self,
            extended_dynamic_state3_color_blend_equation: bool,
        ) -> Self {
            self.extended_dynamic_state3_color_blend_equation =
                extended_dynamic_state3_color_blend_equation.into();
            self
        }

        #[inline]
        pub fn extended_dynamic_state3_color_write_mask(
            mut self,
            extended_dynamic_state3_color_write_mask: bool,
        ) -> Self {
            self.extended_dynamic_state3_color_write_mask =
                extended_dynamic_state3_color_write_mask.into();
            self
        }

        #[inline]
        pub fn extended_dynamic_state3_rasterization_stream(
            mut self,
            extended_dynamic_state3_rasterization_stream: bool,
        ) -> Self {
            self.extended_dynamic_state3_rasterization_stream =
                extended_dynamic_state3_rasterization_stream.into();
            self
        }

        #[inline]
        pub fn extended_dynamic_state3_conservative_rasterization_mode(
            mut self,
            extended_dynamic_state3_conservative_rasterization_mode: bool,
        ) -> Self {
            self.extended_dynamic_state3_conservative_rasterization_mode =
                extended_dynamic_state3_conservative_rasterization_mode.into();
            self
        }

        #[inline]
        pub fn extended_dynamic_state3_extra_primitive_overestimation_size(
            mut self,
            extended_dynamic_state3_extra_primitive_overestimation_size: bool,
        ) -> Self {
            self.extended_dynamic_state3_extra_primitive_overestimation_size =
                extended_dynamic_state3_extra_primitive_overestimation_size.into();
            self
        }

        #[inline]
        pub fn extended_dynamic_state3_depth_clip_enable(
            mut self,
            extended_dynamic_state3_depth_clip_enable: bool,
        ) -> Self {
            self.extended_dynamic_state3_depth_clip_enable =
                extended_dynamic_state3_depth_clip_enable.into();
            self
        }

        #[inline]
        pub fn extended_dynamic_state3_sample_locations_enable(
            mut self,
            extended_dynamic_state3_sample_locations_enable: bool,
        ) -> Self {
            self.extended_dynamic_state3_sample_locations_enable =
                extended_dynamic_state3_sample_locations_enable.into();
            self
        }

        #[inline]
        pub fn extended_dynamic_state3_color_blend_advanced(
            mut self,
            extended_dynamic_state3_color_blend_advanced: bool,
        ) -> Self {
            self.extended_dynamic_state3_color_blend_advanced =
                extended_dynamic_state3_color_blend_advanced.into();
            self
        }

        #[inline]
        pub fn extended_dynamic_state3_provoking_vertex_mode(
            mut self,
            extended_dynamic_state3_provoking_vertex_mode: bool,
        ) -> Self {
            self.extended_dynamic_state3_provoking_vertex_mode =
                extended_dynamic_state3_provoking_vertex_mode.into();
            self
        }

        #[inline]
        pub fn extended_dynamic_state3_line_rasterization_mode(
            mut self,
            extended_dynamic_state3_line_rasterization_mode: bool,
        ) -> Self {
            self.extended_dynamic_state3_line_rasterization_mode =
                extended_dynamic_state3_line_rasterization_mode.into();
            self
        }

        #[inline]
        pub fn extended_dynamic_state3_line_stipple_enable(
            mut self,
            extended_dynamic_state3_line_stipple_enable: bool,
        ) -> Self {
            self.extended_dynamic_state3_line_stipple_enable =
                extended_dynamic_state3_line_stipple_enable.into();
            self
        }

        #[inline]
        pub fn extended_dynamic_state3_depth_clip_negative_one_to_one(
            mut self,
            extended_dynamic_state3_depth_clip_negative_one_to_one: bool,
        ) -> Self {
            self.extended_dynamic_state3_depth_clip_negative_one_to_one =
                extended_dynamic_state3_depth_clip_negative_one_to_one.into();
            self
        }

        #[inline]
        pub fn extended_dynamic_state3_viewport_w_scaling_enable(
            mut self,
            extended_dynamic_state3_viewport_w_scaling_enable: bool,
        ) -> Self {
            self.extended_dynamic_state3_viewport_w_scaling_enable =
                extended_dynamic_state3_viewport_w_scaling_enable.into();
            self
        }

        #[inline]
        pub fn extended_dynamic_state3_viewport_swizzle(
            mut self,
            extended_dynamic_state3_viewport_swizzle: bool,
        ) -> Self {
            self.extended_dynamic_state3_viewport_swizzle =
                extended_dynamic_state3_viewport_swizzle.into();
            self
        }

        #[inline]
        pub fn extended_dynamic_state3_coverage_to_color_enable(
            mut self,
            extended_dynamic_state3_coverage_to_color_enable: bool,
        ) -> Self {
            self.extended_dynamic_state3_coverage_to_color_enable =
                extended_dynamic_state3_coverage_to_color_enable.into();
            self
        }

        #[inline]
        pub fn extended_dynamic_state3_coverage_to_color_location(
            mut self,
            extended_dynamic_state3_coverage_to_color_location: bool,
        ) -> Self {
            self.extended_dynamic_state3_coverage_to_color_location =
                extended_dynamic_state3_coverage_to_color_location.into();
            self
        }

        #[inline]
        pub fn extended_dynamic_state3_coverage_modulation_mode(
            mut self,
            extended_dynamic_state3_coverage_modulation_mode: bool,
        ) -> Self {
            self.extended_dynamic_state3_coverage_modulation_mode =
                extended_dynamic_state3_coverage_modulation_mode.into();
            self
        }

        #[inline]
        pub fn extended_dynamic_state3_coverage_modulation_table_enable(
            mut self,
            extended_dynamic_state3_coverage_modulation_table_enable: bool,
        ) -> Self {
            self.extended_dynamic_state3_coverage_modulation_table_enable =
                extended_dynamic_state3_coverage_modulation_table_enable.into();
            self
        }

        #[inline]
        pub fn extended_dynamic_state3_coverage_modulation_table(
            mut self,
            extended_dynamic_state3_coverage_modulation_table: bool,
        ) -> Self {
            self.extended_dynamic_state3_coverage_modulation_table =
                extended_dynamic_state3_coverage_modulation_table.into();
            self
        }

        #[inline]
        pub fn extended_dynamic_state3_coverage_reduction_mode(
            mut self,
            extended_dynamic_state3_coverage_reduction_mode: bool,
        ) -> Self {
            self.extended_dynamic_state3_coverage_reduction_mode =
                extended_dynamic_state3_coverage_reduction_mode.into();
            self
        }

        #[inline]
        pub fn extended_dynamic_state3_representative_fragment_test_enable(
            mut self,
            extended_dynamic_state3_representative_fragment_test_enable: bool,
        ) -> Self {
            self.extended_dynamic_state3_representative_fragment_test_enable =
                extended_dynamic_state3_representative_fragment_test_enable.into();
            self
        }

        #[inline]
        pub fn extended_dynamic_state3_shading_rate_image_enable(
            mut self,
            extended_dynamic_state3_shading_rate_image_enable: bool,
        ) -> Self {
            self.extended_dynamic_state3_shading_rate_image_enable =
                extended_dynamic_state3_shading_rate_image_enable.into();
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPhysicalDeviceExtendedDynamicState3PropertiesEXT.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct PhysicalDeviceExtendedDynamicState3PropertiesEXT<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub dynamic_primitive_topology_unrestricted: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for PhysicalDeviceExtendedDynamicState3PropertiesEXT<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("PhysicalDeviceExtendedDynamicState3PropertiesEXT")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field(
                    "dynamic_primitive_topology_unrestricted",
                    &self.dynamic_primitive_topology_unrestricted,
                )
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for PhysicalDeviceExtendedDynamicState3PropertiesEXT<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PHYSICAL_DEVICE_EXTENDED_DYNAMIC_STATE_3_PROPERTIES_EXT;
    }

    unsafe impl<'a> Extends<PhysicalDeviceProperties2<'a>>
        for PhysicalDeviceExtendedDynamicState3PropertiesEXT<'a>
    {
    }

    impl Default for PhysicalDeviceExtendedDynamicState3PropertiesEXT<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null_mut(),
                dynamic_primitive_topology_unrestricted: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> PhysicalDeviceExtendedDynamicState3PropertiesEXT<'a> {
        #[inline]
        pub fn dynamic_primitive_topology_unrestricted(
            mut self,
            dynamic_primitive_topology_unrestricted: bool,
        ) -> Self {
            self.dynamic_primitive_topology_unrestricted =
                dynamic_primitive_topology_unrestricted.into();
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkColorBlendEquationEXT.html>
    #[repr(C)]
    #[cfg_attr(feature = "debug", derive(Debug))]
    #[derive(Copy, Clone, Default)]
    #[must_use]
    pub struct ColorBlendEquationEXT {
        pub src_color_blend_factor: BlendFactor,
        pub dst_color_blend_factor: BlendFactor,
        pub color_blend_op: BlendOp,
        pub src_alpha_blend_factor: BlendFactor,
        pub dst_alpha_blend_factor: BlendFactor,
        pub alpha_blend_op: BlendOp,
    }

    impl ColorBlendEquationEXT {
        #[inline]
        pub fn src_color_blend_factor(mut self, src_color_blend_factor: BlendFactor) -> Self {
            self.src_color_blend_factor = src_color_blend_factor;
            self
        }

        #[inline]
        pub fn dst_color_blend_factor(mut self, dst_color_blend_factor: BlendFactor) -> Self {
            self.dst_color_blend_factor = dst_color_blend_factor;
            self
        }

        #[inline]
        pub fn color_blend_op(mut self, color_blend_op: BlendOp) -> Self {
            self.color_blend_op = color_blend_op;
            self
        }

        #[inline]
        pub fn src_alpha_blend_factor(mut self, src_alpha_blend_factor: BlendFactor) -> Self {
            self.src_alpha_blend_factor = src_alpha_blend_factor;
            self
        }

        #[inline]
        pub fn dst_alpha_blend_factor(mut self, dst_alpha_blend_factor: BlendFactor) -> Self {
            self.dst_alpha_blend_factor = dst_alpha_blend_factor;
            self
        }

        #[inline]
        pub fn alpha_blend_op(mut self, alpha_blend_op: BlendOp) -> Self {
            self.alpha_blend_op = alpha_blend_op;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkColorBlendAdvancedEXT.html>
    #[repr(C)]
    #[cfg_attr(feature = "debug", derive(Debug))]
    #[derive(Copy, Clone, Default)]
    #[must_use]
    pub struct ColorBlendAdvancedEXT {
        pub advanced_blend_op: BlendOp,
        pub src_premultiplied: Bool32,
        pub dst_premultiplied: Bool32,
        pub blend_overlap: BlendOverlapEXT,
        pub clamp_results: Bool32,
    }

    impl ColorBlendAdvancedEXT {
        #[inline]
        pub fn advanced_blend_op(mut self, advanced_blend_op: BlendOp) -> Self {
            self.advanced_blend_op = advanced_blend_op;
            self
        }

        #[inline]
        pub fn src_premultiplied(mut self, src_premultiplied: bool) -> Self {
            self.src_premultiplied = src_premultiplied.into();
            self
        }

        #[inline]
        pub fn dst_premultiplied(mut self, dst_premultiplied: bool) -> Self {
            self.dst_premultiplied = dst_premultiplied.into();
            self
        }

        #[inline]
        pub fn blend_overlap(mut self, blend_overlap: BlendOverlapEXT) -> Self {
            self.blend_overlap = blend_overlap;
            self
        }

        #[inline]
        pub fn clamp_results(mut self, clamp_results: bool) -> Self {
            self.clamp_results = clamp_results.into();
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdSetTessellationDomainOriginEXT.html>
    pub type PFN_vkCmdSetTessellationDomainOriginEXT = unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        domain_origin: TessellationDomainOrigin,
    );
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdSetDepthClampEnableEXT.html>
    pub type PFN_vkCmdSetDepthClampEnableEXT =
        unsafe extern "system" fn(command_buffer: CommandBuffer, depth_clamp_enable: Bool32);
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdSetPolygonModeEXT.html>
    pub type PFN_vkCmdSetPolygonModeEXT =
        unsafe extern "system" fn(command_buffer: CommandBuffer, polygon_mode: PolygonMode);
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdSetRasterizationSamplesEXT.html>
    pub type PFN_vkCmdSetRasterizationSamplesEXT = unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        rasterization_samples: SampleCountFlagBits,
    );
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdSetSampleMaskEXT.html>
    pub type PFN_vkCmdSetSampleMaskEXT = unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        samples: SampleCountFlagBits,
        p_sample_mask: *const SampleMask,
    );
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdSetAlphaToCoverageEnableEXT.html>
    pub type PFN_vkCmdSetAlphaToCoverageEnableEXT =
        unsafe extern "system" fn(command_buffer: CommandBuffer, alpha_to_coverage_enable: Bool32);
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdSetAlphaToOneEnableEXT.html>
    pub type PFN_vkCmdSetAlphaToOneEnableEXT =
        unsafe extern "system" fn(command_buffer: CommandBuffer, alpha_to_one_enable: Bool32);
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdSetLogicOpEnableEXT.html>
    pub type PFN_vkCmdSetLogicOpEnableEXT =
        unsafe extern "system" fn(command_buffer: CommandBuffer, logic_op_enable: Bool32);
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdSetColorBlendEnableEXT.html>
    pub type PFN_vkCmdSetColorBlendEnableEXT = unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        first_attachment: u32,
        attachment_count: u32,
        p_color_blend_enables: *const Bool32,
    );
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdSetColorBlendEquationEXT.html>
    pub type PFN_vkCmdSetColorBlendEquationEXT = unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        first_attachment: u32,
        attachment_count: u32,
        p_color_blend_equations: *const ColorBlendEquationEXT,
    );
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdSetColorWriteMaskEXT.html>
    pub type PFN_vkCmdSetColorWriteMaskEXT = unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        first_attachment: u32,
        attachment_count: u32,
        p_color_write_masks: *const ColorComponentFlags,
    );
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdSetRasterizationStreamEXT.html>
    pub type PFN_vkCmdSetRasterizationStreamEXT =
        unsafe extern "system" fn(command_buffer: CommandBuffer, rasterization_stream: u32);
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdSetConservativeRasterizationModeEXT.html>
    pub type PFN_vkCmdSetConservativeRasterizationModeEXT = unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        conservative_rasterization_mode: ConservativeRasterizationModeEXT,
    );
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdSetExtraPrimitiveOverestimationSizeEXT.html>
    pub type PFN_vkCmdSetExtraPrimitiveOverestimationSizeEXT = unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        extra_primitive_overestimation_size: f32,
    );
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdSetDepthClipEnableEXT.html>
    pub type PFN_vkCmdSetDepthClipEnableEXT =
        unsafe extern "system" fn(command_buffer: CommandBuffer, depth_clip_enable: Bool32);
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdSetSampleLocationsEnableEXT.html>
    pub type PFN_vkCmdSetSampleLocationsEnableEXT =
        unsafe extern "system" fn(command_buffer: CommandBuffer, sample_locations_enable: Bool32);
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdSetColorBlendAdvancedEXT.html>
    pub type PFN_vkCmdSetColorBlendAdvancedEXT = unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        first_attachment: u32,
        attachment_count: u32,
        p_color_blend_advanced: *const ColorBlendAdvancedEXT,
    );
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdSetProvokingVertexModeEXT.html>
    pub type PFN_vkCmdSetProvokingVertexModeEXT = unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        provoking_vertex_mode: ProvokingVertexModeEXT,
    );
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdSetLineRasterizationModeEXT.html>
    pub type PFN_vkCmdSetLineRasterizationModeEXT = unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        line_rasterization_mode: LineRasterizationModeEXT,
    );
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdSetLineStippleEnableEXT.html>
    pub type PFN_vkCmdSetLineStippleEnableEXT =
        unsafe extern "system" fn(command_buffer: CommandBuffer, stippled_line_enable: Bool32);
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdSetDepthClipNegativeOneToOneEXT.html>
    pub type PFN_vkCmdSetDepthClipNegativeOneToOneEXT =
        unsafe extern "system" fn(command_buffer: CommandBuffer, negative_one_to_one: Bool32);
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdSetViewportWScalingEnableNV.html>
    pub type PFN_vkCmdSetViewportWScalingEnableNV =
        unsafe extern "system" fn(command_buffer: CommandBuffer, viewport_w_scaling_enable: Bool32);
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdSetViewportSwizzleNV.html>
    pub type PFN_vkCmdSetViewportSwizzleNV = unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        first_viewport: u32,
        viewport_count: u32,
        p_viewport_swizzles: *const ViewportSwizzleNV,
    );
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdSetCoverageToColorEnableNV.html>
    pub type PFN_vkCmdSetCoverageToColorEnableNV =
        unsafe extern "system" fn(command_buffer: CommandBuffer, coverage_to_color_enable: Bool32);
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdSetCoverageToColorLocationNV.html>
    pub type PFN_vkCmdSetCoverageToColorLocationNV =
        unsafe extern "system" fn(command_buffer: CommandBuffer, coverage_to_color_location: u32);
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdSetCoverageModulationModeNV.html>
    pub type PFN_vkCmdSetCoverageModulationModeNV = unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        coverage_modulation_mode: CoverageModulationModeNV,
    );
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdSetCoverageModulationTableEnableNV.html>
    pub type PFN_vkCmdSetCoverageModulationTableEnableNV = unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        coverage_modulation_table_enable: Bool32,
    );
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdSetCoverageModulationTableNV.html>
    pub type PFN_vkCmdSetCoverageModulationTableNV = unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        coverage_modulation_table_count: u32,
        p_coverage_modulation_table: *const f32,
    );
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdSetShadingRateImageEnableNV.html>
    pub type PFN_vkCmdSetShadingRateImageEnableNV =
        unsafe extern "system" fn(command_buffer: CommandBuffer, shading_rate_image_enable: Bool32);
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdSetCoverageReductionModeNV.html>
    pub type PFN_vkCmdSetCoverageReductionModeNV = unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        coverage_reduction_mode: CoverageReductionModeNV,
    );
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdSetRepresentativeFragmentTestEnableNV.html>
    pub type PFN_vkCmdSetRepresentativeFragmentTestEnableNV = unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        representative_fragment_test_enable: Bool32,
    );
}

pub struct DeviceFn {
    cmd_set_depth_clamp_enable_ext: PFN_vkCmdSetDepthClampEnableEXT,
    cmd_set_polygon_mode_ext: PFN_vkCmdSetPolygonModeEXT,
    cmd_set_rasterization_samples_ext: PFN_vkCmdSetRasterizationSamplesEXT,
    cmd_set_sample_mask_ext: PFN_vkCmdSetSampleMaskEXT,
    cmd_set_alpha_to_coverage_enable_ext: PFN_vkCmdSetAlphaToCoverageEnableEXT,
    cmd_set_alpha_to_one_enable_ext: PFN_vkCmdSetAlphaToOneEnableEXT,
    cmd_set_logic_op_enable_ext: PFN_vkCmdSetLogicOpEnableEXT,
    cmd_set_color_blend_enable_ext: PFN_vkCmdSetColorBlendEnableEXT,
    cmd_set_color_blend_equation_ext: PFN_vkCmdSetColorBlendEquationEXT,
    cmd_set_color_write_mask_ext: PFN_vkCmdSetColorWriteMaskEXT,
    cmd_set_tessellation_domain_origin_ext: Option<PFN_vkCmdSetTessellationDomainOriginEXT>,
    cmd_set_rasterization_stream_ext: Option<PFN_vkCmdSetRasterizationStreamEXT>,
    cmd_set_conservative_rasterization_mode_ext:
        Option<PFN_vkCmdSetConservativeRasterizationModeEXT>,
    cmd_set_extra_primitive_overestimation_size_ext:
        Option<PFN_vkCmdSetExtraPrimitiveOverestimationSizeEXT>,
    cmd_set_depth_clip_enable_ext: Option<PFN_vkCmdSetDepthClipEnableEXT>,
    cmd_set_sample_locations_enable_ext: Option<PFN_vkCmdSetSampleLocationsEnableEXT>,
    cmd_set_color_blend_advanced_ext: Option<PFN_vkCmdSetColorBlendAdvancedEXT>,
    cmd_set_provoking_vertex_mode_ext: Option<PFN_vkCmdSetProvokingVertexModeEXT>,
    cmd_set_line_rasterization_mode_ext: Option<PFN_vkCmdSetLineRasterizationModeEXT>,
    cmd_set_line_stipple_enable_ext: Option<PFN_vkCmdSetLineStippleEnableEXT>,
    cmd_set_depth_clip_negative_one_to_one_ext: Option<PFN_vkCmdSetDepthClipNegativeOneToOneEXT>,
    cmd_set_viewport_w_scaling_enable_nv: Option<PFN_vkCmdSetViewportWScalingEnableNV>,
    cmd_set_viewport_swizzle_nv: Option<PFN_vkCmdSetViewportSwizzleNV>,
    cmd_set_coverage_to_color_enable_nv: Option<PFN_vkCmdSetCoverageToColorEnableNV>,
    cmd_set_coverage_to_color_location_nv: Option<PFN_vkCmdSetCoverageToColorLocationNV>,
    cmd_set_coverage_modulation_mode_nv: Option<PFN_vkCmdSetCoverageModulationModeNV>,
    cmd_set_coverage_modulation_table_enable_nv:
        Option<PFN_vkCmdSetCoverageModulationTableEnableNV>,
    cmd_set_coverage_modulation_table_nv: Option<PFN_vkCmdSetCoverageModulationTableNV>,
    cmd_set_shading_rate_image_enable_nv: Option<PFN_vkCmdSetShadingRateImageEnableNV>,
    cmd_set_representative_fragment_test_enable_nv:
        Option<PFN_vkCmdSetRepresentativeFragmentTestEnableNV>,
    cmd_set_coverage_reduction_mode_nv: Option<PFN_vkCmdSetCoverageReductionModeNV>,
}

impl DeviceFn {
    pub unsafe fn load(
        load: impl Fn(&CStr) -> Option<PFN_vkVoidFunction>,
    ) -> core::result::Result<Self, MissingEntryPointError> {
        unsafe {
            Ok(Self {
                cmd_set_depth_clamp_enable_ext: transmute(
                    load(c"vkCmdSetDepthClampEnableEXT").ok_or(MissingEntryPointError)?,
                ),
                cmd_set_polygon_mode_ext: transmute(
                    load(c"vkCmdSetPolygonModeEXT").ok_or(MissingEntryPointError)?,
                ),
                cmd_set_rasterization_samples_ext: transmute(
                    load(c"vkCmdSetRasterizationSamplesEXT").ok_or(MissingEntryPointError)?,
                ),
                cmd_set_sample_mask_ext: transmute(
                    load(c"vkCmdSetSampleMaskEXT").ok_or(MissingEntryPointError)?,
                ),
                cmd_set_alpha_to_coverage_enable_ext: transmute(
                    load(c"vkCmdSetAlphaToCoverageEnableEXT").ok_or(MissingEntryPointError)?,
                ),
                cmd_set_alpha_to_one_enable_ext: transmute(
                    load(c"vkCmdSetAlphaToOneEnableEXT").ok_or(MissingEntryPointError)?,
                ),
                cmd_set_logic_op_enable_ext: transmute(
                    load(c"vkCmdSetLogicOpEnableEXT").ok_or(MissingEntryPointError)?,
                ),
                cmd_set_color_blend_enable_ext: transmute(
                    load(c"vkCmdSetColorBlendEnableEXT").ok_or(MissingEntryPointError)?,
                ),
                cmd_set_color_blend_equation_ext: transmute(
                    load(c"vkCmdSetColorBlendEquationEXT").ok_or(MissingEntryPointError)?,
                ),
                cmd_set_color_write_mask_ext: transmute(
                    load(c"vkCmdSetColorWriteMaskEXT").ok_or(MissingEntryPointError)?,
                ),
                cmd_set_tessellation_domain_origin_ext: transmute(load(
                    c"vkCmdSetTessellationDomainOriginEXT",
                )),
                cmd_set_rasterization_stream_ext: transmute(load(
                    c"vkCmdSetRasterizationStreamEXT",
                )),
                cmd_set_conservative_rasterization_mode_ext: transmute(load(
                    c"vkCmdSetConservativeRasterizationModeEXT",
                )),
                cmd_set_extra_primitive_overestimation_size_ext: transmute(load(
                    c"vkCmdSetExtraPrimitiveOverestimationSizeEXT",
                )),
                cmd_set_depth_clip_enable_ext: transmute(load(c"vkCmdSetDepthClipEnableEXT")),
                cmd_set_sample_locations_enable_ext: transmute(load(
                    c"vkCmdSetSampleLocationsEnableEXT",
                )),
                cmd_set_color_blend_advanced_ext: transmute(load(c"vkCmdSetColorBlendAdvancedEXT")),
                cmd_set_provoking_vertex_mode_ext: transmute(load(
                    c"vkCmdSetProvokingVertexModeEXT",
                )),
                cmd_set_line_rasterization_mode_ext: transmute(load(
                    c"vkCmdSetLineRasterizationModeEXT",
                )),
                cmd_set_line_stipple_enable_ext: transmute(load(c"vkCmdSetLineStippleEnableEXT")),
                cmd_set_depth_clip_negative_one_to_one_ext: transmute(load(
                    c"vkCmdSetDepthClipNegativeOneToOneEXT",
                )),
                cmd_set_viewport_w_scaling_enable_nv: transmute(load(
                    c"vkCmdSetViewportWScalingEnableNV",
                )),
                cmd_set_viewport_swizzle_nv: transmute(load(c"vkCmdSetViewportSwizzleNV")),
                cmd_set_coverage_to_color_enable_nv: transmute(load(
                    c"vkCmdSetCoverageToColorEnableNV",
                )),
                cmd_set_coverage_to_color_location_nv: transmute(load(
                    c"vkCmdSetCoverageToColorLocationNV",
                )),
                cmd_set_coverage_modulation_mode_nv: transmute(load(
                    c"vkCmdSetCoverageModulationModeNV",
                )),
                cmd_set_coverage_modulation_table_enable_nv: transmute(load(
                    c"vkCmdSetCoverageModulationTableEnableNV",
                )),
                cmd_set_coverage_modulation_table_nv: transmute(load(
                    c"vkCmdSetCoverageModulationTableNV",
                )),
                cmd_set_shading_rate_image_enable_nv: transmute(load(
                    c"vkCmdSetShadingRateImageEnableNV",
                )),
                cmd_set_representative_fragment_test_enable_nv: transmute(load(
                    c"vkCmdSetRepresentativeFragmentTestEnableNV",
                )),
                cmd_set_coverage_reduction_mode_nv: transmute(load(
                    c"vkCmdSetCoverageReductionModeNV",
                )),
            })
        }
    }
}

impl DeviceFn {
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdSetDepthClampEnableEXT.html>
    #[inline]
    pub unsafe fn cmd_set_depth_clamp_enable_ext(
        &self,
        command_buffer: CommandBuffer,
        depth_clamp_enable: bool,
    ) {
        unsafe { (self.cmd_set_depth_clamp_enable_ext)(command_buffer, depth_clamp_enable.into()) }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdSetPolygonModeEXT.html>
    #[inline]
    pub unsafe fn cmd_set_polygon_mode_ext(
        &self,
        command_buffer: CommandBuffer,
        polygon_mode: PolygonMode,
    ) {
        unsafe { (self.cmd_set_polygon_mode_ext)(command_buffer, polygon_mode) }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdSetRasterizationSamplesEXT.html>
    #[inline]
    pub unsafe fn cmd_set_rasterization_samples_ext(
        &self,
        command_buffer: CommandBuffer,
        rasterization_samples: SampleCountFlagBits,
    ) {
        unsafe { (self.cmd_set_rasterization_samples_ext)(command_buffer, rasterization_samples) }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdSetSampleMaskEXT.html>
    #[inline]
    pub unsafe fn cmd_set_sample_mask_ext(
        &self,
        command_buffer: CommandBuffer,
        samples: SampleCountFlagBits,
        sample_mask: Option<&[SampleMask]>,
    ) {
        unsafe { (self.cmd_set_sample_mask_ext)(command_buffer, samples, sample_mask.to_raw_ptr()) }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdSetAlphaToCoverageEnableEXT.html>
    #[inline]
    pub unsafe fn cmd_set_alpha_to_coverage_enable_ext(
        &self,
        command_buffer: CommandBuffer,
        alpha_to_coverage_enable: bool,
    ) {
        unsafe {
            (self.cmd_set_alpha_to_coverage_enable_ext)(
                command_buffer,
                alpha_to_coverage_enable.into(),
            )
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdSetAlphaToOneEnableEXT.html>
    #[inline]
    pub unsafe fn cmd_set_alpha_to_one_enable_ext(
        &self,
        command_buffer: CommandBuffer,
        alpha_to_one_enable: bool,
    ) {
        unsafe {
            (self.cmd_set_alpha_to_one_enable_ext)(command_buffer, alpha_to_one_enable.into())
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdSetLogicOpEnableEXT.html>
    #[inline]
    pub unsafe fn cmd_set_logic_op_enable_ext(
        &self,
        command_buffer: CommandBuffer,
        logic_op_enable: bool,
    ) {
        unsafe { (self.cmd_set_logic_op_enable_ext)(command_buffer, logic_op_enable.into()) }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdSetColorBlendEnableEXT.html>
    #[inline]
    pub unsafe fn cmd_set_color_blend_enable_ext(
        &self,
        command_buffer: CommandBuffer,
        first_attachment: u32,
        color_blend_enables: &[Bool32],
    ) {
        unsafe {
            (self.cmd_set_color_blend_enable_ext)(
                command_buffer,
                first_attachment,
                color_blend_enables.len().try_into().unwrap(),
                color_blend_enables.as_ptr() as _,
            )
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdSetColorBlendEquationEXT.html>
    #[inline]
    pub unsafe fn cmd_set_color_blend_equation_ext(
        &self,
        command_buffer: CommandBuffer,
        first_attachment: u32,
        color_blend_equations: &[ColorBlendEquationEXT],
    ) {
        unsafe {
            (self.cmd_set_color_blend_equation_ext)(
                command_buffer,
                first_attachment,
                color_blend_equations.len().try_into().unwrap(),
                color_blend_equations.as_ptr() as _,
            )
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdSetColorWriteMaskEXT.html>
    #[inline]
    pub unsafe fn cmd_set_color_write_mask_ext(
        &self,
        command_buffer: CommandBuffer,
        first_attachment: u32,
        color_write_masks: &[ColorComponentFlags],
    ) {
        unsafe {
            (self.cmd_set_color_write_mask_ext)(
                command_buffer,
                first_attachment,
                color_write_masks.len().try_into().unwrap(),
                color_write_masks.as_ptr() as _,
            )
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdSetTessellationDomainOriginEXT.html>
    #[inline]
    pub unsafe fn cmd_set_tessellation_domain_origin_ext(
        &self,
        command_buffer: CommandBuffer,
        domain_origin: TessellationDomainOrigin,
    ) {
        unsafe {
            (self.cmd_set_tessellation_domain_origin_ext.unwrap())(command_buffer, domain_origin)
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdSetRasterizationStreamEXT.html>
    #[inline]
    pub unsafe fn cmd_set_rasterization_stream_ext(
        &self,
        command_buffer: CommandBuffer,
        rasterization_stream: u32,
    ) {
        unsafe {
            (self.cmd_set_rasterization_stream_ext.unwrap())(command_buffer, rasterization_stream)
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdSetConservativeRasterizationModeEXT.html>
    #[inline]
    pub unsafe fn cmd_set_conservative_rasterization_mode_ext(
        &self,
        command_buffer: CommandBuffer,
        conservative_rasterization_mode: ConservativeRasterizationModeEXT,
    ) {
        unsafe {
            (self.cmd_set_conservative_rasterization_mode_ext.unwrap())(
                command_buffer,
                conservative_rasterization_mode,
            )
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdSetExtraPrimitiveOverestimationSizeEXT.html>
    #[inline]
    pub unsafe fn cmd_set_extra_primitive_overestimation_size_ext(
        &self,
        command_buffer: CommandBuffer,
        extra_primitive_overestimation_size: f32,
    ) {
        unsafe {
            (self
                .cmd_set_extra_primitive_overestimation_size_ext
                .unwrap())(command_buffer, extra_primitive_overestimation_size)
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdSetDepthClipEnableEXT.html>
    #[inline]
    pub unsafe fn cmd_set_depth_clip_enable_ext(
        &self,
        command_buffer: CommandBuffer,
        depth_clip_enable: bool,
    ) {
        unsafe {
            (self.cmd_set_depth_clip_enable_ext.unwrap())(command_buffer, depth_clip_enable.into())
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdSetSampleLocationsEnableEXT.html>
    #[inline]
    pub unsafe fn cmd_set_sample_locations_enable_ext(
        &self,
        command_buffer: CommandBuffer,
        sample_locations_enable: bool,
    ) {
        unsafe {
            (self.cmd_set_sample_locations_enable_ext.unwrap())(
                command_buffer,
                sample_locations_enable.into(),
            )
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdSetColorBlendAdvancedEXT.html>
    #[inline]
    pub unsafe fn cmd_set_color_blend_advanced_ext(
        &self,
        command_buffer: CommandBuffer,
        first_attachment: u32,
        color_blend_advanced: &[ColorBlendAdvancedEXT],
    ) {
        unsafe {
            (self.cmd_set_color_blend_advanced_ext.unwrap())(
                command_buffer,
                first_attachment,
                color_blend_advanced.len().try_into().unwrap(),
                color_blend_advanced.as_ptr() as _,
            )
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdSetProvokingVertexModeEXT.html>
    #[inline]
    pub unsafe fn cmd_set_provoking_vertex_mode_ext(
        &self,
        command_buffer: CommandBuffer,
        provoking_vertex_mode: ProvokingVertexModeEXT,
    ) {
        unsafe {
            (self.cmd_set_provoking_vertex_mode_ext.unwrap())(command_buffer, provoking_vertex_mode)
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdSetLineRasterizationModeEXT.html>
    #[inline]
    pub unsafe fn cmd_set_line_rasterization_mode_ext(
        &self,
        command_buffer: CommandBuffer,
        line_rasterization_mode: LineRasterizationModeEXT,
    ) {
        unsafe {
            (self.cmd_set_line_rasterization_mode_ext.unwrap())(
                command_buffer,
                line_rasterization_mode,
            )
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdSetLineStippleEnableEXT.html>
    #[inline]
    pub unsafe fn cmd_set_line_stipple_enable_ext(
        &self,
        command_buffer: CommandBuffer,
        stippled_line_enable: bool,
    ) {
        unsafe {
            (self.cmd_set_line_stipple_enable_ext.unwrap())(
                command_buffer,
                stippled_line_enable.into(),
            )
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdSetDepthClipNegativeOneToOneEXT.html>
    #[inline]
    pub unsafe fn cmd_set_depth_clip_negative_one_to_one_ext(
        &self,
        command_buffer: CommandBuffer,
        negative_one_to_one: bool,
    ) {
        unsafe {
            (self.cmd_set_depth_clip_negative_one_to_one_ext.unwrap())(
                command_buffer,
                negative_one_to_one.into(),
            )
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdSetViewportWScalingEnableNV.html>
    #[inline]
    pub unsafe fn cmd_set_viewport_w_scaling_enable_nv(
        &self,
        command_buffer: CommandBuffer,
        viewport_w_scaling_enable: bool,
    ) {
        unsafe {
            (self.cmd_set_viewport_w_scaling_enable_nv.unwrap())(
                command_buffer,
                viewport_w_scaling_enable.into(),
            )
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdSetViewportSwizzleNV.html>
    #[inline]
    pub unsafe fn cmd_set_viewport_swizzle_nv(
        &self,
        command_buffer: CommandBuffer,
        first_viewport: u32,
        viewport_swizzles: &[ViewportSwizzleNV],
    ) {
        unsafe {
            (self.cmd_set_viewport_swizzle_nv.unwrap())(
                command_buffer,
                first_viewport,
                viewport_swizzles.len().try_into().unwrap(),
                viewport_swizzles.as_ptr() as _,
            )
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdSetCoverageToColorEnableNV.html>
    #[inline]
    pub unsafe fn cmd_set_coverage_to_color_enable_nv(
        &self,
        command_buffer: CommandBuffer,
        coverage_to_color_enable: bool,
    ) {
        unsafe {
            (self.cmd_set_coverage_to_color_enable_nv.unwrap())(
                command_buffer,
                coverage_to_color_enable.into(),
            )
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdSetCoverageToColorLocationNV.html>
    #[inline]
    pub unsafe fn cmd_set_coverage_to_color_location_nv(
        &self,
        command_buffer: CommandBuffer,
        coverage_to_color_location: u32,
    ) {
        unsafe {
            (self.cmd_set_coverage_to_color_location_nv.unwrap())(
                command_buffer,
                coverage_to_color_location,
            )
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdSetCoverageModulationModeNV.html>
    #[inline]
    pub unsafe fn cmd_set_coverage_modulation_mode_nv(
        &self,
        command_buffer: CommandBuffer,
        coverage_modulation_mode: CoverageModulationModeNV,
    ) {
        unsafe {
            (self.cmd_set_coverage_modulation_mode_nv.unwrap())(
                command_buffer,
                coverage_modulation_mode,
            )
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdSetCoverageModulationTableEnableNV.html>
    #[inline]
    pub unsafe fn cmd_set_coverage_modulation_table_enable_nv(
        &self,
        command_buffer: CommandBuffer,
        coverage_modulation_table_enable: bool,
    ) {
        unsafe {
            (self.cmd_set_coverage_modulation_table_enable_nv.unwrap())(
                command_buffer,
                coverage_modulation_table_enable.into(),
            )
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdSetCoverageModulationTableNV.html>
    #[inline]
    pub unsafe fn cmd_set_coverage_modulation_table_nv(
        &self,
        command_buffer: CommandBuffer,
        coverage_modulation_table: &[f32],
    ) {
        unsafe {
            (self.cmd_set_coverage_modulation_table_nv.unwrap())(
                command_buffer,
                coverage_modulation_table.len().try_into().unwrap(),
                coverage_modulation_table.as_ptr() as _,
            )
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdSetShadingRateImageEnableNV.html>
    #[inline]
    pub unsafe fn cmd_set_shading_rate_image_enable_nv(
        &self,
        command_buffer: CommandBuffer,
        shading_rate_image_enable: bool,
    ) {
        unsafe {
            (self.cmd_set_shading_rate_image_enable_nv.unwrap())(
                command_buffer,
                shading_rate_image_enable.into(),
            )
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdSetRepresentativeFragmentTestEnableNV.html>
    #[inline]
    pub unsafe fn cmd_set_representative_fragment_test_enable_nv(
        &self,
        command_buffer: CommandBuffer,
        representative_fragment_test_enable: bool,
    ) {
        unsafe {
            (self.cmd_set_representative_fragment_test_enable_nv.unwrap())(
                command_buffer,
                representative_fragment_test_enable.into(),
            )
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdSetCoverageReductionModeNV.html>
    #[inline]
    pub unsafe fn cmd_set_coverage_reduction_mode_nv(
        &self,
        command_buffer: CommandBuffer,
        coverage_reduction_mode: CoverageReductionModeNV,
    ) {
        unsafe {
            (self.cmd_set_coverage_reduction_mode_nv.unwrap())(
                command_buffer,
                coverage_reduction_mode,
            )
        }
    }
}
