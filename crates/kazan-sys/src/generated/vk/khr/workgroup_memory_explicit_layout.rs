#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{c_char, c_int, c_void};
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceWorkgroupMemoryExplicitLayoutFeaturesKHR {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub workgroup_memory_explicit_layout: Bool32,
    pub workgroup_memory_explicit_layout_scalar_block_layout: Bool32,
    pub workgroup_memory_explicit_layout8_bit_access: Bool32,
    pub workgroup_memory_explicit_layout16_bit_access: Bool32,
}
