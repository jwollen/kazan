#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{c_char, c_int, c_void};
use core::marker::PhantomData;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceImageViewMinLodFeaturesEXT<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub min_lod: Bool32,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for PhysicalDeviceImageViewMinLodFeaturesEXT<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::PHYSICAL_DEVICE_IMAGE_VIEW_MIN_LOD_FEATURES_EXT,
            p_next: core::ptr::null_mut(),
            min_lod: Default::default(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ImageViewMinLodCreateInfoEXT<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub min_lod: f32,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for ImageViewMinLodCreateInfoEXT<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::IMAGE_VIEW_MIN_LOD_CREATE_INFO_EXT,
            p_next: core::ptr::null(),
            min_lod: Default::default(),
            _marker: PhantomData,
        }
    }
}
