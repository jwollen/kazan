#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use std::ffi::{c_char, c_int, c_void};
#[repr(C)]
#[derive(Copy, Clone)]
pub struct DisplayNativeHdrSurfaceCapabilitiesAMD {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub local_dimming_support: Bool32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SwapchainDisplayNativeHdrCreateInfoAMD {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub local_dimming_enable: Bool32,
}
pub type PFN_vkSetLocalDimmingAMD = unsafe extern "system" fn(
    device: Device,
    swap_chain: SwapchainKHR,
    local_dimming_enable: Bool32,
);
