#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{c_char, c_int, c_void};
use core::marker::PhantomData;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceRawAccessChainsFeaturesNV<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub shader_raw_access_chains: Bool32,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for PhysicalDeviceRawAccessChainsFeaturesNV<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::PHYSICAL_DEVICE_RAW_ACCESS_CHAINS_FEATURES_NV,
            p_next: core::ptr::null_mut(),
            shader_raw_access_chains: Default::default(),
            _marker: PhantomData,
        }
    }
}
