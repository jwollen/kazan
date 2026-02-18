#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use std::ffi::{c_char, c_int, c_void};
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDevicePresentBarrierFeaturesNV {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub present_barrier: Bool32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SurfaceCapabilitiesPresentBarrierNV {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub present_barrier_supported: Bool32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SwapchainPresentBarrierCreateInfoNV {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub present_barrier_enable: Bool32,
}
