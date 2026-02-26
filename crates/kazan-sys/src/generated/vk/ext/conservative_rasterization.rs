#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{c_char, c_int, c_void};
use core::marker::PhantomData;
#[repr(C)]
#[derive(Copy, Clone)]
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
impl Default for PhysicalDeviceConservativeRasterizationPropertiesEXT<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::PHYSICAL_DEVICE_CONSERVATIVE_RASTERIZATION_PROPERTIES_EXT,
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
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PipelineRasterizationConservativeStateCreateInfoEXT<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: PipelineRasterizationConservativeStateCreateFlagsEXT,
    pub conservative_rasterization_mode: ConservativeRasterizationModeEXT,
    pub extra_primitive_overestimation_size: f32,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for PipelineRasterizationConservativeStateCreateInfoEXT<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::PIPELINE_RASTERIZATION_CONSERVATIVE_STATE_CREATE_INFO_EXT,
            p_next: core::ptr::null(),
            flags: Default::default(),
            conservative_rasterization_mode: Default::default(),
            extra_primitive_overestimation_size: Default::default(),
            _marker: PhantomData,
        }
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ConservativeRasterizationModeEXT(i32);
impl ConservativeRasterizationModeEXT {
    pub const DISABLED_EXT: Self = Self(0);
    pub const OVERESTIMATE_EXT: Self = Self(1);
    pub const UNDERESTIMATE_EXT: Self = Self(2);
}
bitflags! {
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct PipelineRasterizationConservativeStateCreateFlagsEXT: Flags {
    }
}
