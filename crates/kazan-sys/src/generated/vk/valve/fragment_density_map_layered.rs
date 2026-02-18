#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use std::ffi::{c_char, c_int, c_void};
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceFragmentDensityMapLayeredPropertiesVALVE {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub max_fragment_density_map_layers: u32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceFragmentDensityMapLayeredFeaturesVALVE {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub fragment_density_map_layered: Bool32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PipelineFragmentDensityMapLayeredCreateInfoVALVE {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub max_fragment_density_map_layers: u32,
}
