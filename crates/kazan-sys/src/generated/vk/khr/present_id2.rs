#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{c_char, c_int, c_void};
#[repr(C)]
pub struct PhysicalDevicePresentId2FeaturesKHR {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub present_id2: Bool32,
}
#[repr(C)]
pub struct PresentId2KHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub swapchain_count: u32,
    pub p_present_ids: *const u64,
}
#[repr(C)]
pub struct SurfaceCapabilitiesPresentId2KHR {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub present_id2_supported: Bool32,
}
