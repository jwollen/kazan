#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{c_char, c_int, c_void};
use core::marker::PhantomData;
#[repr(C)]
pub struct PhysicalDeviceFragmentDensityMapLayeredPropertiesVALVE<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub max_fragment_density_map_layers: u32,
    pub _marker: PhantomData<&'a ()>,
}
#[repr(C)]
pub struct PhysicalDeviceFragmentDensityMapLayeredFeaturesVALVE<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub fragment_density_map_layered: Bool32,
    pub _marker: PhantomData<&'a ()>,
}
#[repr(C)]
pub struct PipelineFragmentDensityMapLayeredCreateInfoVALVE<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub max_fragment_density_map_layers: u32,
    pub _marker: PhantomData<&'a ()>,
}
