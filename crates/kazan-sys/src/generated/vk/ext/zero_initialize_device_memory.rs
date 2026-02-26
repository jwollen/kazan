#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{c_char, c_int, c_void};
use core::marker::PhantomData;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceZeroInitializeDeviceMemoryFeaturesEXT<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub zero_initialize_device_memory: Bool32,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for PhysicalDeviceZeroInitializeDeviceMemoryFeaturesEXT<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::PHYSICAL_DEVICE_ZERO_INITIALIZE_DEVICE_MEMORY_FEATURES_EXT,
            p_next: core::ptr::null_mut(),
            zero_initialize_device_memory: Default::default(),
            _marker: PhantomData,
        }
    }
}
