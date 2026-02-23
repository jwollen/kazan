#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{c_char, c_int, c_void};
#[repr(C)]
pub struct PhysicalDeviceMapMemoryPlacedFeaturesEXT {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub memory_map_placed: Bool32,
    pub memory_map_range_placed: Bool32,
    pub memory_unmap_reserve: Bool32,
}
#[repr(C)]
pub struct PhysicalDeviceMapMemoryPlacedPropertiesEXT {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub min_placed_memory_map_alignment: DeviceSize,
}
#[repr(C)]
pub struct MemoryMapPlacedInfoEXT {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub p_placed_address: *mut c_void,
}
