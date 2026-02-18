#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use std::ffi::{c_char, c_int, c_void};
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceRayTracingPositionFetchFeaturesKHR {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub ray_tracing_position_fetch: Bool32,
}
