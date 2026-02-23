#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{c_char, c_int, c_void};
#[repr(C)]
pub struct PhysicalDevicePresentIdFeaturesKHR {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub present_id: Bool32,
}
#[repr(C)]
pub struct PresentIdKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub swapchain_count: u32,
    pub p_present_ids: *const u64,
}
