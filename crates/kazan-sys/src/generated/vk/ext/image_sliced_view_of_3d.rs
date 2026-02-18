#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use std::ffi::{c_char, c_int, c_void};
pub const REMAINING_3D_SLICES_EXT: u32 = !0;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ImageViewSlicedCreateInfoEXT {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub slice_offset: u32,
    pub slice_count: u32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceImageSlicedViewOf3DFeaturesEXT {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub image_sliced_view_of3_d: Bool32,
}
