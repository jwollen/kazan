//! <https://registry.khronos.org/vulkan/specs/latest/man/html/VK_EXT_memory_decompression.html>
#![allow(unused_imports)]
use crate::{vk::Result as VkResult, vk::*, *};
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
use core::ptr;

pub const EXTENSION_NAME: &CStr = c"VK_EXT_memory_decompression";

pub(super) mod defs {
    #![allow(non_camel_case_types, unused_imports)]
    use crate::{vk::*, *};
    use core::ffi::{CStr, c_char, c_int, c_void};
    use core::fmt;
    use core::marker::PhantomData;
    use core::ptr;

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPhysicalDeviceMemoryDecompressionFeaturesEXT.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct PhysicalDeviceMemoryDecompressionFeaturesEXT<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub memory_decompression: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for PhysicalDeviceMemoryDecompressionFeaturesEXT<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("PhysicalDeviceMemoryDecompressionFeaturesEXT")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("memory_decompression", &self.memory_decompression)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for PhysicalDeviceMemoryDecompressionFeaturesEXT<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PHYSICAL_DEVICE_MEMORY_DECOMPRESSION_FEATURES_EXT;
    }

    unsafe impl<'a> Extends<PhysicalDeviceFeatures2<'a>>
        for PhysicalDeviceMemoryDecompressionFeaturesEXT<'a>
    {
    }
    unsafe impl<'a> Extends<DeviceCreateInfo<'a>> for PhysicalDeviceMemoryDecompressionFeaturesEXT<'a> {}

    impl Default for PhysicalDeviceMemoryDecompressionFeaturesEXT<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null_mut(),
                memory_decompression: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> PhysicalDeviceMemoryDecompressionFeaturesEXT<'a> {
        #[inline]
        pub fn memory_decompression(mut self, memory_decompression: bool) -> Self {
            self.memory_decompression = memory_decompression.into();
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPhysicalDeviceMemoryDecompressionPropertiesEXT.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct PhysicalDeviceMemoryDecompressionPropertiesEXT<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub decompression_methods: MemoryDecompressionMethodFlagsEXT,
        pub max_decompression_indirect_count: u64,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for PhysicalDeviceMemoryDecompressionPropertiesEXT<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("PhysicalDeviceMemoryDecompressionPropertiesEXT")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("decompression_methods", &self.decompression_methods)
                .field(
                    "max_decompression_indirect_count",
                    &self.max_decompression_indirect_count,
                )
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for PhysicalDeviceMemoryDecompressionPropertiesEXT<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PHYSICAL_DEVICE_MEMORY_DECOMPRESSION_PROPERTIES_EXT;
    }

    unsafe impl<'a> Extends<PhysicalDeviceProperties2<'a>>
        for PhysicalDeviceMemoryDecompressionPropertiesEXT<'a>
    {
    }

    impl Default for PhysicalDeviceMemoryDecompressionPropertiesEXT<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null_mut(),
                decompression_methods: Default::default(),
                max_decompression_indirect_count: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> PhysicalDeviceMemoryDecompressionPropertiesEXT<'a> {
        #[inline]
        pub fn decompression_methods(
            mut self,
            decompression_methods: MemoryDecompressionMethodFlagsEXT,
        ) -> Self {
            self.decompression_methods = decompression_methods;
            self
        }

        #[inline]
        pub fn max_decompression_indirect_count(
            mut self,
            max_decompression_indirect_count: u64,
        ) -> Self {
            self.max_decompression_indirect_count = max_decompression_indirect_count;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkDecompressMemoryRegionEXT.html>
    #[repr(C)]
    #[cfg_attr(feature = "debug", derive(Debug))]
    #[derive(Copy, Clone, Default)]
    #[must_use]
    pub struct DecompressMemoryRegionEXT {
        pub src_address: DeviceAddress,
        pub dst_address: DeviceAddress,
        pub compressed_size: DeviceSize,
        pub decompressed_size: DeviceSize,
    }

    impl DecompressMemoryRegionEXT {
        #[inline]
        pub fn src_address(mut self, src_address: DeviceAddress) -> Self {
            self.src_address = src_address;
            self
        }

        #[inline]
        pub fn dst_address(mut self, dst_address: DeviceAddress) -> Self {
            self.dst_address = dst_address;
            self
        }

        #[inline]
        pub fn compressed_size(mut self, compressed_size: DeviceSize) -> Self {
            self.compressed_size = compressed_size;
            self
        }

        #[inline]
        pub fn decompressed_size(mut self, decompressed_size: DeviceSize) -> Self {
            self.decompressed_size = decompressed_size;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkDecompressMemoryInfoEXT.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct DecompressMemoryInfoEXT<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub decompression_method: MemoryDecompressionMethodFlagsEXT,
        pub region_count: u32,
        pub p_regions: *const DecompressMemoryRegionEXT,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for DecompressMemoryInfoEXT<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("DecompressMemoryInfoEXT")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("decompression_method", &self.decompression_method)
                .field("region_count", &self.region_count)
                .field("p_regions", &self.p_regions)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for DecompressMemoryInfoEXT<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::DECOMPRESS_MEMORY_INFO_EXT;
    }

    impl Default for DecompressMemoryInfoEXT<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null(),
                decompression_method: Default::default(),
                region_count: Default::default(),
                p_regions: ptr::null(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> DecompressMemoryInfoEXT<'a> {
        #[inline]
        pub fn decompression_method(
            mut self,
            decompression_method: MemoryDecompressionMethodFlagsEXT,
        ) -> Self {
            self.decompression_method = decompression_method;
            self
        }

        #[inline]
        pub fn regions(mut self, regions: &'a [DecompressMemoryRegionEXT]) -> Self {
            self.region_count = regions.len().try_into().unwrap();
            self.p_regions = regions.as_ptr();
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkMemoryDecompressionMethodFlagsEXT.html>
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Hash)]
    pub struct MemoryDecompressionMethodFlagsEXT(Flags64);
    vk_bitflags_wrapped!(MemoryDecompressionMethodFlagsEXT, Flags64);

    impl MemoryDecompressionMethodFlagsEXT {
        pub const GDEFLATE_1_0_EXT: Self =
            Self(MemoryDecompressionMethodFlagBitsEXT::GDEFLATE_1_0_EXT.0);
        pub const GDEFLATE_1_0_NV: Self = Self::GDEFLATE_1_0_EXT;
    }

    impl fmt::Debug for MemoryDecompressionMethodFlagsEXT {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            const KNOWN: &[(Flags64, &str)] = &[(
                MemoryDecompressionMethodFlagsEXT::GDEFLATE_1_0_EXT.0,
                "GDEFLATE_1_0_EXT",
            )];
            debug_flags(f, KNOWN, self.0)
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkMemoryDecompressionMethodFlagBitsEXT.html>
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, Hash)]
    pub struct MemoryDecompressionMethodFlagBitsEXT(u64);

    impl MemoryDecompressionMethodFlagBitsEXT {
        pub const GDEFLATE_1_0_EXT: Self = Self(1 << 0);
    }

    impl fmt::Debug for MemoryDecompressionMethodFlagBitsEXT {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let name = match *self {
                Self::GDEFLATE_1_0_EXT => Some("GDEFLATE_1_0_EXT"),
                _ => None,
            };
            if let Some(name) = name {
                f.write_str(name)
            } else {
                self.0.fmt(f)
            }
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdDecompressMemoryEXT.html>
    pub type PFN_vkCmdDecompressMemoryEXT = unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        p_decompress_memory_info_ext: *const DecompressMemoryInfoEXT<'_>,
    );
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdDecompressMemoryIndirectCountEXT.html>
    pub type PFN_vkCmdDecompressMemoryIndirectCountEXT = unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        decompression_method: MemoryDecompressionMethodFlagsEXT,
        indirect_commands_address: DeviceAddress,
        indirect_commands_count_address: DeviceAddress,
        max_decompression_count: u32,
        stride: u32,
    );
}

pub struct DeviceFn {
    cmd_decompress_memory_ext: PFN_vkCmdDecompressMemoryEXT,
    cmd_decompress_memory_indirect_count_ext: PFN_vkCmdDecompressMemoryIndirectCountEXT,
}

impl DeviceFn {
    pub unsafe fn load(
        load: impl Fn(&CStr) -> Option<PFN_vkVoidFunction>,
    ) -> core::result::Result<Self, MissingEntryPointError> {
        unsafe {
            Ok(Self {
                cmd_decompress_memory_ext: transmute(
                    load(c"vkCmdDecompressMemoryEXT").ok_or(MissingEntryPointError)?,
                ),
                cmd_decompress_memory_indirect_count_ext: transmute(
                    load(c"vkCmdDecompressMemoryIndirectCountEXT").ok_or(MissingEntryPointError)?,
                ),
            })
        }
    }
}

impl DeviceFn {
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdDecompressMemoryEXT.html>
    #[inline]
    pub unsafe fn cmd_decompress_memory_ext(
        &self,
        command_buffer: CommandBuffer,
        decompress_memory_info_ext: &DecompressMemoryInfoEXT<'_>,
    ) {
        unsafe { (self.cmd_decompress_memory_ext)(command_buffer, decompress_memory_info_ext) }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdDecompressMemoryIndirectCountEXT.html>
    #[inline]
    pub unsafe fn cmd_decompress_memory_indirect_count_ext(
        &self,
        command_buffer: CommandBuffer,
        decompression_method: MemoryDecompressionMethodFlagsEXT,
        indirect_commands_address: DeviceAddress,
        indirect_commands_count_address: DeviceAddress,
        max_decompression_count: u32,
        stride: u32,
    ) {
        unsafe {
            (self.cmd_decompress_memory_indirect_count_ext)(
                command_buffer,
                decompression_method,
                indirect_commands_address,
                indirect_commands_count_address,
                max_decompression_count,
                stride,
            )
        }
    }
}
