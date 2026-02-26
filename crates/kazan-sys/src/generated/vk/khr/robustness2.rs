#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{c_char, c_int, c_void};
use core::marker::PhantomData;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceRobustness2FeaturesKHR<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub robust_buffer_access2: Bool32,
    pub robust_image_access2: Bool32,
    pub null_descriptor: Bool32,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for PhysicalDeviceRobustness2FeaturesKHR<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::PHYSICAL_DEVICE_ROBUSTNESS_2_FEATURES_KHR,
            p_next: core::ptr::null_mut(),
            robust_buffer_access2: Default::default(),
            robust_image_access2: Default::default(),
            null_descriptor: Default::default(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceRobustness2PropertiesKHR<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub robust_storage_buffer_access_size_alignment: DeviceSize,
    pub robust_uniform_buffer_access_size_alignment: DeviceSize,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for PhysicalDeviceRobustness2PropertiesKHR<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::PHYSICAL_DEVICE_ROBUSTNESS_2_PROPERTIES_KHR,
            p_next: core::ptr::null_mut(),
            robust_storage_buffer_access_size_alignment: Default::default(),
            robust_uniform_buffer_access_size_alignment: Default::default(),
            _marker: PhantomData,
        }
    }
}
