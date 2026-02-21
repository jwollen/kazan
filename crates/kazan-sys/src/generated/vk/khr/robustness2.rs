#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{c_char, c_int, c_void};
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceRobustness2FeaturesKHR {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub robust_buffer_access2: Bool32,
    pub robust_image_access2: Bool32,
    pub null_descriptor: Bool32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceRobustness2PropertiesKHR {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub robust_storage_buffer_access_size_alignment: DeviceSize,
    pub robust_uniform_buffer_access_size_alignment: DeviceSize,
}
