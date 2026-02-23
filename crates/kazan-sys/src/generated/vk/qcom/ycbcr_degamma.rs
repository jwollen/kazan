#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{c_char, c_int, c_void};
#[repr(C)]
pub struct PhysicalDeviceYcbcrDegammaFeaturesQCOM {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub ycbcr_degamma: Bool32,
}
#[repr(C)]
pub struct SamplerYcbcrConversionYcbcrDegammaCreateInfoQCOM {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub enable_y_degamma: Bool32,
    pub enable_cb_cr_degamma: Bool32,
}
