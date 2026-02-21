#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{c_char, c_int, c_void};
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PushConstantBankInfoNV {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub bank: u32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDevicePushConstantBankFeaturesNV {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub push_constant_bank: Bool32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDevicePushConstantBankPropertiesNV {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub max_graphics_push_constant_banks: u32,
    pub max_compute_push_constant_banks: u32,
    pub max_graphics_push_data_banks: u32,
    pub max_compute_push_data_banks: u32,
}
