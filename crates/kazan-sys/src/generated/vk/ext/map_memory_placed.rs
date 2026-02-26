#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{c_char, c_int, c_void};
use core::marker::PhantomData;
#[repr(C)]
pub struct PhysicalDeviceMapMemoryPlacedFeaturesEXT<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub memory_map_placed: Bool32,
    pub memory_map_range_placed: Bool32,
    pub memory_unmap_reserve: Bool32,
    pub _marker: PhantomData<&'a ()>,
}
#[repr(C)]
pub struct PhysicalDeviceMapMemoryPlacedPropertiesEXT<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub min_placed_memory_map_alignment: DeviceSize,
    pub _marker: PhantomData<&'a ()>,
}
#[repr(C)]
pub struct MemoryMapPlacedInfoEXT<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub p_placed_address: *mut c_void,
    pub _marker: PhantomData<&'a ()>,
}
