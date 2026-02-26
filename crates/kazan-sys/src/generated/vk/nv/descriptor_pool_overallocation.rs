#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{c_char, c_int, c_void};
use core::marker::PhantomData;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceDescriptorPoolOverallocationFeaturesNV<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub descriptor_pool_overallocation: Bool32,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for PhysicalDeviceDescriptorPoolOverallocationFeaturesNV<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::PHYSICAL_DEVICE_DESCRIPTOR_POOL_OVERALLOCATION_FEATURES_NV,
            p_next: core::ptr::null_mut(),
            descriptor_pool_overallocation: Default::default(),
            _marker: PhantomData,
        }
    }
}
