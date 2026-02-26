#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{c_char, c_int, c_void};
use core::marker::PhantomData;
#[repr(C)]
pub struct PhysicalDeviceCubicWeightsFeaturesQCOM<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub selectable_cubic_weights: Bool32,
    pub _marker: PhantomData<&'a ()>,
}
#[repr(C)]
pub struct SamplerCubicWeightsCreateInfoQCOM<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub cubic_weights: CubicFilterWeightsQCOM,
    pub _marker: PhantomData<&'a ()>,
}
#[repr(C)]
pub struct BlitImageCubicWeightsInfoQCOM<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub cubic_weights: CubicFilterWeightsQCOM,
    pub _marker: PhantomData<&'a ()>,
}
#[repr(transparent)]
#[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CubicFilterWeightsQCOM(i32);
impl CubicFilterWeightsQCOM {
    pub const CATMULL_ROM_QCOM: Self = Self(0);
    pub const ZERO_TANGENT_CARDINAL_QCOM: Self = Self(1);
    pub const B_SPLINE_QCOM: Self = Self(2);
    pub const MITCHELL_NETRAVALI_QCOM: Self = Self(3);
}
