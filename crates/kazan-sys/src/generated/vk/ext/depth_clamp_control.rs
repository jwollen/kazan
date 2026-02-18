#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use std::ffi::{c_char, c_int, c_void};
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceDepthClampControlFeaturesEXT {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub depth_clamp_control: Bool32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PipelineViewportDepthClampControlCreateInfoEXT {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub depth_clamp_mode: DepthClampModeEXT,
    pub p_depth_clamp_range: *const DepthClampRangeEXT,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct DepthClampRangeEXT {
    pub min_depth_clamp: f32,
    pub max_depth_clamp: f32,
}
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DepthClampModeEXT(i32);
impl DepthClampModeEXT {
    pub const VIEWPORT_RANGE_EXT: Self = Self(0);
    pub const USER_DEFINED_RANGE_EXT: Self = Self(1);
}
