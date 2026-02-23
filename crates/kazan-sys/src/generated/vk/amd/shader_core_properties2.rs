#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{c_char, c_int, c_void};
#[repr(C)]
pub struct PhysicalDeviceShaderCoreProperties2AMD {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub shader_core_features: ShaderCorePropertiesFlagsAMD,
    pub active_compute_unit_count: u32,
}
bitflags! {
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Default)]
    pub struct ShaderCorePropertiesFlagsAMD: Flags {
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ShaderCorePropertiesFlagBitsAMD(u32);
impl ShaderCorePropertiesFlagBitsAMD {}
