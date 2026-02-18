#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use std::ffi::{c_char, c_int, c_void};
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SamplerBorderColorComponentMappingCreateInfoEXT {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub components: ComponentMapping,
    pub srgb: Bool32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceBorderColorSwizzleFeaturesEXT {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub border_color_swizzle: Bool32,
    pub border_color_swizzle_from_image: Bool32,
}
