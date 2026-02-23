#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{c_char, c_int, c_void};
#[repr(C)]
pub struct HdrVividDynamicMetadataHUAWEI {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub dynamic_metadata_size: usize,
    pub p_dynamic_metadata: *const c_void,
}
#[repr(C)]
pub struct PhysicalDeviceHdrVividFeaturesHUAWEI {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub hdr_vivid: Bool32,
}
