#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{c_char, c_int, c_void};
use core::marker::PhantomData;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct DepthBiasInfoEXT<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub depth_bias_constant_factor: f32,
    pub depth_bias_clamp: f32,
    pub depth_bias_slope_factor: f32,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for DepthBiasInfoEXT<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::DEPTH_BIAS_INFO_EXT,
            p_next: core::ptr::null(),
            depth_bias_constant_factor: Default::default(),
            depth_bias_clamp: Default::default(),
            depth_bias_slope_factor: Default::default(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct DepthBiasRepresentationInfoEXT<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub depth_bias_representation: DepthBiasRepresentationEXT,
    pub depth_bias_exact: Bool32,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for DepthBiasRepresentationInfoEXT<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::DEPTH_BIAS_REPRESENTATION_INFO_EXT,
            p_next: core::ptr::null(),
            depth_bias_representation: Default::default(),
            depth_bias_exact: Default::default(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceDepthBiasControlFeaturesEXT<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub depth_bias_control: Bool32,
    pub least_representable_value_force_unorm_representation: Bool32,
    pub float_representation: Bool32,
    pub depth_bias_exact: Bool32,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for PhysicalDeviceDepthBiasControlFeaturesEXT<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::PHYSICAL_DEVICE_DEPTH_BIAS_CONTROL_FEATURES_EXT,
            p_next: core::ptr::null_mut(),
            depth_bias_control: Default::default(),
            least_representable_value_force_unorm_representation: Default::default(),
            float_representation: Default::default(),
            depth_bias_exact: Default::default(),
            _marker: PhantomData,
        }
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DepthBiasRepresentationEXT(i32);
impl DepthBiasRepresentationEXT {
    pub const LEAST_REPRESENTABLE_VALUE_FORMAT_EXT: Self = Self(0);
    pub const LEAST_REPRESENTABLE_VALUE_FORCE_UNORM_EXT: Self = Self(1);
    pub const FLOAT_EXT: Self = Self(2);
}
pub type PFN_vkCmdSetDepthBias2EXT = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    p_depth_bias_info: *const DepthBiasInfoEXT<'_>,
);
