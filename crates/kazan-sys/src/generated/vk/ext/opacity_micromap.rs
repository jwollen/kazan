#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{c_char, c_int, c_void};
use core::marker::PhantomData;
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct MicromapEXT(u64);
#[repr(C)]
#[derive(Copy, Clone)]
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
impl Default for MicromapBuildInfoEXT<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::MICROMAP_BUILD_INFO_EXT,
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
#[repr(C)]
#[derive(Copy, Clone)]
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
impl Default for MicromapCreateInfoEXT<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::MICROMAP_CREATE_INFO_EXT,
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
#[repr(C)]
#[derive(Copy, Clone)]
pub struct MicromapVersionInfoEXT<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub p_version_data: *const u8,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for MicromapVersionInfoEXT<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::MICROMAP_VERSION_INFO_EXT,
            p_next: core::ptr::null(),
            p_version_data: core::ptr::null(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct CopyMicromapInfoEXT<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub src: MicromapEXT,
    pub dst: MicromapEXT,
    pub mode: CopyMicromapModeEXT,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for CopyMicromapInfoEXT<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::COPY_MICROMAP_INFO_EXT,
            p_next: core::ptr::null(),
            src: Default::default(),
            dst: Default::default(),
            mode: Default::default(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct CopyMicromapToMemoryInfoEXT<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub src: MicromapEXT,
    pub dst: DeviceOrHostAddressKHR<'a>,
    pub mode: CopyMicromapModeEXT,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for CopyMicromapToMemoryInfoEXT<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::COPY_MICROMAP_TO_MEMORY_INFO_EXT,
            p_next: core::ptr::null(),
            src: Default::default(),
            dst: Default::default(),
            mode: Default::default(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct CopyMemoryToMicromapInfoEXT<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub src: DeviceOrHostAddressConstKHR<'a>,
    pub dst: MicromapEXT,
    pub mode: CopyMicromapModeEXT,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for CopyMemoryToMicromapInfoEXT<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::COPY_MEMORY_TO_MICROMAP_INFO_EXT,
            p_next: core::ptr::null(),
            src: Default::default(),
            dst: Default::default(),
            mode: Default::default(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct MicromapBuildSizesInfoEXT<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub micromap_size: DeviceSize,
    pub build_scratch_size: DeviceSize,
    pub discardable: Bool32,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for MicromapBuildSizesInfoEXT<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::MICROMAP_BUILD_SIZES_INFO_EXT,
            p_next: core::ptr::null(),
            micromap_size: Default::default(),
            build_scratch_size: Default::default(),
            discardable: Default::default(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct MicromapUsageEXT {
    pub count: u32,
    pub subdivision_level: u32,
    pub format: u32,
}
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct MicromapTriangleEXT {
    pub data_offset: u32,
    pub subdivision_level: u16,
    pub format: u16,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceOpacityMicromapFeaturesEXT<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub micromap: Bool32,
    pub micromap_capture_replay: Bool32,
    pub micromap_host_commands: Bool32,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for PhysicalDeviceOpacityMicromapFeaturesEXT<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::PHYSICAL_DEVICE_OPACITY_MICROMAP_FEATURES_EXT,
            p_next: core::ptr::null_mut(),
            micromap: Default::default(),
            micromap_capture_replay: Default::default(),
            micromap_host_commands: Default::default(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceOpacityMicromapPropertiesEXT<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub max_opacity2_state_subdivision_level: u32,
    pub max_opacity4_state_subdivision_level: u32,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for PhysicalDeviceOpacityMicromapPropertiesEXT<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::PHYSICAL_DEVICE_OPACITY_MICROMAP_PROPERTIES_EXT,
            p_next: core::ptr::null_mut(),
            max_opacity2_state_subdivision_level: Default::default(),
            max_opacity4_state_subdivision_level: Default::default(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
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
impl Default for AccelerationStructureTrianglesOpacityMicromapEXT<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::ACCELERATION_STRUCTURE_TRIANGLES_OPACITY_MICROMAP_EXT,
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
#[repr(transparent)]
#[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct MicromapTypeEXT(i32);
impl MicromapTypeEXT {
    pub const OPACITY_MICROMAP_EXT: Self = Self(0);
    pub const DISPLACEMENT_MICROMAP_NV: Self = Self(1000397000);
}
#[repr(transparent)]
#[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CopyMicromapModeEXT(i32);
impl CopyMicromapModeEXT {
    pub const CLONE_EXT: Self = Self(0);
    pub const SERIALIZE_EXT: Self = Self(1);
    pub const DESERIALIZE_EXT: Self = Self(2);
    pub const COMPACT_EXT: Self = Self(3);
}
#[repr(transparent)]
#[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct BuildMicromapModeEXT(i32);
impl BuildMicromapModeEXT {
    pub const BUILD_EXT: Self = Self(0);
}
#[repr(transparent)]
#[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct OpacityMicromapFormatEXT(i32);
impl OpacityMicromapFormatEXT {
    pub const _2_STATE_EXT: Self = Self(1);
    pub const _4_STATE_EXT: Self = Self(2);
}
#[repr(transparent)]
#[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct OpacityMicromapSpecialIndexEXT(i32);
impl OpacityMicromapSpecialIndexEXT {
    pub const FULLY_TRANSPARENT_EXT: Self = Self(-1);
    pub const FULLY_OPAQUE_EXT: Self = Self(-2);
    pub const FULLY_UNKNOWN_TRANSPARENT_EXT: Self = Self(-3);
    pub const FULLY_UNKNOWN_OPAQUE_EXT: Self = Self(-4);
    pub const CLUSTER_GEOMETRY_DISABLE_OPACITY_MICROMAP_NV: Self = Self(-5);
}
bitflags! {
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct BuildMicromapFlagsEXT: Flags {
        const PREFER_FAST_TRACE_EXT = BuildMicromapFlagBitsEXT::PREFER_FAST_TRACE_EXT.0;
        const PREFER_FAST_BUILD_EXT = BuildMicromapFlagBitsEXT::PREFER_FAST_BUILD_EXT.0;
        const ALLOW_COMPACTION_EXT = BuildMicromapFlagBitsEXT::ALLOW_COMPACTION_EXT.0;
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct BuildMicromapFlagBitsEXT(u32);
impl BuildMicromapFlagBitsEXT {
    pub const PREFER_FAST_TRACE_EXT: Self = Self(1 << 0);
    pub const PREFER_FAST_BUILD_EXT: Self = Self(1 << 1);
    pub const ALLOW_COMPACTION_EXT: Self = Self(1 << 2);
}
bitflags! {
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct MicromapCreateFlagsEXT: Flags {
        const DEVICE_ADDRESS_CAPTURE_REPLAY_EXT = MicromapCreateFlagBitsEXT::DEVICE_ADDRESS_CAPTURE_REPLAY_EXT.0;
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct MicromapCreateFlagBitsEXT(u32);
impl MicromapCreateFlagBitsEXT {
    pub const DEVICE_ADDRESS_CAPTURE_REPLAY_EXT: Self = Self(1 << 0);
}
pub type PFN_vkCreateMicromapEXT = unsafe extern "system" fn(
    device: Device,
    p_create_info: *const MicromapCreateInfoEXT<'_>,
    p_allocator: *const AllocationCallbacks<'_>,
    p_micromap: *mut MicromapEXT,
) -> Result;
pub type PFN_vkCmdBuildMicromapsEXT = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    info_count: u32,
    p_infos: *const MicromapBuildInfoEXT<'_>,
);
pub type PFN_vkBuildMicromapsEXT = unsafe extern "system" fn(
    device: Device,
    deferred_operation: DeferredOperationKHR,
    info_count: u32,
    p_infos: *const MicromapBuildInfoEXT<'_>,
) -> Result;
pub type PFN_vkDestroyMicromapEXT = unsafe extern "system" fn(
    device: Device,
    micromap: MicromapEXT,
    p_allocator: *const AllocationCallbacks<'_>,
);
pub type PFN_vkCmdCopyMicromapEXT = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    p_info: *const CopyMicromapInfoEXT<'_>,
);
pub type PFN_vkCopyMicromapEXT = unsafe extern "system" fn(
    device: Device,
    deferred_operation: DeferredOperationKHR,
    p_info: *const CopyMicromapInfoEXT<'_>,
) -> Result;
pub type PFN_vkCmdCopyMicromapToMemoryEXT = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    p_info: *const CopyMicromapToMemoryInfoEXT<'_>,
);
pub type PFN_vkCopyMicromapToMemoryEXT = unsafe extern "system" fn(
    device: Device,
    deferred_operation: DeferredOperationKHR,
    p_info: *const CopyMicromapToMemoryInfoEXT<'_>,
) -> Result;
pub type PFN_vkCmdCopyMemoryToMicromapEXT = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    p_info: *const CopyMemoryToMicromapInfoEXT<'_>,
);
pub type PFN_vkCopyMemoryToMicromapEXT = unsafe extern "system" fn(
    device: Device,
    deferred_operation: DeferredOperationKHR,
    p_info: *const CopyMemoryToMicromapInfoEXT<'_>,
) -> Result;
pub type PFN_vkCmdWriteMicromapsPropertiesEXT = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    micromap_count: u32,
    p_micromaps: *const MicromapEXT,
    query_type: QueryType,
    query_pool: QueryPool,
    first_query: u32,
);
pub type PFN_vkWriteMicromapsPropertiesEXT = unsafe extern "system" fn(
    device: Device,
    micromap_count: u32,
    p_micromaps: *const MicromapEXT,
    query_type: QueryType,
    data_size: usize,
    p_data: *mut c_void,
    stride: usize,
) -> Result;
pub type PFN_vkGetDeviceMicromapCompatibilityEXT = unsafe extern "system" fn(
    device: Device,
    p_version_info: *const MicromapVersionInfoEXT<'_>,
    p_compatibility: *mut AccelerationStructureCompatibilityKHR,
);
pub type PFN_vkGetMicromapBuildSizesEXT = unsafe extern "system" fn(
    device: Device,
    build_type: AccelerationStructureBuildTypeKHR,
    p_build_info: *const MicromapBuildInfoEXT<'_>,
    p_size_info: *mut MicromapBuildSizesInfoEXT<'_>,
);
