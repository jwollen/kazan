#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{CStr, c_char, c_int, c_void};
use core::marker::PhantomData;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceFragmentDensityMapLayeredPropertiesVALVE<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub max_fragment_density_map_layers: u32,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for PhysicalDeviceFragmentDensityMapLayeredPropertiesVALVE<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::PHYSICAL_DEVICE_FRAGMENT_DENSITY_MAP_LAYERED_PROPERTIES_VALVE,
            p_next: core::ptr::null_mut(),
            max_fragment_density_map_layers: Default::default(),
            _marker: PhantomData,
        }
    }
}
impl<'a> PhysicalDeviceFragmentDensityMapLayeredPropertiesVALVE<'a> {
    pub fn max_fragment_density_map_layers(mut self, max_fragment_density_map_layers: u32) -> Self {
        self.max_fragment_density_map_layers = max_fragment_density_map_layers;
        self
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceFragmentDensityMapLayeredFeaturesVALVE<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub fragment_density_map_layered: Bool32,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for PhysicalDeviceFragmentDensityMapLayeredFeaturesVALVE<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::PHYSICAL_DEVICE_FRAGMENT_DENSITY_MAP_LAYERED_FEATURES_VALVE,
            p_next: core::ptr::null_mut(),
            fragment_density_map_layered: Default::default(),
            _marker: PhantomData,
        }
    }
}
impl<'a> PhysicalDeviceFragmentDensityMapLayeredFeaturesVALVE<'a> {
    pub fn fragment_density_map_layered(mut self, fragment_density_map_layered: Bool32) -> Self {
        self.fragment_density_map_layered = fragment_density_map_layered;
        self
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PipelineFragmentDensityMapLayeredCreateInfoVALVE<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub max_fragment_density_map_layers: u32,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for PipelineFragmentDensityMapLayeredCreateInfoVALVE<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::PIPELINE_FRAGMENT_DENSITY_MAP_LAYERED_CREATE_INFO_VALVE,
            p_next: core::ptr::null(),
            max_fragment_density_map_layers: Default::default(),
            _marker: PhantomData,
        }
    }
}
impl<'a> PipelineFragmentDensityMapLayeredCreateInfoVALVE<'a> {
    pub fn max_fragment_density_map_layers(mut self, max_fragment_density_map_layers: u32) -> Self {
        self.max_fragment_density_map_layers = max_fragment_density_map_layers;
        self
    }
}
