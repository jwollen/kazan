#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{c_char, c_int, c_void};
use core::marker::PhantomData;
pub type PhysicalDeviceMemoryDecompressionFeaturesNV<'a> =
    PhysicalDeviceMemoryDecompressionFeaturesEXT<'a>;
pub type PhysicalDeviceMemoryDecompressionPropertiesNV<'a> =
    PhysicalDeviceMemoryDecompressionPropertiesEXT<'a>;
pub type MemoryDecompressionMethodFlagsNV = MemoryDecompressionMethodFlagsEXT;
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct DecompressMemoryRegionNV {
    pub src_address: DeviceAddress,
    pub dst_address: DeviceAddress,
    pub compressed_size: DeviceSize,
    pub decompressed_size: DeviceSize,
    pub decompression_method: MemoryDecompressionMethodFlagsNV,
}
pub type PFN_vkCmdDecompressMemoryNV = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    decompress_region_count: u32,
    p_decompress_memory_regions: *const DecompressMemoryRegionNV,
);
pub type PFN_vkCmdDecompressMemoryIndirectCountNV = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    indirect_commands_address: DeviceAddress,
    indirect_commands_count_address: DeviceAddress,
    stride: u32,
);
