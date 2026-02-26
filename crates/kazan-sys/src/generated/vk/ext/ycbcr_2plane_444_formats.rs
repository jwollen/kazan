#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{c_char, c_int, c_void};
use core::marker::PhantomData;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceYcbcr2Plane444FormatsFeaturesEXT<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub ycbcr2plane444_formats: Bool32,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for PhysicalDeviceYcbcr2Plane444FormatsFeaturesEXT<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::PHYSICAL_DEVICE_YCBCR_2_PLANE_444_FORMATS_FEATURES_EXT,
            p_next: core::ptr::null_mut(),
            ycbcr2plane444_formats: Default::default(),
            _marker: PhantomData,
        }
    }
}
