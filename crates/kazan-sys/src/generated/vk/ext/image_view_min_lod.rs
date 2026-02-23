#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{c_char, c_int, c_void};
#[repr(C)]
pub struct PhysicalDeviceImageViewMinLodFeaturesEXT {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub min_lod: Bool32,
}
#[repr(C)]
pub struct ImageViewMinLodCreateInfoEXT {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub min_lod: f32,
}
