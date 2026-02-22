#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{c_char, c_int, c_void};
#[repr(C)]
#[derive(Copy, Clone)]
pub struct MicromapEXT(u64);
#[repr(C)]
#[derive(Copy, Clone)]
pub struct MicromapBuildInfoEXT {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub ty: MicromapTypeEXT,
    pub flags: BuildMicromapFlagsEXT,
    pub mode: BuildMicromapModeEXT,
    pub dst_micromap: MicromapEXT,
    pub usage_counts_count: u32,
    pub p_usage_counts: *const MicromapUsageEXT,
    pub pp_usage_counts: *const *const MicromapUsageEXT,
    pub data: DeviceOrHostAddressConstKHR,
    pub scratch_data: DeviceOrHostAddressKHR,
    pub triangle_array: DeviceOrHostAddressConstKHR,
    pub triangle_array_stride: DeviceSize,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct MicromapCreateInfoEXT {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub create_flags: MicromapCreateFlagsEXT,
    pub buffer: Buffer,
    pub offset: DeviceSize,
    pub size: DeviceSize,
    pub ty: MicromapTypeEXT,
    pub device_address: DeviceAddress,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct MicromapVersionInfoEXT {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub p_version_data: *const u8,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct CopyMicromapInfoEXT {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub src: MicromapEXT,
    pub dst: MicromapEXT,
    pub mode: CopyMicromapModeEXT,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct CopyMicromapToMemoryInfoEXT {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub src: MicromapEXT,
    pub dst: DeviceOrHostAddressKHR,
    pub mode: CopyMicromapModeEXT,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct CopyMemoryToMicromapInfoEXT {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub src: DeviceOrHostAddressConstKHR,
    pub dst: MicromapEXT,
    pub mode: CopyMicromapModeEXT,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct MicromapBuildSizesInfoEXT {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub micromap_size: DeviceSize,
    pub build_scratch_size: DeviceSize,
    pub discardable: Bool32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct MicromapUsageEXT {
    pub count: u32,
    pub subdivision_level: u32,
    pub format: u32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct MicromapTriangleEXT {
    pub data_offset: u32,
    pub subdivision_level: u16,
    pub format: u16,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceOpacityMicromapFeaturesEXT {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub micromap: Bool32,
    pub micromap_capture_replay: Bool32,
    pub micromap_host_commands: Bool32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceOpacityMicromapPropertiesEXT {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub max_opacity2_state_subdivision_level: u32,
    pub max_opacity4_state_subdivision_level: u32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct AccelerationStructureTrianglesOpacityMicromapEXT {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub index_type: IndexType,
    pub index_buffer: DeviceOrHostAddressConstKHR,
    pub index_stride: DeviceSize,
    pub base_triangle: u32,
    pub usage_counts_count: u32,
    pub p_usage_counts: *const MicromapUsageEXT,
    pub pp_usage_counts: *const *const MicromapUsageEXT,
    pub micromap: MicromapEXT,
}
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct MicromapTypeEXT(i32);
impl MicromapTypeEXT {
    pub const OPACITY_MICROMAP_EXT: Self = Self(0);
    pub const DISPLACEMENT_MICROMAP_NV: Self = Self(1000397000);
}
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CopyMicromapModeEXT(i32);
impl CopyMicromapModeEXT {
    pub const CLONE_EXT: Self = Self(0);
    pub const SERIALIZE_EXT: Self = Self(1);
    pub const DESERIALIZE_EXT: Self = Self(2);
    pub const COMPACT_EXT: Self = Self(3);
}
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct BuildMicromapModeEXT(i32);
impl BuildMicromapModeEXT {
    pub const BUILD_EXT: Self = Self(0);
}
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct OpacityMicromapFormatEXT(i32);
impl OpacityMicromapFormatEXT {
    pub const _2_STATE_EXT: Self = Self(1);
    pub const _4_STATE_EXT: Self = Self(2);
}
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
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
    #[derive(Copy, Clone, PartialEq, Eq, Default)]
    pub struct BuildMicromapFlagsEXT: Flags {
        const PREFER_FAST_TRACE_EXT = BuildMicromapFlagBitsEXT::PREFER_FAST_TRACE_EXT.0;
        const PREFER_FAST_BUILD_EXT = BuildMicromapFlagBitsEXT::PREFER_FAST_BUILD_EXT.0;
        const ALLOW_COMPACTION_EXT = BuildMicromapFlagBitsEXT::ALLOW_COMPACTION_EXT.0;
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct BuildMicromapFlagBitsEXT(u32);
impl BuildMicromapFlagBitsEXT {
    pub const PREFER_FAST_TRACE_EXT: Self = Self(1 << 0);
    pub const PREFER_FAST_BUILD_EXT: Self = Self(1 << 1);
    pub const ALLOW_COMPACTION_EXT: Self = Self(1 << 2);
}
bitflags! {
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Default)]
    pub struct MicromapCreateFlagsEXT: Flags {
        const DEVICE_ADDRESS_CAPTURE_REPLAY_EXT = MicromapCreateFlagBitsEXT::DEVICE_ADDRESS_CAPTURE_REPLAY_EXT.0;
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct MicromapCreateFlagBitsEXT(u32);
impl MicromapCreateFlagBitsEXT {
    pub const DEVICE_ADDRESS_CAPTURE_REPLAY_EXT: Self = Self(1 << 0);
}
pub type PFN_vkCreateMicromapEXT = unsafe extern "system" fn(
    device: Device,
    p_create_info: *const MicromapCreateInfoEXT,
    p_allocator: *const AllocationCallbacks,
    p_micromap: *mut MicromapEXT,
) -> Result;
pub type PFN_vkCmdBuildMicromapsEXT = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    info_count: u32,
    p_infos: *const MicromapBuildInfoEXT,
);
pub type PFN_vkBuildMicromapsEXT = unsafe extern "system" fn(
    device: Device,
    deferred_operation: DeferredOperationKHR,
    info_count: u32,
    p_infos: *const MicromapBuildInfoEXT,
) -> Result;
pub type PFN_vkDestroyMicromapEXT = unsafe extern "system" fn(
    device: Device,
    micromap: MicromapEXT,
    p_allocator: *const AllocationCallbacks,
);
pub type PFN_vkCmdCopyMicromapEXT =
    unsafe extern "system" fn(command_buffer: CommandBuffer, p_info: *const CopyMicromapInfoEXT);
pub type PFN_vkCopyMicromapEXT = unsafe extern "system" fn(
    device: Device,
    deferred_operation: DeferredOperationKHR,
    p_info: *const CopyMicromapInfoEXT,
) -> Result;
pub type PFN_vkCmdCopyMicromapToMemoryEXT = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    p_info: *const CopyMicromapToMemoryInfoEXT,
);
pub type PFN_vkCopyMicromapToMemoryEXT = unsafe extern "system" fn(
    device: Device,
    deferred_operation: DeferredOperationKHR,
    p_info: *const CopyMicromapToMemoryInfoEXT,
) -> Result;
pub type PFN_vkCmdCopyMemoryToMicromapEXT = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    p_info: *const CopyMemoryToMicromapInfoEXT,
);
pub type PFN_vkCopyMemoryToMicromapEXT = unsafe extern "system" fn(
    device: Device,
    deferred_operation: DeferredOperationKHR,
    p_info: *const CopyMemoryToMicromapInfoEXT,
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
    p_version_info: *const MicromapVersionInfoEXT,
    p_compatibility: *mut AccelerationStructureCompatibilityKHR,
);
pub type PFN_vkGetMicromapBuildSizesEXT = unsafe extern "system" fn(
    device: Device,
    build_type: AccelerationStructureBuildTypeKHR,
    p_build_info: *const MicromapBuildInfoEXT,
    p_size_info: *mut MicromapBuildSizesInfoEXT,
);
