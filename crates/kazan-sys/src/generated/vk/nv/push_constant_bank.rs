#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{c_char, c_int, c_void};
use core::marker::PhantomData;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PushConstantBankInfoNV<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub bank: u32,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for PushConstantBankInfoNV<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::PUSH_CONSTANT_BANK_INFO_NV,
            p_next: core::ptr::null(),
            bank: Default::default(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDevicePushConstantBankFeaturesNV<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub push_constant_bank: Bool32,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for PhysicalDevicePushConstantBankFeaturesNV<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::PHYSICAL_DEVICE_PUSH_CONSTANT_BANK_FEATURES_NV,
            p_next: core::ptr::null_mut(),
            push_constant_bank: Default::default(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDevicePushConstantBankPropertiesNV<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub max_graphics_push_constant_banks: u32,
    pub max_compute_push_constant_banks: u32,
    pub max_graphics_push_data_banks: u32,
    pub max_compute_push_data_banks: u32,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for PhysicalDevicePushConstantBankPropertiesNV<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::PHYSICAL_DEVICE_PUSH_CONSTANT_BANK_PROPERTIES_NV,
            p_next: core::ptr::null_mut(),
            max_graphics_push_constant_banks: Default::default(),
            max_compute_push_constant_banks: Default::default(),
            max_graphics_push_data_banks: Default::default(),
            max_compute_push_data_banks: Default::default(),
            _marker: PhantomData,
        }
    }
}
