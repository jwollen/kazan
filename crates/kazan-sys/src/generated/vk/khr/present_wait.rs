#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{c_char, c_int, c_void};
#[repr(C)]
pub struct PhysicalDevicePresentWaitFeaturesKHR {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub present_wait: Bool32,
}
pub type PFN_vkWaitForPresentKHR = unsafe extern "system" fn(
    device: Device,
    swapchain: SwapchainKHR,
    present_id: u64,
    timeout: u64,
) -> Result;
