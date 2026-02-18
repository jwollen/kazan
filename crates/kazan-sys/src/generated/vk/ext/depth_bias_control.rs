#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use std::ffi::{c_char, c_int, c_void};
#[repr(C)]
#[derive(Copy, Clone)]
pub struct DepthBiasInfoEXT {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub depth_bias_constant_factor: f32,
    pub depth_bias_clamp: f32,
    pub depth_bias_slope_factor: f32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct DepthBiasRepresentationInfoEXT {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub depth_bias_representation: DepthBiasRepresentationEXT,
    pub depth_bias_exact: Bool32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceDepthBiasControlFeaturesEXT {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub depth_bias_control: Bool32,
    pub least_representable_value_force_unorm_representation: Bool32,
    pub float_representation: Bool32,
    pub depth_bias_exact: Bool32,
}
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DepthBiasRepresentationEXT(i32);
impl DepthBiasRepresentationEXT {
    pub const LEAST_REPRESENTABLE_VALUE_FORMAT_EXT: Self = Self(0);
    pub const LEAST_REPRESENTABLE_VALUE_FORCE_UNORM_EXT: Self = Self(1);
    pub const FLOAT_EXT: Self = Self(2);
}
pub type PFN_vkCmdSetDepthBias2EXT = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    p_depth_bias_info: *const DepthBiasInfoEXT,
);
