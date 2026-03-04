#![allow(unused_imports)]
use crate::{vk::Result as VkResult, vk::*, *};
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
pub(super) mod defs {
    #![allow(non_camel_case_types, unused_imports)]
    use crate::{vk::*, *};
    use core::ffi::{CStr, c_char, c_int, c_void};
    use core::fmt;
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
    impl DecompressMemoryRegionNV {
        pub fn src_address(mut self, src_address: DeviceAddress) -> Self {
            self.src_address = src_address;
            self
        }
        pub fn dst_address(mut self, dst_address: DeviceAddress) -> Self {
            self.dst_address = dst_address;
            self
        }
        pub fn compressed_size(mut self, compressed_size: DeviceSize) -> Self {
            self.compressed_size = compressed_size;
            self
        }
        pub fn decompressed_size(mut self, decompressed_size: DeviceSize) -> Self {
            self.decompressed_size = decompressed_size;
            self
        }
        pub fn decompression_method(
            mut self,
            decompression_method: MemoryDecompressionMethodFlagsNV,
        ) -> Self {
            self.decompression_method = decompression_method;
            self
        }
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
}
pub struct DeviceFn {
    cmd_decompress_memory_nv: PFN_vkCmdDecompressMemoryNV,
    cmd_decompress_memory_indirect_count_nv: PFN_vkCmdDecompressMemoryIndirectCountNV,
}
impl DeviceFn {
    pub unsafe fn load(
        load: impl Fn(&CStr) -> Option<PFN_vkVoidFunction>,
    ) -> core::result::Result<Self, MissingEntryPointError> {
        unsafe {
            Ok(Self {
                cmd_decompress_memory_nv: transmute(
                    load(c"vkCmdDecompressMemoryNV").ok_or(MissingEntryPointError)?,
                ),
                cmd_decompress_memory_indirect_count_nv: transmute(
                    load(c"vkCmdDecompressMemoryIndirectCountNV").ok_or(MissingEntryPointError)?,
                ),
            })
        }
    }
}
impl DeviceFn {
    pub unsafe fn cmd_decompress_memory_nv(
        &self,
        command_buffer: CommandBuffer,
        decompress_memory_regions: &[DecompressMemoryRegionNV],
    ) {
        unsafe {
            (self.cmd_decompress_memory_nv)(
                command_buffer,
                decompress_memory_regions.len().try_into().unwrap(),
                decompress_memory_regions.as_ptr() as _,
            )
        }
    }
    pub unsafe fn cmd_decompress_memory_indirect_count_nv(
        &self,
        command_buffer: CommandBuffer,
        indirect_commands_address: DeviceAddress,
        indirect_commands_count_address: DeviceAddress,
        stride: u32,
    ) {
        unsafe {
            (self.cmd_decompress_memory_indirect_count_nv)(
                command_buffer,
                indirect_commands_address,
                indirect_commands_count_address,
                stride,
            )
        }
    }
}
