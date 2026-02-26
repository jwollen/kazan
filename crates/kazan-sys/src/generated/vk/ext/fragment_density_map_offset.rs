#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{c_char, c_int, c_void};
use core::marker::PhantomData;
pub type RenderingEndInfoEXT<'a> = RenderingEndInfoKHR<'a>;
pub type PFN_vkCmdEndRendering2EXT = PFN_vkCmdEndRendering2KHR;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceFragmentDensityMapOffsetFeaturesEXT<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub fragment_density_map_offset: Bool32,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for PhysicalDeviceFragmentDensityMapOffsetFeaturesEXT<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::PHYSICAL_DEVICE_FRAGMENT_DENSITY_MAP_OFFSET_FEATURES_EXT,
            p_next: core::ptr::null_mut(),
            fragment_density_map_offset: Default::default(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceFragmentDensityMapOffsetPropertiesEXT<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub fragment_density_offset_granularity: Extent2D,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for PhysicalDeviceFragmentDensityMapOffsetPropertiesEXT<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::PHYSICAL_DEVICE_FRAGMENT_DENSITY_MAP_OFFSET_PROPERTIES_EXT,
            p_next: core::ptr::null_mut(),
            fragment_density_offset_granularity: Default::default(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct RenderPassFragmentDensityMapOffsetEndInfoEXT<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub fragment_density_offset_count: u32,
    pub p_fragment_density_offsets: *const Offset2D,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for RenderPassFragmentDensityMapOffsetEndInfoEXT<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::RENDER_PASS_FRAGMENT_DENSITY_MAP_OFFSET_END_INFO_EXT,
            p_next: core::ptr::null(),
            fragment_density_offset_count: Default::default(),
            p_fragment_density_offsets: core::ptr::null(),
            _marker: PhantomData,
        }
    }
}
