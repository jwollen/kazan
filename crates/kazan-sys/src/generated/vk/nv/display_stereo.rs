#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{CStr, c_char, c_int, c_void};
use core::marker::PhantomData;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct DisplaySurfaceStereoCreateInfoNV<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub stereo_type: DisplaySurfaceStereoTypeNV,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for DisplaySurfaceStereoCreateInfoNV<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::DISPLAY_SURFACE_STEREO_CREATE_INFO_NV,
            p_next: core::ptr::null(),
            stereo_type: Default::default(),
            _marker: PhantomData,
        }
    }
}
impl<'a> DisplaySurfaceStereoCreateInfoNV<'a> {
    pub fn stereo_type(mut self, stereo_type: DisplaySurfaceStereoTypeNV) -> Self {
        self.stereo_type = stereo_type;
        self
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct DisplayModeStereoPropertiesNV<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub hdmi3_d_supported: Bool32,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for DisplayModeStereoPropertiesNV<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::DISPLAY_MODE_STEREO_PROPERTIES_NV,
            p_next: core::ptr::null_mut(),
            hdmi3_d_supported: Default::default(),
            _marker: PhantomData,
        }
    }
}
impl<'a> DisplayModeStereoPropertiesNV<'a> {
    pub fn hdmi3_d_supported(mut self, hdmi3_d_supported: Bool32) -> Self {
        self.hdmi3_d_supported = hdmi3_d_supported;
        self
    }
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
