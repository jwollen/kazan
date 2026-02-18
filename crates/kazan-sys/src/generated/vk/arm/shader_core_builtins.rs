#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use std::ffi::{c_char, c_int, c_void};
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceShaderCoreBuiltinsPropertiesARM {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub shader_core_mask: u64,
    pub shader_core_count: u32,
    pub shader_warps_per_core: u32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceShaderCoreBuiltinsFeaturesARM {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub shader_core_builtins: Bool32,
}
