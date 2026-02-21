#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{c_char, c_int, c_void};
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceYcbcr2Plane444FormatsFeaturesEXT {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub ycbcr2plane444_formats: Bool32,
}
