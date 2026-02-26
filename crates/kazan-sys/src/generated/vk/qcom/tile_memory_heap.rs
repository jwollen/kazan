#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{c_char, c_int, c_void};
use core::marker::PhantomData;
#[repr(C)]
pub struct TileMemoryBindInfoQCOM<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub memory: DeviceMemory,
    pub _marker: PhantomData<&'a ()>,
}
#[repr(C)]
pub struct PhysicalDeviceTileMemoryHeapFeaturesQCOM<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub tile_memory_heap: Bool32,
    pub _marker: PhantomData<&'a ()>,
}
#[repr(C)]
pub struct PhysicalDeviceTileMemoryHeapPropertiesQCOM<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub queue_submit_boundary: Bool32,
    pub tile_buffer_transfers: Bool32,
    pub _marker: PhantomData<&'a ()>,
}
#[repr(C)]
pub struct TileMemorySizeInfoQCOM<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub size: DeviceSize,
    pub _marker: PhantomData<&'a ()>,
}
#[repr(C)]
pub struct TileMemoryRequirementsQCOM<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub size: DeviceSize,
    pub alignment: DeviceSize,
    pub _marker: PhantomData<&'a ()>,
}
pub type PFN_vkCmdBindTileMemoryQCOM = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    p_tile_memory_bind_info: *const TileMemoryBindInfoQCOM<'_>,
);
