#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{c_char, c_int, c_void};
use core::marker::PhantomData;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceFragmentDensityMap2FeaturesEXT<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub fragment_density_map_deferred: Bool32,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for PhysicalDeviceFragmentDensityMap2FeaturesEXT<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::PHYSICAL_DEVICE_FRAGMENT_DENSITY_MAP_2_FEATURES_EXT,
            p_next: core::ptr::null_mut(),
            fragment_density_map_deferred: Default::default(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceFragmentDensityMap2PropertiesEXT<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub subsampled_loads: Bool32,
    pub subsampled_coarse_reconstruction_early_access: Bool32,
    pub max_subsampled_array_layers: u32,
    pub max_descriptor_set_subsampled_samplers: u32,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for PhysicalDeviceFragmentDensityMap2PropertiesEXT<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::PHYSICAL_DEVICE_FRAGMENT_DENSITY_MAP_2_PROPERTIES_EXT,
            p_next: core::ptr::null_mut(),
            subsampled_loads: Default::default(),
            subsampled_coarse_reconstruction_early_access: Default::default(),
            max_subsampled_array_layers: Default::default(),
            max_descriptor_set_subsampled_samplers: Default::default(),
            _marker: PhantomData,
        }
    }
}
