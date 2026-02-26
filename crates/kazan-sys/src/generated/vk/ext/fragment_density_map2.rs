#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{c_char, c_int, c_void};
use core::marker::PhantomData;
#[repr(C)]
pub struct PhysicalDeviceFragmentDensityMap2FeaturesEXT<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub fragment_density_map_deferred: Bool32,
    pub _marker: PhantomData<&'a ()>,
}
#[repr(C)]
pub struct PhysicalDeviceFragmentDensityMap2PropertiesEXT<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub subsampled_loads: Bool32,
    pub subsampled_coarse_reconstruction_early_access: Bool32,
    pub max_subsampled_array_layers: u32,
    pub max_descriptor_set_subsampled_samplers: u32,
    pub _marker: PhantomData<&'a ()>,
}
