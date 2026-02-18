#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use std::ffi::{c_char, c_int, c_void};
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SamplerCustomBorderColorCreateInfoEXT {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub custom_border_color: ClearColorValue,
    pub format: Format,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceCustomBorderColorPropertiesEXT {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub max_custom_border_color_samplers: u32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceCustomBorderColorFeaturesEXT {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub custom_border_colors: Bool32,
    pub custom_border_color_without_format: Bool32,
}
