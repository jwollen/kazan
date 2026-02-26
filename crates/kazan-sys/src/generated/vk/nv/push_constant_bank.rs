#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{c_char, c_int, c_void};
use core::marker::PhantomData;
#[repr(C)]
pub struct PushConstantBankInfoNV<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub bank: u32,
    pub _marker: PhantomData<&'a ()>,
}
#[repr(C)]
pub struct PhysicalDevicePushConstantBankFeaturesNV<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub push_constant_bank: Bool32,
    pub _marker: PhantomData<&'a ()>,
}
#[repr(C)]
pub struct PhysicalDevicePushConstantBankPropertiesNV<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub max_graphics_push_constant_banks: u32,
    pub max_compute_push_constant_banks: u32,
    pub max_graphics_push_data_banks: u32,
    pub max_compute_push_data_banks: u32,
    pub _marker: PhantomData<&'a ()>,
}
