#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use std::ffi::{c_char, c_int, c_void};
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceMemoryDecompressionFeaturesEXT {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub memory_decompression: Bool32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceMemoryDecompressionPropertiesEXT {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub decompression_methods: MemoryDecompressionMethodFlagsEXT,
    pub max_decompression_indirect_count: u64,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct DecompressMemoryRegionEXT {
    pub src_address: DeviceAddress,
    pub dst_address: DeviceAddress,
    pub compressed_size: DeviceSize,
    pub decompressed_size: DeviceSize,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct DecompressMemoryInfoEXT {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub decompression_method: MemoryDecompressionMethodFlagsEXT,
    pub region_count: u32,
    pub p_regions: *const DecompressMemoryRegionEXT,
}
bitflags! {
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Default)]
    pub struct MemoryDecompressionMethodFlagsEXT: Flags64 {
        const GDEFLATE_1_0_EXT = 1 << 0;
    }
}
pub type PFN_vkCmdDecompressMemoryEXT = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    p_decompress_memory_info_ext: *const DecompressMemoryInfoEXT,
);
pub type PFN_vkCmdDecompressMemoryIndirectCountEXT = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    decompression_method: MemoryDecompressionMethodFlagsEXT,
    indirect_commands_address: DeviceAddress,
    indirect_commands_count_address: DeviceAddress,
    max_decompression_count: u32,
    stride: u32,
);
