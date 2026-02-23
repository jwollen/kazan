#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{c_char, c_int, c_void};
#[repr(C)]
pub struct SharedPresentSurfaceCapabilitiesKHR {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub shared_present_supported_usage_flags: ImageUsageFlags,
}
pub type PFN_vkGetSwapchainStatusKHR =
    unsafe extern "system" fn(device: Device, swapchain: SwapchainKHR) -> Result;
