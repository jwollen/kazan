#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use std::ffi::{c_char, c_int, c_void};
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PresentWait2InfoKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub present_id: u64,
    pub timeout: u64,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDevicePresentWait2FeaturesKHR {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub present_wait2: Bool32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SurfaceCapabilitiesPresentWait2KHR {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub present_wait2_supported: Bool32,
}
pub type PFN_vkWaitForPresent2KHR = unsafe extern "system" fn(
    device: Device,
    swapchain: SwapchainKHR,
    p_present_wait2_info: *const PresentWait2InfoKHR,
) -> Result;
