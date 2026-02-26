#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{c_char, c_int, c_void};
use core::marker::PhantomData;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceYcbcrDegammaFeaturesQCOM<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub ycbcr_degamma: Bool32,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for PhysicalDeviceYcbcrDegammaFeaturesQCOM<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::PHYSICAL_DEVICE_YCBCR_DEGAMMA_FEATURES_QCOM,
            p_next: core::ptr::null_mut(),
            ycbcr_degamma: Default::default(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SamplerYcbcrConversionYcbcrDegammaCreateInfoQCOM<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub enable_y_degamma: Bool32,
    pub enable_cb_cr_degamma: Bool32,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for SamplerYcbcrConversionYcbcrDegammaCreateInfoQCOM<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::SAMPLER_YCBCR_CONVERSION_YCBCR_DEGAMMA_CREATE_INFO_QCOM,
            p_next: core::ptr::null_mut(),
            enable_y_degamma: Default::default(),
            enable_cb_cr_degamma: Default::default(),
            _marker: PhantomData,
        }
    }
}
