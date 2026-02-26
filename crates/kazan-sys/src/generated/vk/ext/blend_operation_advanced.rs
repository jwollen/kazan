#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{c_char, c_int, c_void};
use core::marker::PhantomData;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceBlendOperationAdvancedFeaturesEXT<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub advanced_blend_coherent_operations: Bool32,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for PhysicalDeviceBlendOperationAdvancedFeaturesEXT<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::PHYSICAL_DEVICE_BLEND_OPERATION_ADVANCED_FEATURES_EXT,
            p_next: core::ptr::null_mut(),
            advanced_blend_coherent_operations: Default::default(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceBlendOperationAdvancedPropertiesEXT<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub advanced_blend_max_color_attachments: u32,
    pub advanced_blend_independent_blend: Bool32,
    pub advanced_blend_non_premultiplied_src_color: Bool32,
    pub advanced_blend_non_premultiplied_dst_color: Bool32,
    pub advanced_blend_correlated_overlap: Bool32,
    pub advanced_blend_all_operations: Bool32,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for PhysicalDeviceBlendOperationAdvancedPropertiesEXT<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::PHYSICAL_DEVICE_BLEND_OPERATION_ADVANCED_PROPERTIES_EXT,
            p_next: core::ptr::null_mut(),
            advanced_blend_max_color_attachments: Default::default(),
            advanced_blend_independent_blend: Default::default(),
            advanced_blend_non_premultiplied_src_color: Default::default(),
            advanced_blend_non_premultiplied_dst_color: Default::default(),
            advanced_blend_correlated_overlap: Default::default(),
            advanced_blend_all_operations: Default::default(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PipelineColorBlendAdvancedStateCreateInfoEXT<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub src_premultiplied: Bool32,
    pub dst_premultiplied: Bool32,
    pub blend_overlap: BlendOverlapEXT,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for PipelineColorBlendAdvancedStateCreateInfoEXT<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::PIPELINE_COLOR_BLEND_ADVANCED_STATE_CREATE_INFO_EXT,
            p_next: core::ptr::null(),
            src_premultiplied: Default::default(),
            dst_premultiplied: Default::default(),
            blend_overlap: Default::default(),
            _marker: PhantomData,
        }
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct BlendOverlapEXT(i32);
impl BlendOverlapEXT {
    pub const UNCORRELATED_EXT: Self = Self(0);
    pub const DISJOINT_EXT: Self = Self(1);
    pub const CONJOINT_EXT: Self = Self(2);
}
