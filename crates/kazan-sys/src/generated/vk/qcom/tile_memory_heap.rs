#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{c_char, c_int, c_void};
#[repr(C)]
#[derive(Copy, Clone)]
pub struct TileMemoryBindInfoQCOM {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub memory: DeviceMemory,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceTileMemoryHeapFeaturesQCOM {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub tile_memory_heap: Bool32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceTileMemoryHeapPropertiesQCOM {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub queue_submit_boundary: Bool32,
    pub tile_buffer_transfers: Bool32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct TileMemorySizeInfoQCOM {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub size: DeviceSize,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct TileMemoryRequirementsQCOM {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub size: DeviceSize,
    pub alignment: DeviceSize,
}
pub type PFN_vkCmdBindTileMemoryQCOM = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    p_tile_memory_bind_info: *const TileMemoryBindInfoQCOM,
);
