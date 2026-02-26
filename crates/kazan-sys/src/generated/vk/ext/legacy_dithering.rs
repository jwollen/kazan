#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{c_char, c_int, c_void};
use core::marker::PhantomData;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceLegacyDitheringFeaturesEXT<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub legacy_dithering: Bool32,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for PhysicalDeviceLegacyDitheringFeaturesEXT<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::PHYSICAL_DEVICE_LEGACY_DITHERING_FEATURES_EXT,
            p_next: core::ptr::null_mut(),
            legacy_dithering: Default::default(),
            _marker: PhantomData,
        }
    }
}
