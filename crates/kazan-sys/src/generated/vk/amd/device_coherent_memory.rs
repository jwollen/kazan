#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{c_char, c_int, c_void};
use core::marker::PhantomData;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceCoherentMemoryFeaturesAMD<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub device_coherent_memory: Bool32,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for PhysicalDeviceCoherentMemoryFeaturesAMD<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::PHYSICAL_DEVICE_COHERENT_MEMORY_FEATURES_AMD,
            p_next: core::ptr::null_mut(),
            device_coherent_memory: Default::default(),
            _marker: PhantomData,
        }
    }
}
