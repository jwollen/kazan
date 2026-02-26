#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{c_char, c_int, c_void};
use core::marker::PhantomData;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDevice4444FormatsFeaturesEXT<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub format_a4r4g4b4: Bool32,
    pub format_a4b4g4r4: Bool32,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for PhysicalDevice4444FormatsFeaturesEXT<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::PHYSICAL_DEVICE_4444_FORMATS_FEATURES_EXT,
            p_next: core::ptr::null_mut(),
            format_a4r4g4b4: Default::default(),
            format_a4b4g4r4: Default::default(),
            _marker: PhantomData,
        }
    }
}
