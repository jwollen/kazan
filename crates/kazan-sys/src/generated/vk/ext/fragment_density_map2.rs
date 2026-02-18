#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use std::ffi::{c_char, c_int, c_void};
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceFragmentDensityMap2FeaturesEXT {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub fragment_density_map_deferred: Bool32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceFragmentDensityMap2PropertiesEXT {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub subsampled_loads: Bool32,
    pub subsampled_coarse_reconstruction_early_access: Bool32,
    pub max_subsampled_array_layers: u32,
    pub max_descriptor_set_subsampled_samplers: u32,
}
