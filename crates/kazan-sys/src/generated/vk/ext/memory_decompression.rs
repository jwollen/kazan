#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{c_char, c_int, c_void};
use core::marker::PhantomData;
#[repr(C)]
pub struct PhysicalDeviceMemoryDecompressionFeaturesEXT<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub memory_decompression: Bool32,
    pub _marker: PhantomData<&'a ()>,
}
#[repr(C)]
pub struct PhysicalDeviceMemoryDecompressionPropertiesEXT<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub decompression_methods: MemoryDecompressionMethodFlagsEXT,
    pub max_decompression_indirect_count: u64,
    pub _marker: PhantomData<&'a ()>,
}
#[repr(C)]
pub struct DecompressMemoryRegionEXT {
    pub src_address: DeviceAddress,
    pub dst_address: DeviceAddress,
    pub compressed_size: DeviceSize,
    pub decompressed_size: DeviceSize,
}
#[repr(C)]
pub struct DecompressMemoryInfoEXT<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub decompression_method: MemoryDecompressionMethodFlagsEXT,
    pub region_count: u32,
    pub p_regions: *const DecompressMemoryRegionEXT,
    pub _marker: PhantomData<&'a ()>,
}
bitflags! {
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct MemoryDecompressionMethodFlagsEXT: Flags64 {
        const GDEFLATE_1_0_EXT = MemoryDecompressionMethodFlagBitsEXT::GDEFLATE_1_0_EXT.0;
        const GDEFLATE_1_0_NV = Self::GDEFLATE_1_0_EXT.bits();
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct MemoryDecompressionMethodFlagBitsEXT(u64);
impl MemoryDecompressionMethodFlagBitsEXT {
    pub const GDEFLATE_1_0_EXT: Self = Self(1 << 0);
}
pub type PFN_vkCmdDecompressMemoryEXT = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    p_decompress_memory_info_ext: *const DecompressMemoryInfoEXT<'_>,
);
pub type PFN_vkCmdDecompressMemoryIndirectCountEXT = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    decompression_method: MemoryDecompressionMethodFlagsEXT,
    indirect_commands_address: DeviceAddress,
    indirect_commands_count_address: DeviceAddress,
    max_decompression_count: u32,
    stride: u32,
);
