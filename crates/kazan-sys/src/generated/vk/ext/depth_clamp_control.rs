#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{c_char, c_int, c_void};
use core::marker::PhantomData;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceDepthClampControlFeaturesEXT<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub depth_clamp_control: Bool32,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for PhysicalDeviceDepthClampControlFeaturesEXT<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::PHYSICAL_DEVICE_DEPTH_CLAMP_CONTROL_FEATURES_EXT,
            p_next: core::ptr::null_mut(),
            depth_clamp_control: Default::default(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PipelineViewportDepthClampControlCreateInfoEXT<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub depth_clamp_mode: DepthClampModeEXT,
    pub p_depth_clamp_range: *const DepthClampRangeEXT,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for PipelineViewportDepthClampControlCreateInfoEXT<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::PIPELINE_VIEWPORT_DEPTH_CLAMP_CONTROL_CREATE_INFO_EXT,
            p_next: core::ptr::null(),
            depth_clamp_mode: Default::default(),
            p_depth_clamp_range: core::ptr::null(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Default)]
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
