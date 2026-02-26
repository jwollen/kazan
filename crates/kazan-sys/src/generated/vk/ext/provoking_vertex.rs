#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{c_char, c_int, c_void};
use core::marker::PhantomData;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceProvokingVertexFeaturesEXT<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub provoking_vertex_last: Bool32,
    pub transform_feedback_preserves_provoking_vertex: Bool32,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for PhysicalDeviceProvokingVertexFeaturesEXT<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::PHYSICAL_DEVICE_PROVOKING_VERTEX_FEATURES_EXT,
            p_next: core::ptr::null_mut(),
            provoking_vertex_last: Default::default(),
            transform_feedback_preserves_provoking_vertex: Default::default(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceProvokingVertexPropertiesEXT<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub provoking_vertex_mode_per_pipeline: Bool32,
    pub transform_feedback_preserves_triangle_fan_provoking_vertex: Bool32,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for PhysicalDeviceProvokingVertexPropertiesEXT<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::PHYSICAL_DEVICE_PROVOKING_VERTEX_PROPERTIES_EXT,
            p_next: core::ptr::null_mut(),
            provoking_vertex_mode_per_pipeline: Default::default(),
            transform_feedback_preserves_triangle_fan_provoking_vertex: Default::default(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PipelineRasterizationProvokingVertexStateCreateInfoEXT<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub provoking_vertex_mode: ProvokingVertexModeEXT,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for PipelineRasterizationProvokingVertexStateCreateInfoEXT<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::PIPELINE_RASTERIZATION_PROVOKING_VERTEX_STATE_CREATE_INFO_EXT,
            p_next: core::ptr::null(),
            provoking_vertex_mode: Default::default(),
            _marker: PhantomData,
        }
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ProvokingVertexModeEXT(i32);
impl ProvokingVertexModeEXT {
    pub const FIRST_VERTEX_EXT: Self = Self(0);
    pub const LAST_VERTEX_EXT: Self = Self(1);
}
