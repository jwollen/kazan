#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use std::ffi::{c_char, c_int, c_void};
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceImageViewImageFormatInfoEXT {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub image_view_type: ImageViewType,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct FilterCubicImageViewImageFormatPropertiesEXT {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub filter_cubic: Bool32,
    pub filter_cubic_minmax: Bool32,
}
