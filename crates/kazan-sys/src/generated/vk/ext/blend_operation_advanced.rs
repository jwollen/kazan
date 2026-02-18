#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use std::ffi::{c_char, c_int, c_void};
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceBlendOperationAdvancedFeaturesEXT {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub advanced_blend_coherent_operations: Bool32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceBlendOperationAdvancedPropertiesEXT {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub advanced_blend_max_color_attachments: u32,
    pub advanced_blend_independent_blend: Bool32,
    pub advanced_blend_non_premultiplied_src_color: Bool32,
    pub advanced_blend_non_premultiplied_dst_color: Bool32,
    pub advanced_blend_correlated_overlap: Bool32,
    pub advanced_blend_all_operations: Bool32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PipelineColorBlendAdvancedStateCreateInfoEXT {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub src_premultiplied: Bool32,
    pub dst_premultiplied: Bool32,
    pub blend_overlap: BlendOverlapEXT,
}
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct BlendOverlapEXT(i32);
impl BlendOverlapEXT {
    pub const UNCORRELATED_EXT: Self = Self(0);
    pub const DISJOINT_EXT: Self = Self(1);
    pub const CONJOINT_EXT: Self = Self(2);
}
