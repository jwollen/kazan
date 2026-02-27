#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{CStr, c_char, c_int, c_void};
use core::marker::PhantomData;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceCubicWeightsFeaturesQCOM<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub selectable_cubic_weights: Bool32,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for PhysicalDeviceCubicWeightsFeaturesQCOM<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::PHYSICAL_DEVICE_CUBIC_WEIGHTS_FEATURES_QCOM,
            p_next: core::ptr::null_mut(),
            selectable_cubic_weights: Default::default(),
            _marker: PhantomData,
        }
    }
}
impl<'a> PhysicalDeviceCubicWeightsFeaturesQCOM<'a> {
    pub fn selectable_cubic_weights(mut self, selectable_cubic_weights: Bool32) -> Self {
        self.selectable_cubic_weights = selectable_cubic_weights;
        self
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SamplerCubicWeightsCreateInfoQCOM<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub cubic_weights: CubicFilterWeightsQCOM,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for SamplerCubicWeightsCreateInfoQCOM<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::SAMPLER_CUBIC_WEIGHTS_CREATE_INFO_QCOM,
            p_next: core::ptr::null(),
            cubic_weights: Default::default(),
            _marker: PhantomData,
        }
    }
}
impl<'a> SamplerCubicWeightsCreateInfoQCOM<'a> {
    pub fn cubic_weights(mut self, cubic_weights: CubicFilterWeightsQCOM) -> Self {
        self.cubic_weights = cubic_weights;
        self
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct BlitImageCubicWeightsInfoQCOM<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub cubic_weights: CubicFilterWeightsQCOM,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for BlitImageCubicWeightsInfoQCOM<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::BLIT_IMAGE_CUBIC_WEIGHTS_INFO_QCOM,
            p_next: core::ptr::null(),
            cubic_weights: Default::default(),
            _marker: PhantomData,
        }
    }
}
impl<'a> BlitImageCubicWeightsInfoQCOM<'a> {
    pub fn cubic_weights(mut self, cubic_weights: CubicFilterWeightsQCOM) -> Self {
        self.cubic_weights = cubic_weights;
        self
    }
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
