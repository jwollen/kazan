#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{c_char, c_int, c_void};
use core::marker::PhantomData;
#[repr(C)]
pub struct PhysicalDeviceDepthClampControlFeaturesEXT<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub depth_clamp_control: Bool32,
    pub _marker: PhantomData<&'a ()>,
}
#[repr(C)]
pub struct PipelineViewportDepthClampControlCreateInfoEXT<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub depth_clamp_mode: DepthClampModeEXT,
    pub p_depth_clamp_range: *const DepthClampRangeEXT,
    pub _marker: PhantomData<&'a ()>,
}
#[repr(C)]
pub struct DepthClampRangeEXT {
    pub min_depth_clamp: f32,
    pub max_depth_clamp: f32,
}
#[repr(transparent)]
#[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DepthClampModeEXT(i32);
impl DepthClampModeEXT {
    pub const VIEWPORT_RANGE_EXT: Self = Self(0);
    pub const USER_DEFINED_RANGE_EXT: Self = Self(1);
}
