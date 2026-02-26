#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{c_char, c_int, c_void};
use core::marker::PhantomData;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceYcbcrImageArraysFeaturesEXT<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub ycbcr_image_arrays: Bool32,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for PhysicalDeviceYcbcrImageArraysFeaturesEXT<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::PHYSICAL_DEVICE_YCBCR_IMAGE_ARRAYS_FEATURES_EXT,
            p_next: core::ptr::null_mut(),
            ycbcr_image_arrays: Default::default(),
            _marker: PhantomData,
        }
    }
}
