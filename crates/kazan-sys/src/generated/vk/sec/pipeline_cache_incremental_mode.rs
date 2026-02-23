#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{c_char, c_int, c_void};
#[repr(C)]
pub struct PhysicalDevicePipelineCacheIncrementalModeFeaturesSEC {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub pipeline_cache_incremental_mode: Bool32,
}
