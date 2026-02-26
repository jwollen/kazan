#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{c_char, c_int, c_void};
use core::marker::PhantomData;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceMapMemoryPlacedFeaturesEXT<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub memory_map_placed: Bool32,
    pub memory_map_range_placed: Bool32,
    pub memory_unmap_reserve: Bool32,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for PhysicalDeviceMapMemoryPlacedFeaturesEXT<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::PHYSICAL_DEVICE_MAP_MEMORY_PLACED_FEATURES_EXT,
            p_next: core::ptr::null_mut(),
            memory_map_placed: Default::default(),
            memory_map_range_placed: Default::default(),
            memory_unmap_reserve: Default::default(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceMapMemoryPlacedPropertiesEXT<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub min_placed_memory_map_alignment: DeviceSize,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for PhysicalDeviceMapMemoryPlacedPropertiesEXT<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::PHYSICAL_DEVICE_MAP_MEMORY_PLACED_PROPERTIES_EXT,
            p_next: core::ptr::null_mut(),
            min_placed_memory_map_alignment: Default::default(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct MemoryMapPlacedInfoEXT<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub p_placed_address: *mut c_void,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for MemoryMapPlacedInfoEXT<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::MEMORY_MAP_PLACED_INFO_EXT,
            p_next: core::ptr::null(),
            p_placed_address: core::ptr::null_mut(),
            _marker: PhantomData,
        }
    }
}
