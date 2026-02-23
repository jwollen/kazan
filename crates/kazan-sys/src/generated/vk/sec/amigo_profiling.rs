#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{c_char, c_int, c_void};
#[repr(C)]
pub struct PhysicalDeviceAmigoProfilingFeaturesSEC {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub amigo_profiling: Bool32,
}
#[repr(C)]
pub struct AmigoProfilingSubmitInfoSEC {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub first_draw_timestamp: u64,
    pub swap_buffer_timestamp: u64,
}
