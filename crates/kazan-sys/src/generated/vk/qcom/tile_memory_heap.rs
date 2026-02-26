#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{c_char, c_int, c_void};
use core::marker::PhantomData;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct TileMemoryBindInfoQCOM<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub memory: DeviceMemory,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for TileMemoryBindInfoQCOM<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::TILE_MEMORY_BIND_INFO_QCOM,
            p_next: core::ptr::null(),
            memory: Default::default(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceTileMemoryHeapFeaturesQCOM<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub tile_memory_heap: Bool32,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for PhysicalDeviceTileMemoryHeapFeaturesQCOM<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::PHYSICAL_DEVICE_TILE_MEMORY_HEAP_FEATURES_QCOM,
            p_next: core::ptr::null_mut(),
            tile_memory_heap: Default::default(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceTileMemoryHeapPropertiesQCOM<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub queue_submit_boundary: Bool32,
    pub tile_buffer_transfers: Bool32,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for PhysicalDeviceTileMemoryHeapPropertiesQCOM<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::PHYSICAL_DEVICE_TILE_MEMORY_HEAP_PROPERTIES_QCOM,
            p_next: core::ptr::null_mut(),
            queue_submit_boundary: Default::default(),
            tile_buffer_transfers: Default::default(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct TileMemorySizeInfoQCOM<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub size: DeviceSize,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for TileMemorySizeInfoQCOM<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::TILE_MEMORY_SIZE_INFO_QCOM,
            p_next: core::ptr::null(),
            size: Default::default(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct TileMemoryRequirementsQCOM<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub size: DeviceSize,
    pub alignment: DeviceSize,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for TileMemoryRequirementsQCOM<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::TILE_MEMORY_REQUIREMENTS_QCOM,
            p_next: core::ptr::null_mut(),
            size: Default::default(),
            alignment: Default::default(),
            _marker: PhantomData,
        }
    }
}
pub type PFN_vkCmdBindTileMemoryQCOM = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    p_tile_memory_bind_info: *const TileMemoryBindInfoQCOM<'_>,
);
