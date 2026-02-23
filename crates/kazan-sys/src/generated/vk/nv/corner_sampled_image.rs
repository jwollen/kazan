#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{c_char, c_int, c_void};
#[repr(C)]
pub struct PhysicalDeviceCornerSampledImageFeaturesNV {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub corner_sampled_image: Bool32,
}
