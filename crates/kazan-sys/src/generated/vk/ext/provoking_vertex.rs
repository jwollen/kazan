#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{c_char, c_int, c_void};
#[repr(C)]
pub struct PhysicalDeviceProvokingVertexFeaturesEXT {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub provoking_vertex_last: Bool32,
    pub transform_feedback_preserves_provoking_vertex: Bool32,
}
#[repr(C)]
pub struct PhysicalDeviceProvokingVertexPropertiesEXT {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub provoking_vertex_mode_per_pipeline: Bool32,
    pub transform_feedback_preserves_triangle_fan_provoking_vertex: Bool32,
}
#[repr(C)]
pub struct PipelineRasterizationProvokingVertexStateCreateInfoEXT {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub provoking_vertex_mode: ProvokingVertexModeEXT,
}
#[repr(transparent)]
#[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ProvokingVertexModeEXT(i32);
impl ProvokingVertexModeEXT {
    pub const FIRST_VERTEX_EXT: Self = Self(0);
    pub const LAST_VERTEX_EXT: Self = Self(1);
}
