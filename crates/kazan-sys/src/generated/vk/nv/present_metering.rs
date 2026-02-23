#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{c_char, c_int, c_void};
#[repr(C)]
pub struct SetPresentConfigNV {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub num_frames_per_batch: u32,
    pub present_config_feedback: u32,
}
#[repr(C)]
pub struct PhysicalDevicePresentMeteringFeaturesNV {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub present_metering: Bool32,
}
