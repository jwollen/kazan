#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{c_char, c_int, c_void};
use core::marker::PhantomData;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct HdrVividDynamicMetadataHUAWEI<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub dynamic_metadata_size: usize,
    pub p_dynamic_metadata: *const c_void,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for HdrVividDynamicMetadataHUAWEI<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::HDR_VIVID_DYNAMIC_METADATA_HUAWEI,
            p_next: core::ptr::null(),
            dynamic_metadata_size: Default::default(),
            p_dynamic_metadata: core::ptr::null(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceHdrVividFeaturesHUAWEI<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub hdr_vivid: Bool32,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for PhysicalDeviceHdrVividFeaturesHUAWEI<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::PHYSICAL_DEVICE_HDR_VIVID_FEATURES_HUAWEI,
            p_next: core::ptr::null_mut(),
            hdr_vivid: Default::default(),
            _marker: PhantomData,
        }
    }
}
