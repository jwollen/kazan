#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use std::ffi::{c_char, c_int, c_void};
#[repr(C)]
#[derive(Copy, Clone)]
pub struct QueryLowLatencySupportNV {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub p_queried_low_latency_data: *mut c_void,
}
