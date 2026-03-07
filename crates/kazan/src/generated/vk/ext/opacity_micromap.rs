//! <https://registry.khronos.org/vulkan/specs/latest/man/html/VK_EXT_opacity_micromap.html>
#![allow(unused_imports)]
use crate::{vk::Result as VkResult, vk::*, *};
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;

pub const EXTENSION_NAME: &CStr = c"VK_EXT_opacity_micromap";

pub(super) mod defs {
    #![allow(non_camel_case_types, unused_imports)]
    use crate::{vk::*, *};
    use core::ffi::{CStr, c_char, c_int, c_void};
    use core::fmt;
    use core::marker::PhantomData;

    handle_nondispatchable!(
        MicromapEXT,
        MICROMAP_EXT,
        doc = "<https://registry.khronos.org/vulkan/specs/latest/man/html/VkMicromapEXT.html>"
    );

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkMicromapBuildInfoEXT.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct MicromapBuildInfoEXT<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub ty: MicromapTypeEXT,
        pub flags: BuildMicromapFlagsEXT,
        pub mode: BuildMicromapModeEXT,
        pub dst_micromap: MicromapEXT,
        pub usage_counts_count: u32,
        pub p_usage_counts: *const MicromapUsageEXT,
        pub pp_usage_counts: *const *const MicromapUsageEXT,
        pub data: DeviceOrHostAddressConstKHR<'a>,
        pub scratch_data: DeviceOrHostAddressKHR<'a>,
        pub triangle_array: DeviceOrHostAddressConstKHR<'a>,
        pub triangle_array_stride: DeviceSize,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for MicromapBuildInfoEXT<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("MicromapBuildInfoEXT")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("ty", &self.ty)
                .field("flags", &self.flags)
                .field("mode", &self.mode)
                .field("dst_micromap", &self.dst_micromap)
                .field("usage_counts_count", &self.usage_counts_count)
                .field("p_usage_counts", &self.p_usage_counts)
                .field("pp_usage_counts", &self.pp_usage_counts)
                .field("data", &self.data)
                .field("scratch_data", &self.scratch_data)
                .field("triangle_array", &self.triangle_array)
                .field("triangle_array_stride", &self.triangle_array_stride)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for MicromapBuildInfoEXT<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::MICROMAP_BUILD_INFO_EXT;
    }

    impl Default for MicromapBuildInfoEXT<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null(),
                ty: Default::default(),
                flags: Default::default(),
                mode: Default::default(),
                dst_micromap: Default::default(),
                usage_counts_count: Default::default(),
                p_usage_counts: core::ptr::null(),
                pp_usage_counts: core::ptr::null(),
                data: Default::default(),
                scratch_data: Default::default(),
                triangle_array: Default::default(),
                triangle_array_stride: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> MicromapBuildInfoEXT<'a> {
        #[inline]
        pub fn ty(mut self, ty: MicromapTypeEXT) -> Self {
            self.ty = ty;
            self
        }

        #[inline]
        pub fn flags(mut self, flags: BuildMicromapFlagsEXT) -> Self {
            self.flags = flags;
            self
        }

        #[inline]
        pub fn mode(mut self, mode: BuildMicromapModeEXT) -> Self {
            self.mode = mode;
            self
        }

        #[inline]
        pub fn dst_micromap(mut self, dst_micromap: MicromapEXT) -> Self {
            self.dst_micromap = dst_micromap;
            self
        }

        #[inline]
        pub fn usage_counts(mut self, usage_counts: &'a [MicromapUsageEXT]) -> Self {
            self.usage_counts_count = usage_counts.len().try_into().unwrap();
            self.p_usage_counts = usage_counts.as_ptr();
            self
        }

        #[inline]
        pub fn usage_counts_ptrs(mut self, usage_counts_ptrs: &'a [&'a MicromapUsageEXT]) -> Self {
            self.usage_counts_count = usage_counts_ptrs.len().try_into().unwrap();
            self.pp_usage_counts = usage_counts_ptrs.as_ptr() as _;
            self
        }

        #[inline]
        pub fn data(mut self, data: DeviceOrHostAddressConstKHR<'a>) -> Self {
            self.data = data;
            self
        }

        #[inline]
        pub fn scratch_data(mut self, scratch_data: DeviceOrHostAddressKHR<'a>) -> Self {
            self.scratch_data = scratch_data;
            self
        }

        #[inline]
        pub fn triangle_array(mut self, triangle_array: DeviceOrHostAddressConstKHR<'a>) -> Self {
            self.triangle_array = triangle_array;
            self
        }

        #[inline]
        pub fn triangle_array_stride(mut self, triangle_array_stride: DeviceSize) -> Self {
            self.triangle_array_stride = triangle_array_stride;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkMicromapCreateInfoEXT.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct MicromapCreateInfoEXT<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub create_flags: MicromapCreateFlagsEXT,
        pub buffer: Buffer,
        pub offset: DeviceSize,
        pub size: DeviceSize,
        pub ty: MicromapTypeEXT,
        pub device_address: DeviceAddress,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for MicromapCreateInfoEXT<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("MicromapCreateInfoEXT")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("create_flags", &self.create_flags)
                .field("buffer", &self.buffer)
                .field("offset", &self.offset)
                .field("size", &self.size)
                .field("ty", &self.ty)
                .field("device_address", &self.device_address)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for MicromapCreateInfoEXT<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::MICROMAP_CREATE_INFO_EXT;
    }

    impl Default for MicromapCreateInfoEXT<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null(),
                create_flags: Default::default(),
                buffer: Default::default(),
                offset: Default::default(),
                size: Default::default(),
                ty: Default::default(),
                device_address: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> MicromapCreateInfoEXT<'a> {
        #[inline]
        pub fn create_flags(mut self, create_flags: MicromapCreateFlagsEXT) -> Self {
            self.create_flags = create_flags;
            self
        }

        #[inline]
        pub fn buffer(mut self, buffer: Buffer) -> Self {
            self.buffer = buffer;
            self
        }

        #[inline]
        pub fn offset(mut self, offset: DeviceSize) -> Self {
            self.offset = offset;
            self
        }

        #[inline]
        pub fn size(mut self, size: DeviceSize) -> Self {
            self.size = size;
            self
        }

        #[inline]
        pub fn ty(mut self, ty: MicromapTypeEXT) -> Self {
            self.ty = ty;
            self
        }

        #[inline]
        pub fn device_address(mut self, device_address: DeviceAddress) -> Self {
            self.device_address = device_address;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkMicromapVersionInfoEXT.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct MicromapVersionInfoEXT<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub p_version_data: *const u8,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for MicromapVersionInfoEXT<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("MicromapVersionInfoEXT")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("p_version_data", &self.p_version_data)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for MicromapVersionInfoEXT<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::MICROMAP_VERSION_INFO_EXT;
    }

    impl Default for MicromapVersionInfoEXT<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null(),
                p_version_data: core::ptr::null(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> MicromapVersionInfoEXT<'a> {}

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkCopyMicromapInfoEXT.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct CopyMicromapInfoEXT<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub src: MicromapEXT,
        pub dst: MicromapEXT,
        pub mode: CopyMicromapModeEXT,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for CopyMicromapInfoEXT<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("CopyMicromapInfoEXT")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("src", &self.src)
                .field("dst", &self.dst)
                .field("mode", &self.mode)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for CopyMicromapInfoEXT<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::COPY_MICROMAP_INFO_EXT;
    }

    impl Default for CopyMicromapInfoEXT<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null(),
                src: Default::default(),
                dst: Default::default(),
                mode: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> CopyMicromapInfoEXT<'a> {
        #[inline]
        pub fn src(mut self, src: MicromapEXT) -> Self {
            self.src = src;
            self
        }

        #[inline]
        pub fn dst(mut self, dst: MicromapEXT) -> Self {
            self.dst = dst;
            self
        }

        #[inline]
        pub fn mode(mut self, mode: CopyMicromapModeEXT) -> Self {
            self.mode = mode;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkCopyMicromapToMemoryInfoEXT.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct CopyMicromapToMemoryInfoEXT<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub src: MicromapEXT,
        pub dst: DeviceOrHostAddressKHR<'a>,
        pub mode: CopyMicromapModeEXT,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for CopyMicromapToMemoryInfoEXT<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("CopyMicromapToMemoryInfoEXT")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("src", &self.src)
                .field("dst", &self.dst)
                .field("mode", &self.mode)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for CopyMicromapToMemoryInfoEXT<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::COPY_MICROMAP_TO_MEMORY_INFO_EXT;
    }

    impl Default for CopyMicromapToMemoryInfoEXT<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null(),
                src: Default::default(),
                dst: Default::default(),
                mode: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> CopyMicromapToMemoryInfoEXT<'a> {
        #[inline]
        pub fn src(mut self, src: MicromapEXT) -> Self {
            self.src = src;
            self
        }

        #[inline]
        pub fn dst(mut self, dst: DeviceOrHostAddressKHR<'a>) -> Self {
            self.dst = dst;
            self
        }

        #[inline]
        pub fn mode(mut self, mode: CopyMicromapModeEXT) -> Self {
            self.mode = mode;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkCopyMemoryToMicromapInfoEXT.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct CopyMemoryToMicromapInfoEXT<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub src: DeviceOrHostAddressConstKHR<'a>,
        pub dst: MicromapEXT,
        pub mode: CopyMicromapModeEXT,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for CopyMemoryToMicromapInfoEXT<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("CopyMemoryToMicromapInfoEXT")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("src", &self.src)
                .field("dst", &self.dst)
                .field("mode", &self.mode)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for CopyMemoryToMicromapInfoEXT<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::COPY_MEMORY_TO_MICROMAP_INFO_EXT;
    }

    impl Default for CopyMemoryToMicromapInfoEXT<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null(),
                src: Default::default(),
                dst: Default::default(),
                mode: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> CopyMemoryToMicromapInfoEXT<'a> {
        #[inline]
        pub fn src(mut self, src: DeviceOrHostAddressConstKHR<'a>) -> Self {
            self.src = src;
            self
        }

        #[inline]
        pub fn dst(mut self, dst: MicromapEXT) -> Self {
            self.dst = dst;
            self
        }

        #[inline]
        pub fn mode(mut self, mode: CopyMicromapModeEXT) -> Self {
            self.mode = mode;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkMicromapBuildSizesInfoEXT.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct MicromapBuildSizesInfoEXT<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub micromap_size: DeviceSize,
        pub build_scratch_size: DeviceSize,
        pub discardable: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for MicromapBuildSizesInfoEXT<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("MicromapBuildSizesInfoEXT")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("micromap_size", &self.micromap_size)
                .field("build_scratch_size", &self.build_scratch_size)
                .field("discardable", &self.discardable)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for MicromapBuildSizesInfoEXT<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::MICROMAP_BUILD_SIZES_INFO_EXT;
    }

    impl Default for MicromapBuildSizesInfoEXT<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null(),
                micromap_size: Default::default(),
                build_scratch_size: Default::default(),
                discardable: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> MicromapBuildSizesInfoEXT<'a> {
        #[inline]
        pub fn micromap_size(mut self, micromap_size: DeviceSize) -> Self {
            self.micromap_size = micromap_size;
            self
        }

        #[inline]
        pub fn build_scratch_size(mut self, build_scratch_size: DeviceSize) -> Self {
            self.build_scratch_size = build_scratch_size;
            self
        }

        #[inline]
        pub fn discardable(mut self, discardable: bool) -> Self {
            self.discardable = discardable.into();
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkMicromapUsageEXT.html>
    #[repr(C)]
    #[cfg_attr(feature = "debug", derive(Debug))]
    #[derive(Copy, Clone, Default)]
    #[must_use]
    pub struct MicromapUsageEXT {
        pub count: u32,
        pub subdivision_level: u32,
        pub format: u32,
    }

    impl MicromapUsageEXT {
        #[inline]
        pub fn count(mut self, count: u32) -> Self {
            self.count = count;
            self
        }

        #[inline]
        pub fn subdivision_level(mut self, subdivision_level: u32) -> Self {
            self.subdivision_level = subdivision_level;
            self
        }

        #[inline]
        pub fn format(mut self, format: u32) -> Self {
            self.format = format;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkMicromapTriangleEXT.html>
    #[repr(C)]
    #[cfg_attr(feature = "debug", derive(Debug))]
    #[derive(Copy, Clone, Default)]
    #[must_use]
    pub struct MicromapTriangleEXT {
        pub data_offset: u32,
        pub subdivision_level: u16,
        pub format: u16,
    }

    impl MicromapTriangleEXT {
        #[inline]
        pub fn data_offset(mut self, data_offset: u32) -> Self {
            self.data_offset = data_offset;
            self
        }

        #[inline]
        pub fn subdivision_level(mut self, subdivision_level: u16) -> Self {
            self.subdivision_level = subdivision_level;
            self
        }

        #[inline]
        pub fn format(mut self, format: u16) -> Self {
            self.format = format;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPhysicalDeviceOpacityMicromapFeaturesEXT.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct PhysicalDeviceOpacityMicromapFeaturesEXT<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub micromap: Bool32,
        pub micromap_capture_replay: Bool32,
        pub micromap_host_commands: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for PhysicalDeviceOpacityMicromapFeaturesEXT<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("PhysicalDeviceOpacityMicromapFeaturesEXT")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("micromap", &self.micromap)
                .field("micromap_capture_replay", &self.micromap_capture_replay)
                .field("micromap_host_commands", &self.micromap_host_commands)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for PhysicalDeviceOpacityMicromapFeaturesEXT<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PHYSICAL_DEVICE_OPACITY_MICROMAP_FEATURES_EXT;
    }

    unsafe impl<'a> Extends<PhysicalDeviceFeatures2<'a>>
        for PhysicalDeviceOpacityMicromapFeaturesEXT<'a>
    {
    }
    unsafe impl<'a> Extends<DeviceCreateInfo<'a>> for PhysicalDeviceOpacityMicromapFeaturesEXT<'a> {}

    impl Default for PhysicalDeviceOpacityMicromapFeaturesEXT<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null_mut(),
                micromap: Default::default(),
                micromap_capture_replay: Default::default(),
                micromap_host_commands: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> PhysicalDeviceOpacityMicromapFeaturesEXT<'a> {
        #[inline]
        pub fn micromap(mut self, micromap: bool) -> Self {
            self.micromap = micromap.into();
            self
        }

        #[inline]
        pub fn micromap_capture_replay(mut self, micromap_capture_replay: bool) -> Self {
            self.micromap_capture_replay = micromap_capture_replay.into();
            self
        }

        #[inline]
        pub fn micromap_host_commands(mut self, micromap_host_commands: bool) -> Self {
            self.micromap_host_commands = micromap_host_commands.into();
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPhysicalDeviceOpacityMicromapPropertiesEXT.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct PhysicalDeviceOpacityMicromapPropertiesEXT<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub max_opacity2_state_subdivision_level: u32,
        pub max_opacity4_state_subdivision_level: u32,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for PhysicalDeviceOpacityMicromapPropertiesEXT<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("PhysicalDeviceOpacityMicromapPropertiesEXT")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field(
                    "max_opacity2_state_subdivision_level",
                    &self.max_opacity2_state_subdivision_level,
                )
                .field(
                    "max_opacity4_state_subdivision_level",
                    &self.max_opacity4_state_subdivision_level,
                )
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for PhysicalDeviceOpacityMicromapPropertiesEXT<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PHYSICAL_DEVICE_OPACITY_MICROMAP_PROPERTIES_EXT;
    }

    unsafe impl<'a> Extends<PhysicalDeviceProperties2<'a>>
        for PhysicalDeviceOpacityMicromapPropertiesEXT<'a>
    {
    }

    impl Default for PhysicalDeviceOpacityMicromapPropertiesEXT<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null_mut(),
                max_opacity2_state_subdivision_level: Default::default(),
                max_opacity4_state_subdivision_level: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> PhysicalDeviceOpacityMicromapPropertiesEXT<'a> {
        #[inline]
        pub fn max_opacity2_state_subdivision_level(
            mut self,
            max_opacity2_state_subdivision_level: u32,
        ) -> Self {
            self.max_opacity2_state_subdivision_level = max_opacity2_state_subdivision_level;
            self
        }

        #[inline]
        pub fn max_opacity4_state_subdivision_level(
            mut self,
            max_opacity4_state_subdivision_level: u32,
        ) -> Self {
            self.max_opacity4_state_subdivision_level = max_opacity4_state_subdivision_level;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkAccelerationStructureTrianglesOpacityMicromapEXT.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct AccelerationStructureTrianglesOpacityMicromapEXT<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub index_type: IndexType,
        pub index_buffer: DeviceOrHostAddressConstKHR<'a>,
        pub index_stride: DeviceSize,
        pub base_triangle: u32,
        pub usage_counts_count: u32,
        pub p_usage_counts: *const MicromapUsageEXT,
        pub pp_usage_counts: *const *const MicromapUsageEXT,
        pub micromap: MicromapEXT,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for AccelerationStructureTrianglesOpacityMicromapEXT<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("AccelerationStructureTrianglesOpacityMicromapEXT")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("index_type", &self.index_type)
                .field("index_buffer", &self.index_buffer)
                .field("index_stride", &self.index_stride)
                .field("base_triangle", &self.base_triangle)
                .field("usage_counts_count", &self.usage_counts_count)
                .field("p_usage_counts", &self.p_usage_counts)
                .field("pp_usage_counts", &self.pp_usage_counts)
                .field("micromap", &self.micromap)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for AccelerationStructureTrianglesOpacityMicromapEXT<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::ACCELERATION_STRUCTURE_TRIANGLES_OPACITY_MICROMAP_EXT;
    }

    unsafe impl<'a> Extends<AccelerationStructureGeometryTrianglesDataKHR<'a>>
        for AccelerationStructureTrianglesOpacityMicromapEXT<'a>
    {
    }
    #[cfg(feature = "provisional")]
    unsafe impl<'a> Extends<AccelerationStructureDenseGeometryFormatTrianglesDataAMDX<'a>>
        for AccelerationStructureTrianglesOpacityMicromapEXT<'a>
    {
    }

    impl Default for AccelerationStructureTrianglesOpacityMicromapEXT<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null_mut(),
                index_type: Default::default(),
                index_buffer: Default::default(),
                index_stride: Default::default(),
                base_triangle: Default::default(),
                usage_counts_count: Default::default(),
                p_usage_counts: core::ptr::null(),
                pp_usage_counts: core::ptr::null(),
                micromap: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> AccelerationStructureTrianglesOpacityMicromapEXT<'a> {
        #[inline]
        pub fn index_type(mut self, index_type: IndexType) -> Self {
            self.index_type = index_type;
            self
        }

        #[inline]
        pub fn index_buffer(mut self, index_buffer: DeviceOrHostAddressConstKHR<'a>) -> Self {
            self.index_buffer = index_buffer;
            self
        }

        #[inline]
        pub fn index_stride(mut self, index_stride: DeviceSize) -> Self {
            self.index_stride = index_stride;
            self
        }

        #[inline]
        pub fn base_triangle(mut self, base_triangle: u32) -> Self {
            self.base_triangle = base_triangle;
            self
        }

        #[inline]
        pub fn usage_counts(mut self, usage_counts: &'a [MicromapUsageEXT]) -> Self {
            self.usage_counts_count = usage_counts.len().try_into().unwrap();
            self.p_usage_counts = usage_counts.as_ptr();
            self
        }

        #[inline]
        pub fn usage_counts_ptrs(mut self, usage_counts_ptrs: &'a [&'a MicromapUsageEXT]) -> Self {
            self.usage_counts_count = usage_counts_ptrs.len().try_into().unwrap();
            self.pp_usage_counts = usage_counts_ptrs.as_ptr() as _;
            self
        }

        #[inline]
        pub fn micromap(mut self, micromap: MicromapEXT) -> Self {
            self.micromap = micromap;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkMicromapTypeEXT.html>
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct MicromapTypeEXT(i32);

    impl MicromapTypeEXT {
        pub const OPACITY_MICROMAP_EXT: Self = Self(0);
        // VK_NV_displacement_micromap
        pub const DISPLACEMENT_MICROMAP_NV: Self = Self(1000397000);
    }

    impl fmt::Debug for MicromapTypeEXT {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let name = match *self {
                Self::OPACITY_MICROMAP_EXT => Some("OPACITY_MICROMAP_EXT"),
                Self::DISPLACEMENT_MICROMAP_NV => Some("DISPLACEMENT_MICROMAP_NV"),
                _ => None,
            };
            if let Some(name) = name {
                f.write_str(name)
            } else {
                self.0.fmt(f)
            }
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkCopyMicromapModeEXT.html>
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct CopyMicromapModeEXT(i32);

    impl CopyMicromapModeEXT {
        pub const CLONE_EXT: Self = Self(0);
        pub const SERIALIZE_EXT: Self = Self(1);
        pub const DESERIALIZE_EXT: Self = Self(2);
        pub const COMPACT_EXT: Self = Self(3);
    }

    impl fmt::Debug for CopyMicromapModeEXT {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let name = match *self {
                Self::CLONE_EXT => Some("CLONE_EXT"),
                Self::SERIALIZE_EXT => Some("SERIALIZE_EXT"),
                Self::DESERIALIZE_EXT => Some("DESERIALIZE_EXT"),
                Self::COMPACT_EXT => Some("COMPACT_EXT"),
                _ => None,
            };
            if let Some(name) = name {
                f.write_str(name)
            } else {
                self.0.fmt(f)
            }
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkBuildMicromapModeEXT.html>
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct BuildMicromapModeEXT(i32);

    impl BuildMicromapModeEXT {
        pub const BUILD_EXT: Self = Self(0);
    }

    impl fmt::Debug for BuildMicromapModeEXT {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let name = match *self {
                Self::BUILD_EXT => Some("BUILD_EXT"),
                _ => None,
            };
            if let Some(name) = name {
                f.write_str(name)
            } else {
                self.0.fmt(f)
            }
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkOpacityMicromapFormatEXT.html>
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct OpacityMicromapFormatEXT(i32);

    impl OpacityMicromapFormatEXT {
        pub const _2_STATE_EXT: Self = Self(1);
        pub const _4_STATE_EXT: Self = Self(2);
    }

    impl fmt::Debug for OpacityMicromapFormatEXT {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let name = match *self {
                Self::_2_STATE_EXT => Some("_2_STATE_EXT"),
                Self::_4_STATE_EXT => Some("_4_STATE_EXT"),
                _ => None,
            };
            if let Some(name) = name {
                f.write_str(name)
            } else {
                self.0.fmt(f)
            }
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkOpacityMicromapSpecialIndexEXT.html>
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct OpacityMicromapSpecialIndexEXT(i32);

    impl OpacityMicromapSpecialIndexEXT {
        pub const FULLY_TRANSPARENT_EXT: Self = Self(-1);
        pub const FULLY_OPAQUE_EXT: Self = Self(-2);
        pub const FULLY_UNKNOWN_TRANSPARENT_EXT: Self = Self(-3);
        pub const FULLY_UNKNOWN_OPAQUE_EXT: Self = Self(-4);
        // VK_NV_cluster_acceleration_structure
        pub const CLUSTER_GEOMETRY_DISABLE_OPACITY_MICROMAP_NV: Self = Self(-5);
    }

    impl fmt::Debug for OpacityMicromapSpecialIndexEXT {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let name = match *self {
                Self::FULLY_TRANSPARENT_EXT => Some("FULLY_TRANSPARENT_EXT"),
                Self::FULLY_OPAQUE_EXT => Some("FULLY_OPAQUE_EXT"),
                Self::FULLY_UNKNOWN_TRANSPARENT_EXT => Some("FULLY_UNKNOWN_TRANSPARENT_EXT"),
                Self::FULLY_UNKNOWN_OPAQUE_EXT => Some("FULLY_UNKNOWN_OPAQUE_EXT"),
                Self::CLUSTER_GEOMETRY_DISABLE_OPACITY_MICROMAP_NV => {
                    Some("CLUSTER_GEOMETRY_DISABLE_OPACITY_MICROMAP_NV")
                }
                _ => None,
            };
            if let Some(name) = name {
                f.write_str(name)
            } else {
                self.0.fmt(f)
            }
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkBuildMicromapFlagsEXT.html>
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Hash)]
    pub struct BuildMicromapFlagsEXT(Flags);
    vk_bitflags_wrapped!(BuildMicromapFlagsEXT, Flags);

    impl BuildMicromapFlagsEXT {
        pub const PREFER_FAST_TRACE_EXT: Self =
            Self(BuildMicromapFlagBitsEXT::PREFER_FAST_TRACE_EXT.0);
        pub const PREFER_FAST_BUILD_EXT: Self =
            Self(BuildMicromapFlagBitsEXT::PREFER_FAST_BUILD_EXT.0);
        pub const ALLOW_COMPACTION_EXT: Self =
            Self(BuildMicromapFlagBitsEXT::ALLOW_COMPACTION_EXT.0);
    }

    impl fmt::Debug for BuildMicromapFlagsEXT {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            const KNOWN: &[(Flags, &str)] = &[
                (
                    BuildMicromapFlagsEXT::PREFER_FAST_TRACE_EXT.0,
                    "PREFER_FAST_TRACE_EXT",
                ),
                (
                    BuildMicromapFlagsEXT::PREFER_FAST_BUILD_EXT.0,
                    "PREFER_FAST_BUILD_EXT",
                ),
                (
                    BuildMicromapFlagsEXT::ALLOW_COMPACTION_EXT.0,
                    "ALLOW_COMPACTION_EXT",
                ),
            ];
            debug_flags(f, KNOWN, self.0)
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkBuildMicromapFlagBitsEXT.html>
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, Hash)]
    pub struct BuildMicromapFlagBitsEXT(u32);

    impl BuildMicromapFlagBitsEXT {
        pub const PREFER_FAST_TRACE_EXT: Self = Self(1 << 0);
        pub const PREFER_FAST_BUILD_EXT: Self = Self(1 << 1);
        pub const ALLOW_COMPACTION_EXT: Self = Self(1 << 2);
    }

    impl fmt::Debug for BuildMicromapFlagBitsEXT {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let name = match *self {
                Self::PREFER_FAST_TRACE_EXT => Some("PREFER_FAST_TRACE_EXT"),
                Self::PREFER_FAST_BUILD_EXT => Some("PREFER_FAST_BUILD_EXT"),
                Self::ALLOW_COMPACTION_EXT => Some("ALLOW_COMPACTION_EXT"),
                _ => None,
            };
            if let Some(name) = name {
                f.write_str(name)
            } else {
                self.0.fmt(f)
            }
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkMicromapCreateFlagsEXT.html>
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Hash)]
    pub struct MicromapCreateFlagsEXT(Flags);
    vk_bitflags_wrapped!(MicromapCreateFlagsEXT, Flags);

    impl MicromapCreateFlagsEXT {
        pub const DEVICE_ADDRESS_CAPTURE_REPLAY_EXT: Self =
            Self(MicromapCreateFlagBitsEXT::DEVICE_ADDRESS_CAPTURE_REPLAY_EXT.0);
    }

    impl fmt::Debug for MicromapCreateFlagsEXT {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            const KNOWN: &[(Flags, &str)] = &[(
                MicromapCreateFlagsEXT::DEVICE_ADDRESS_CAPTURE_REPLAY_EXT.0,
                "DEVICE_ADDRESS_CAPTURE_REPLAY_EXT",
            )];
            debug_flags(f, KNOWN, self.0)
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkMicromapCreateFlagBitsEXT.html>
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, Hash)]
    pub struct MicromapCreateFlagBitsEXT(u32);

    impl MicromapCreateFlagBitsEXT {
        pub const DEVICE_ADDRESS_CAPTURE_REPLAY_EXT: Self = Self(1 << 0);
    }

    impl fmt::Debug for MicromapCreateFlagBitsEXT {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let name = match *self {
                Self::DEVICE_ADDRESS_CAPTURE_REPLAY_EXT => {
                    Some("DEVICE_ADDRESS_CAPTURE_REPLAY_EXT")
                }
                _ => None,
            };
            if let Some(name) = name {
                f.write_str(name)
            } else {
                self.0.fmt(f)
            }
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCreateMicromapEXT.html>
    pub type PFN_vkCreateMicromapEXT = unsafe extern "system" fn(
        device: Device,
        p_create_info: *const MicromapCreateInfoEXT<'_>,
        p_allocator: *const AllocationCallbacks<'_>,
        p_micromap: *mut MicromapEXT,
    ) -> vk::Result;
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdBuildMicromapsEXT.html>
    pub type PFN_vkCmdBuildMicromapsEXT = unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        info_count: u32,
        p_infos: *const MicromapBuildInfoEXT<'_>,
    );
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkBuildMicromapsEXT.html>
    pub type PFN_vkBuildMicromapsEXT = unsafe extern "system" fn(
        device: Device,
        deferred_operation: DeferredOperationKHR,
        info_count: u32,
        p_infos: *const MicromapBuildInfoEXT<'_>,
    ) -> vk::Result;
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkDestroyMicromapEXT.html>
    pub type PFN_vkDestroyMicromapEXT = unsafe extern "system" fn(
        device: Device,
        micromap: MicromapEXT,
        p_allocator: *const AllocationCallbacks<'_>,
    );
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdCopyMicromapEXT.html>
    pub type PFN_vkCmdCopyMicromapEXT = unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        p_info: *const CopyMicromapInfoEXT<'_>,
    );
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCopyMicromapEXT.html>
    pub type PFN_vkCopyMicromapEXT = unsafe extern "system" fn(
        device: Device,
        deferred_operation: DeferredOperationKHR,
        p_info: *const CopyMicromapInfoEXT<'_>,
    ) -> vk::Result;
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdCopyMicromapToMemoryEXT.html>
    pub type PFN_vkCmdCopyMicromapToMemoryEXT = unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        p_info: *const CopyMicromapToMemoryInfoEXT<'_>,
    );
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCopyMicromapToMemoryEXT.html>
    pub type PFN_vkCopyMicromapToMemoryEXT = unsafe extern "system" fn(
        device: Device,
        deferred_operation: DeferredOperationKHR,
        p_info: *const CopyMicromapToMemoryInfoEXT<'_>,
    ) -> vk::Result;
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdCopyMemoryToMicromapEXT.html>
    pub type PFN_vkCmdCopyMemoryToMicromapEXT = unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        p_info: *const CopyMemoryToMicromapInfoEXT<'_>,
    );
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCopyMemoryToMicromapEXT.html>
    pub type PFN_vkCopyMemoryToMicromapEXT = unsafe extern "system" fn(
        device: Device,
        deferred_operation: DeferredOperationKHR,
        p_info: *const CopyMemoryToMicromapInfoEXT<'_>,
    ) -> vk::Result;
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdWriteMicromapsPropertiesEXT.html>
    pub type PFN_vkCmdWriteMicromapsPropertiesEXT = unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        micromap_count: u32,
        p_micromaps: *const MicromapEXT,
        query_type: QueryType,
        query_pool: QueryPool,
        first_query: u32,
    );
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkWriteMicromapsPropertiesEXT.html>
    pub type PFN_vkWriteMicromapsPropertiesEXT = unsafe extern "system" fn(
        device: Device,
        micromap_count: u32,
        p_micromaps: *const MicromapEXT,
        query_type: QueryType,
        data_size: usize,
        p_data: *mut c_void,
        stride: usize,
    ) -> vk::Result;
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkGetDeviceMicromapCompatibilityEXT.html>
    pub type PFN_vkGetDeviceMicromapCompatibilityEXT = unsafe extern "system" fn(
        device: Device,
        p_version_info: *const MicromapVersionInfoEXT<'_>,
        p_compatibility: *mut AccelerationStructureCompatibilityKHR,
    );
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkGetMicromapBuildSizesEXT.html>
    pub type PFN_vkGetMicromapBuildSizesEXT = unsafe extern "system" fn(
        device: Device,
        build_type: AccelerationStructureBuildTypeKHR,
        p_build_info: *const MicromapBuildInfoEXT<'_>,
        p_size_info: *mut MicromapBuildSizesInfoEXT<'_>,
    );
}

pub struct DeviceFn {
    create_micromap_ext: PFN_vkCreateMicromapEXT,
    destroy_micromap_ext: PFN_vkDestroyMicromapEXT,
    cmd_build_micromaps_ext: PFN_vkCmdBuildMicromapsEXT,
    build_micromaps_ext: PFN_vkBuildMicromapsEXT,
    copy_micromap_ext: PFN_vkCopyMicromapEXT,
    copy_micromap_to_memory_ext: PFN_vkCopyMicromapToMemoryEXT,
    copy_memory_to_micromap_ext: PFN_vkCopyMemoryToMicromapEXT,
    write_micromaps_properties_ext: PFN_vkWriteMicromapsPropertiesEXT,
    cmd_copy_micromap_ext: PFN_vkCmdCopyMicromapEXT,
    cmd_copy_micromap_to_memory_ext: PFN_vkCmdCopyMicromapToMemoryEXT,
    cmd_copy_memory_to_micromap_ext: PFN_vkCmdCopyMemoryToMicromapEXT,
    cmd_write_micromaps_properties_ext: PFN_vkCmdWriteMicromapsPropertiesEXT,
    get_device_micromap_compatibility_ext: PFN_vkGetDeviceMicromapCompatibilityEXT,
    get_micromap_build_sizes_ext: PFN_vkGetMicromapBuildSizesEXT,
}

impl DeviceFn {
    pub unsafe fn load(
        load: impl Fn(&CStr) -> Option<PFN_vkVoidFunction>,
    ) -> core::result::Result<Self, MissingEntryPointError> {
        unsafe {
            Ok(Self {
                create_micromap_ext: transmute(
                    load(c"vkCreateMicromapEXT").ok_or(MissingEntryPointError)?,
                ),
                destroy_micromap_ext: transmute(
                    load(c"vkDestroyMicromapEXT").ok_or(MissingEntryPointError)?,
                ),
                cmd_build_micromaps_ext: transmute(
                    load(c"vkCmdBuildMicromapsEXT").ok_or(MissingEntryPointError)?,
                ),
                build_micromaps_ext: transmute(
                    load(c"vkBuildMicromapsEXT").ok_or(MissingEntryPointError)?,
                ),
                copy_micromap_ext: transmute(
                    load(c"vkCopyMicromapEXT").ok_or(MissingEntryPointError)?,
                ),
                copy_micromap_to_memory_ext: transmute(
                    load(c"vkCopyMicromapToMemoryEXT").ok_or(MissingEntryPointError)?,
                ),
                copy_memory_to_micromap_ext: transmute(
                    load(c"vkCopyMemoryToMicromapEXT").ok_or(MissingEntryPointError)?,
                ),
                write_micromaps_properties_ext: transmute(
                    load(c"vkWriteMicromapsPropertiesEXT").ok_or(MissingEntryPointError)?,
                ),
                cmd_copy_micromap_ext: transmute(
                    load(c"vkCmdCopyMicromapEXT").ok_or(MissingEntryPointError)?,
                ),
                cmd_copy_micromap_to_memory_ext: transmute(
                    load(c"vkCmdCopyMicromapToMemoryEXT").ok_or(MissingEntryPointError)?,
                ),
                cmd_copy_memory_to_micromap_ext: transmute(
                    load(c"vkCmdCopyMemoryToMicromapEXT").ok_or(MissingEntryPointError)?,
                ),
                cmd_write_micromaps_properties_ext: transmute(
                    load(c"vkCmdWriteMicromapsPropertiesEXT").ok_or(MissingEntryPointError)?,
                ),
                get_device_micromap_compatibility_ext: transmute(
                    load(c"vkGetDeviceMicromapCompatibilityEXT").ok_or(MissingEntryPointError)?,
                ),
                get_micromap_build_sizes_ext: transmute(
                    load(c"vkGetMicromapBuildSizesEXT").ok_or(MissingEntryPointError)?,
                ),
            })
        }
    }
}

impl DeviceFn {
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCreateMicromapEXT.html>
    #[inline]
    pub unsafe fn create_micromap_ext(
        &self,
        device: Device,
        create_info: &MicromapCreateInfoEXT<'_>,
        allocator: Option<&AllocationCallbacks<'_>>,
    ) -> crate::Result<MicromapEXT> {
        unsafe {
            let mut micromap = core::mem::MaybeUninit::uninit();
            let result = (self.create_micromap_ext)(
                device,
                create_info,
                allocator.to_raw_ptr(),
                micromap.as_mut_ptr(),
            );

            match result {
                VkResult::SUCCESS => Ok(micromap.assume_init()),
                err => Err(err),
            }
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkDestroyMicromapEXT.html>
    #[inline]
    pub unsafe fn destroy_micromap_ext(
        &self,
        device: Device,
        micromap: MicromapEXT,
        allocator: Option<&AllocationCallbacks<'_>>,
    ) {
        unsafe { (self.destroy_micromap_ext)(device, micromap, allocator.to_raw_ptr()) }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdBuildMicromapsEXT.html>
    #[inline]
    pub unsafe fn cmd_build_micromaps_ext(
        &self,
        command_buffer: CommandBuffer,
        infos: &[MicromapBuildInfoEXT<'_>],
    ) {
        unsafe {
            (self.cmd_build_micromaps_ext)(
                command_buffer,
                infos.len().try_into().unwrap(),
                infos.as_ptr() as _,
            )
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkBuildMicromapsEXT.html>
    #[inline]
    pub unsafe fn build_micromaps_ext(
        &self,
        device: Device,
        deferred_operation: DeferredOperationKHR,
        infos: &[MicromapBuildInfoEXT<'_>],
    ) -> crate::Result<()> {
        unsafe {
            let result = (self.build_micromaps_ext)(
                device,
                deferred_operation,
                infos.len().try_into().unwrap(),
                infos.as_ptr() as _,
            );

            match result {
                VkResult::SUCCESS => Ok(()),
                err => Err(err),
            }
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCopyMicromapEXT.html>
    #[inline]
    pub unsafe fn copy_micromap_ext(
        &self,
        device: Device,
        deferred_operation: DeferredOperationKHR,
        info: &CopyMicromapInfoEXT<'_>,
    ) -> crate::Result<()> {
        unsafe {
            let result = (self.copy_micromap_ext)(device, deferred_operation, info);

            match result {
                VkResult::SUCCESS => Ok(()),
                err => Err(err),
            }
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCopyMicromapToMemoryEXT.html>
    #[inline]
    pub unsafe fn copy_micromap_to_memory_ext(
        &self,
        device: Device,
        deferred_operation: DeferredOperationKHR,
        info: &CopyMicromapToMemoryInfoEXT<'_>,
    ) -> crate::Result<()> {
        unsafe {
            let result = (self.copy_micromap_to_memory_ext)(device, deferred_operation, info);

            match result {
                VkResult::SUCCESS => Ok(()),
                err => Err(err),
            }
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCopyMemoryToMicromapEXT.html>
    #[inline]
    pub unsafe fn copy_memory_to_micromap_ext(
        &self,
        device: Device,
        deferred_operation: DeferredOperationKHR,
        info: &CopyMemoryToMicromapInfoEXT<'_>,
    ) -> crate::Result<()> {
        unsafe {
            let result = (self.copy_memory_to_micromap_ext)(device, deferred_operation, info);

            match result {
                VkResult::SUCCESS => Ok(()),
                err => Err(err),
            }
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkWriteMicromapsPropertiesEXT.html>
    #[inline]
    pub unsafe fn write_micromaps_properties_ext(
        &self,
        device: Device,
        micromaps: &[MicromapEXT],
        query_type: QueryType,
        data: &mut [u8],
        stride: usize,
    ) -> crate::Result<()> {
        unsafe {
            let result = (self.write_micromaps_properties_ext)(
                device,
                micromaps.len().try_into().unwrap(),
                micromaps.as_ptr() as _,
                query_type,
                data.len().try_into().unwrap(),
                data.as_mut_ptr() as _,
                stride,
            );

            match result {
                VkResult::SUCCESS => Ok(()),
                err => Err(err),
            }
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdCopyMicromapEXT.html>
    #[inline]
    pub unsafe fn cmd_copy_micromap_ext(
        &self,
        command_buffer: CommandBuffer,
        info: &CopyMicromapInfoEXT<'_>,
    ) {
        unsafe { (self.cmd_copy_micromap_ext)(command_buffer, info) }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdCopyMicromapToMemoryEXT.html>
    #[inline]
    pub unsafe fn cmd_copy_micromap_to_memory_ext(
        &self,
        command_buffer: CommandBuffer,
        info: &CopyMicromapToMemoryInfoEXT<'_>,
    ) {
        unsafe { (self.cmd_copy_micromap_to_memory_ext)(command_buffer, info) }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdCopyMemoryToMicromapEXT.html>
    #[inline]
    pub unsafe fn cmd_copy_memory_to_micromap_ext(
        &self,
        command_buffer: CommandBuffer,
        info: &CopyMemoryToMicromapInfoEXT<'_>,
    ) {
        unsafe { (self.cmd_copy_memory_to_micromap_ext)(command_buffer, info) }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdWriteMicromapsPropertiesEXT.html>
    #[inline]
    pub unsafe fn cmd_write_micromaps_properties_ext(
        &self,
        command_buffer: CommandBuffer,
        micromaps: &[MicromapEXT],
        query_type: QueryType,
        query_pool: QueryPool,
        first_query: u32,
    ) {
        unsafe {
            (self.cmd_write_micromaps_properties_ext)(
                command_buffer,
                micromaps.len().try_into().unwrap(),
                micromaps.as_ptr() as _,
                query_type,
                query_pool,
                first_query,
            )
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkGetDeviceMicromapCompatibilityEXT.html>
    #[inline]
    pub unsafe fn get_device_micromap_compatibility_ext(
        &self,
        device: Device,
        version_info: &MicromapVersionInfoEXT<'_>,
    ) -> AccelerationStructureCompatibilityKHR {
        unsafe {
            let mut compatibility = core::mem::MaybeUninit::uninit();
            (self.get_device_micromap_compatibility_ext)(
                device,
                version_info,
                compatibility.as_mut_ptr(),
            );
            compatibility.assume_init()
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkGetMicromapBuildSizesEXT.html>
    #[inline]
    pub unsafe fn get_micromap_build_sizes_ext(
        &self,
        device: Device,
        build_type: AccelerationStructureBuildTypeKHR,
        build_info: &MicromapBuildInfoEXT<'_>,
        size_info: &mut MicromapBuildSizesInfoEXT<'_>,
    ) {
        unsafe { (self.get_micromap_build_sizes_ext)(device, build_type, build_info, size_info) }
    }
}
