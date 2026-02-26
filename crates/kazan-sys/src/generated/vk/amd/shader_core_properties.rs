#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{c_char, c_int, c_void};
use core::marker::PhantomData;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceShaderCorePropertiesAMD<'a> {
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
    pub _marker: PhantomData<&'a ()>,
}
impl Default for PhysicalDeviceShaderCorePropertiesAMD<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::PHYSICAL_DEVICE_SHADER_CORE_PROPERTIES_AMD,
            p_next: core::ptr::null_mut(),
            shader_engine_count: Default::default(),
            shader_arrays_per_engine_count: Default::default(),
            compute_units_per_shader_array: Default::default(),
            simd_per_compute_unit: Default::default(),
            wavefronts_per_simd: Default::default(),
            wavefront_size: Default::default(),
            sgprs_per_simd: Default::default(),
            min_sgpr_allocation: Default::default(),
            max_sgpr_allocation: Default::default(),
            sgpr_allocation_granularity: Default::default(),
            vgprs_per_simd: Default::default(),
            min_vgpr_allocation: Default::default(),
            max_vgpr_allocation: Default::default(),
            vgpr_allocation_granularity: Default::default(),
            _marker: PhantomData,
        }
    }
}
