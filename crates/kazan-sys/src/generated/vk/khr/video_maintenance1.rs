#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use std::ffi::{c_char, c_int, c_void};
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceVideoMaintenance1FeaturesKHR {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub video_maintenance1: Bool32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VideoInlineQueryInfoKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub query_pool: QueryPool,
    pub first_query: u32,
    pub query_count: u32,
}
