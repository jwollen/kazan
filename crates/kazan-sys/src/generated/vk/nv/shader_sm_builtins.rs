#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{c_char, c_int, c_void};
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceShaderSMBuiltinsPropertiesNV {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub shader_sm_count: u32,
    pub shader_warps_per_sm: u32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceShaderSMBuiltinsFeaturesNV {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub shader_sm_builtins: Bool32,
}
