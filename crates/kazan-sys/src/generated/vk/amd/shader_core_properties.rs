#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{c_char, c_int, c_void};
#[repr(C)]
pub struct PhysicalDeviceShaderCorePropertiesAMD {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub shader_engine_count: u32,
    pub shader_arrays_per_engine_count: u32,
    pub compute_units_per_shader_array: u32,
    pub simd_per_compute_unit: u32,
    pub wavefronts_per_simd: u32,
    pub wavefront_size: u32,
    pub sgprs_per_simd: u32,
    pub min_sgpr_allocation: u32,
    pub max_sgpr_allocation: u32,
    pub sgpr_allocation_granularity: u32,
    pub vgprs_per_simd: u32,
    pub min_vgpr_allocation: u32,
    pub max_vgpr_allocation: u32,
    pub vgpr_allocation_granularity: u32,
}
