#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{c_char, c_int, c_void};
pub type RenderingEndInfoEXT = RenderingEndInfoKHR;
pub type PFN_vkCmdEndRendering2EXT = PFN_vkCmdEndRendering2KHR;
#[repr(C)]
pub struct PhysicalDeviceFragmentDensityMapOffsetFeaturesEXT {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub fragment_density_map_offset: Bool32,
}
#[repr(C)]
pub struct PhysicalDeviceFragmentDensityMapOffsetPropertiesEXT {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub fragment_density_offset_granularity: Extent2D,
}
#[repr(C)]
pub struct RenderPassFragmentDensityMapOffsetEndInfoEXT {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub fragment_density_offset_count: u32,
    pub p_fragment_density_offsets: *const Offset2D,
}
