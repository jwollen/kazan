#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use std::ffi::{c_char, c_int, c_void};
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDevice4444FormatsFeaturesEXT {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub format_a4r4g4b4: Bool32,
    pub format_a4b4g4r4: Bool32,
}
