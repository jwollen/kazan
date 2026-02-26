#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{c_char, c_int, c_void};
use core::marker::PhantomData;
#[repr(C)]
pub struct DisplaySurfaceStereoCreateInfoNV<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub stereo_type: DisplaySurfaceStereoTypeNV,
    pub _marker: PhantomData<&'a ()>,
}
#[repr(C)]
pub struct DisplayModeStereoPropertiesNV<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub hdmi3_d_supported: Bool32,
    pub _marker: PhantomData<&'a ()>,
}
#[repr(transparent)]
#[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DisplaySurfaceStereoTypeNV(i32);
impl DisplaySurfaceStereoTypeNV {
    pub const NONE_NV: Self = Self(0);
    pub const ONBOARD_DIN_NV: Self = Self(1);
    pub const HDMI_3D_NV: Self = Self(2);
    pub const INBAND_DISPLAYPORT_NV: Self = Self(3);
}
