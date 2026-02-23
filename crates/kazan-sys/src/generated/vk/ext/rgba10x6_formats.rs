#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{c_char, c_int, c_void};
#[repr(C)]
pub struct PhysicalDeviceRGBA10X6FormatsFeaturesEXT {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub format_rgba10x6_without_y_cb_cr_sampler: Bool32,
}
