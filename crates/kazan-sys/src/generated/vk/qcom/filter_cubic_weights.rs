#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use std::ffi::{c_char, c_int, c_void};
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceCubicWeightsFeaturesQCOM {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub selectable_cubic_weights: Bool32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SamplerCubicWeightsCreateInfoQCOM {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub cubic_weights: CubicFilterWeightsQCOM,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct BlitImageCubicWeightsInfoQCOM {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub cubic_weights: CubicFilterWeightsQCOM,
}
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CubicFilterWeightsQCOM(i32);
impl CubicFilterWeightsQCOM {
    pub const CATMULL_ROM_QCOM: Self = Self(0);
    pub const ZERO_TANGENT_CARDINAL_QCOM: Self = Self(1);
    pub const B_SPLINE_QCOM: Self = Self(2);
    pub const MITCHELL_NETRAVALI_QCOM: Self = Self(3);
}
