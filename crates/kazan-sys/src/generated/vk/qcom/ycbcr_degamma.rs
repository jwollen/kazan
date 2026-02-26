#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{c_char, c_int, c_void};
use core::marker::PhantomData;
#[repr(C)]
pub struct PhysicalDeviceYcbcrDegammaFeaturesQCOM<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub ycbcr_degamma: Bool32,
    pub _marker: PhantomData<&'a ()>,
}
#[repr(C)]
pub struct SamplerYcbcrConversionYcbcrDegammaCreateInfoQCOM<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub enable_y_degamma: Bool32,
    pub enable_cb_cr_degamma: Bool32,
    pub _marker: PhantomData<&'a ()>,
}
