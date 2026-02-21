#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{c_char, c_int, c_void};
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ImageViewASTCDecodeModeEXT {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub decode_mode: Format,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceASTCDecodeFeaturesEXT {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub decode_mode_shared_exponent: Bool32,
}
