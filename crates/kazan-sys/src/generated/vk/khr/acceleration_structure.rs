#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{c_char, c_int, c_void};
#[repr(C)]
#[derive(Copy, Clone)]
pub struct AccelerationStructureKHR(u64);
#[repr(C)]
#[derive(Copy, Clone)]
pub struct WriteDescriptorSetAccelerationStructureKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub acceleration_structure_count: u32,
    pub p_acceleration_structures: *const AccelerationStructureKHR,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceAccelerationStructureFeaturesKHR {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub acceleration_structure: Bool32,
    pub acceleration_structure_capture_replay: Bool32,
    pub acceleration_structure_indirect_build: Bool32,
    pub acceleration_structure_host_commands: Bool32,
    pub descriptor_binding_acceleration_structure_update_after_bind: Bool32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceAccelerationStructurePropertiesKHR {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub max_geometry_count: u64,
    pub max_instance_count: u64,
    pub max_primitive_count: u64,
    pub max_per_stage_descriptor_acceleration_structures: u32,
    pub max_per_stage_descriptor_update_after_bind_acceleration_structures: u32,
    pub max_descriptor_set_acceleration_structures: u32,
    pub max_descriptor_set_update_after_bind_acceleration_structures: u32,
    pub min_acceleration_structure_scratch_offset_alignment: u32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct AccelerationStructureGeometryTrianglesDataKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub vertex_format: Format,
    pub vertex_data: DeviceOrHostAddressConstKHR,
    pub vertex_stride: DeviceSize,
    pub max_vertex: u32,
    pub index_type: IndexType,
    pub index_data: DeviceOrHostAddressConstKHR,
    pub transform_data: DeviceOrHostAddressConstKHR,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct AccelerationStructureGeometryAabbsDataKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub data: DeviceOrHostAddressConstKHR,
    pub stride: DeviceSize,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct AccelerationStructureGeometryInstancesDataKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub array_of_pointers: Bool32,
    pub data: DeviceOrHostAddressConstKHR,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct AccelerationStructureGeometryKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub geometry_type: GeometryTypeKHR,
    pub geometry: AccelerationStructureGeometryDataKHR,
    pub flags: GeometryFlagsKHR,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct AccelerationStructureBuildGeometryInfoKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub ty: AccelerationStructureTypeKHR,
    pub flags: BuildAccelerationStructureFlagsKHR,
    pub mode: BuildAccelerationStructureModeKHR,
    pub src_acceleration_structure: AccelerationStructureKHR,
    pub dst_acceleration_structure: AccelerationStructureKHR,
    pub geometry_count: u32,
    pub p_geometries: *const AccelerationStructureGeometryKHR,
    pub pp_geometries: *const *const AccelerationStructureGeometryKHR,
    pub scratch_data: DeviceOrHostAddressKHR,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct AccelerationStructureBuildRangeInfoKHR {
    pub primitive_count: u32,
    pub primitive_offset: u32,
    pub first_vertex: u32,
    pub transform_offset: u32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct AccelerationStructureCreateInfoKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub create_flags: AccelerationStructureCreateFlagsKHR,
    pub buffer: Buffer,
    pub offset: DeviceSize,
    pub size: DeviceSize,
    pub ty: AccelerationStructureTypeKHR,
    pub device_address: DeviceAddress,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct AabbPositionsKHR {
    pub min_x: f32,
    pub min_y: f32,
    pub min_z: f32,
    pub max_x: f32,
    pub max_y: f32,
    pub max_z: f32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct TransformMatrixKHR {
    pub matrix: [[f32; 4]; 3],
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct AccelerationStructureInstanceKHR {
    pub transform: TransformMatrixKHR,
    pub instance_custom_index: u32,
    pub mask: u32,
    pub instance_shader_binding_table_record_offset: u32,
    pub flags: GeometryInstanceFlagsKHR,
    pub acceleration_structure_reference: u64,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct AccelerationStructureDeviceAddressInfoKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub acceleration_structure: AccelerationStructureKHR,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct AccelerationStructureVersionInfoKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub p_version_data: *const u8,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct CopyAccelerationStructureInfoKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub src: AccelerationStructureKHR,
    pub dst: AccelerationStructureKHR,
    pub mode: CopyAccelerationStructureModeKHR,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct CopyAccelerationStructureToMemoryInfoKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub src: AccelerationStructureKHR,
    pub dst: DeviceOrHostAddressKHR,
    pub mode: CopyAccelerationStructureModeKHR,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct CopyMemoryToAccelerationStructureInfoKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub src: DeviceOrHostAddressConstKHR,
    pub dst: AccelerationStructureKHR,
    pub mode: CopyAccelerationStructureModeKHR,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct AccelerationStructureBuildSizesInfoKHR {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub acceleration_structure_size: DeviceSize,
    pub update_scratch_size: DeviceSize,
    pub build_scratch_size: DeviceSize,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union DeviceOrHostAddressKHR {
    pub device_address: DeviceAddress,
    pub host_address: *mut c_void,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union DeviceOrHostAddressConstKHR {
    pub device_address: DeviceAddress,
    pub host_address: *const c_void,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union AccelerationStructureGeometryDataKHR {
    pub triangles: AccelerationStructureGeometryTrianglesDataKHR,
    pub aabbs: AccelerationStructureGeometryAabbsDataKHR,
    pub instances: AccelerationStructureGeometryInstancesDataKHR,
}
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CopyAccelerationStructureModeKHR(i32);
impl CopyAccelerationStructureModeKHR {
    pub const CLONE_KHR: Self = Self(0);
    pub const COMPACT_KHR: Self = Self(1);
    pub const DESERIALIZE_KHR: Self = Self(3);
    pub const SERIALIZE_KHR: Self = Self(2);
    pub const CLONE_NV: Self = Self::CLONE_KHR;
    pub const COMPACT_NV: Self = Self::COMPACT_KHR;
}
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct BuildAccelerationStructureModeKHR(i32);
impl BuildAccelerationStructureModeKHR {
    pub const BUILD_KHR: Self = Self(0);
    pub const UPDATE_KHR: Self = Self(1);
}
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct AccelerationStructureTypeKHR(i32);
impl AccelerationStructureTypeKHR {
    pub const TOP_LEVEL_KHR: Self = Self(0);
    pub const BOTTOM_LEVEL_KHR: Self = Self(1);
    pub const GENERIC_KHR: Self = Self(2);
    pub const BOTTOM_LEVEL_NV: Self = Self::BOTTOM_LEVEL_KHR;
    pub const TOP_LEVEL_NV: Self = Self::TOP_LEVEL_KHR;
}
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct GeometryTypeKHR(i32);
impl GeometryTypeKHR {
    pub const TRIANGLES_KHR: Self = Self(0);
    pub const AABBS_KHR: Self = Self(1);
    pub const INSTANCES_KHR: Self = Self(2);
    pub const DENSE_GEOMETRY_FORMAT_TRIANGLES_AMDX: Self = Self(1000478000);
    pub const LINEAR_SWEPT_SPHERES_NV: Self = Self(1000429005);
    pub const SPHERES_NV: Self = Self(1000429004);
    pub const AABBS_NV: Self = Self::AABBS_KHR;
    pub const TRIANGLES_NV: Self = Self::TRIANGLES_KHR;
}
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct AccelerationStructureBuildTypeKHR(i32);
impl AccelerationStructureBuildTypeKHR {
    pub const HOST_KHR: Self = Self(0);
    pub const DEVICE_KHR: Self = Self(1);
    pub const HOST_OR_DEVICE_KHR: Self = Self(2);
}
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct AccelerationStructureCompatibilityKHR(i32);
impl AccelerationStructureCompatibilityKHR {
    pub const COMPATIBLE_KHR: Self = Self(0);
    pub const INCOMPATIBLE_KHR: Self = Self(1);
}
bitflags! {
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Default)]
    pub struct GeometryFlagsKHR: Flags {
        const OPAQUE_KHR = GeometryFlagBitsKHR::OPAQUE_KHR.0;
        const NO_DUPLICATE_ANY_HIT_INVOCATION_KHR = GeometryFlagBitsKHR::NO_DUPLICATE_ANY_HIT_INVOCATION_KHR.0;
        const NO_DUPLICATE_ANY_HIT_INVOCATION_NV = Self::NO_DUPLICATE_ANY_HIT_INVOCATION_KHR.bits();
        const OPAQUE_NV = Self::OPAQUE_KHR.bits();
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct GeometryFlagBitsKHR(u32);
impl GeometryFlagBitsKHR {
    pub const OPAQUE_KHR: Self = Self(1 << 0);
    pub const NO_DUPLICATE_ANY_HIT_INVOCATION_KHR: Self = Self(1 << 1);
    pub const NO_DUPLICATE_ANY_HIT_INVOCATION_NV: Self = Self::NO_DUPLICATE_ANY_HIT_INVOCATION_KHR;
    pub const OPAQUE_NV: Self = Self::OPAQUE_KHR;
}
bitflags! {
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Default)]
    pub struct GeometryInstanceFlagsKHR: Flags {
        const TRIANGLE_FACING_CULL_DISABLE_KHR = GeometryInstanceFlagBitsKHR::TRIANGLE_FACING_CULL_DISABLE_KHR.0;
        const TRIANGLE_FLIP_FACING_KHR = GeometryInstanceFlagBitsKHR::TRIANGLE_FLIP_FACING_KHR.0;
        const FORCE_OPAQUE_KHR = GeometryInstanceFlagBitsKHR::FORCE_OPAQUE_KHR.0;
        const FORCE_NO_OPAQUE_KHR = GeometryInstanceFlagBitsKHR::FORCE_NO_OPAQUE_KHR.0;
        const FORCE_OPACITY_MICROMAP_2_STATE_EXT = GeometryInstanceFlagBitsKHR::FORCE_OPACITY_MICROMAP_2_STATE_EXT.0;
        const DISABLE_OPACITY_MICROMAPS_EXT = GeometryInstanceFlagBitsKHR::DISABLE_OPACITY_MICROMAPS_EXT.0;
        const TRIANGLE_FRONT_COUNTERCLOCKWISE_KHR = Self::TRIANGLE_FLIP_FACING_KHR.bits();
        const FORCE_NO_OPAQUE_NV = Self::FORCE_NO_OPAQUE_KHR.bits();
        const FORCE_OPAQUE_NV = Self::FORCE_OPAQUE_KHR.bits();
        const TRIANGLE_CULL_DISABLE_NV = Self::TRIANGLE_FACING_CULL_DISABLE_KHR.bits();
        const TRIANGLE_FRONT_COUNTERCLOCKWISE_NV = Self::TRIANGLE_FRONT_COUNTERCLOCKWISE_KHR.bits();
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct GeometryInstanceFlagBitsKHR(u32);
impl GeometryInstanceFlagBitsKHR {
    pub const TRIANGLE_FACING_CULL_DISABLE_KHR: Self = Self(1 << 0);
    pub const TRIANGLE_FLIP_FACING_KHR: Self = Self(1 << 1);
    pub const FORCE_OPAQUE_KHR: Self = Self(1 << 2);
    pub const FORCE_NO_OPAQUE_KHR: Self = Self(1 << 3);
    pub const FORCE_OPACITY_MICROMAP_2_STATE_EXT: Self = Self(1 << 4);
    pub const DISABLE_OPACITY_MICROMAPS_EXT: Self = Self(1 << 5);
    pub const FORCE_NO_OPAQUE_NV: Self = Self::FORCE_NO_OPAQUE_KHR;
    pub const FORCE_OPAQUE_NV: Self = Self::FORCE_OPAQUE_KHR;
    pub const TRIANGLE_CULL_DISABLE_NV: Self = Self::TRIANGLE_FACING_CULL_DISABLE_KHR;
}
bitflags! {
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Default)]
    pub struct BuildAccelerationStructureFlagsKHR: Flags {
        const ALLOW_UPDATE_KHR = BuildAccelerationStructureFlagBitsKHR::ALLOW_UPDATE_KHR.0;
        const ALLOW_COMPACTION_KHR = BuildAccelerationStructureFlagBitsKHR::ALLOW_COMPACTION_KHR.0;
        const PREFER_FAST_TRACE_KHR = BuildAccelerationStructureFlagBitsKHR::PREFER_FAST_TRACE_KHR.0;
        const PREFER_FAST_BUILD_KHR = BuildAccelerationStructureFlagBitsKHR::PREFER_FAST_BUILD_KHR.0;
        const LOW_MEMORY_KHR = BuildAccelerationStructureFlagBitsKHR::LOW_MEMORY_KHR.0;
        const MOTION_NV = BuildAccelerationStructureFlagBitsKHR::MOTION_NV.0;
        const ALLOW_OPACITY_MICROMAP_UPDATE_EXT = BuildAccelerationStructureFlagBitsKHR::ALLOW_OPACITY_MICROMAP_UPDATE_EXT.0;
        const ALLOW_DISABLE_OPACITY_MICROMAPS_EXT = BuildAccelerationStructureFlagBitsKHR::ALLOW_DISABLE_OPACITY_MICROMAPS_EXT.0;
        const ALLOW_OPACITY_MICROMAP_DATA_UPDATE_EXT = BuildAccelerationStructureFlagBitsKHR::ALLOW_OPACITY_MICROMAP_DATA_UPDATE_EXT.0;
        const ALLOW_DISPLACEMENT_MICROMAP_UPDATE_NV = BuildAccelerationStructureFlagBitsKHR::ALLOW_DISPLACEMENT_MICROMAP_UPDATE_NV.0;
        const ALLOW_DATA_ACCESS_KHR = BuildAccelerationStructureFlagBitsKHR::ALLOW_DATA_ACCESS_KHR.0;
        const ALLOW_CLUSTER_OPACITY_MICROMAPS_NV = BuildAccelerationStructureFlagBitsKHR::ALLOW_CLUSTER_OPACITY_MICROMAPS_NV.0;
        const ALLOW_COMPACTION_NV = Self::ALLOW_COMPACTION_KHR.bits();
        const ALLOW_UPDATE_NV = Self::ALLOW_UPDATE_KHR.bits();
        const LOW_MEMORY_NV = Self::LOW_MEMORY_KHR.bits();
        const PREFER_FAST_BUILD_NV = Self::PREFER_FAST_BUILD_KHR.bits();
        const PREFER_FAST_TRACE_NV = Self::PREFER_FAST_TRACE_KHR.bits();
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct BuildAccelerationStructureFlagBitsKHR(u32);
impl BuildAccelerationStructureFlagBitsKHR {
    pub const ALLOW_UPDATE_KHR: Self = Self(1 << 0);
    pub const ALLOW_COMPACTION_KHR: Self = Self(1 << 1);
    pub const PREFER_FAST_TRACE_KHR: Self = Self(1 << 2);
    pub const PREFER_FAST_BUILD_KHR: Self = Self(1 << 3);
    pub const LOW_MEMORY_KHR: Self = Self(1 << 4);
    pub const MOTION_NV: Self = Self(1 << 5);
    pub const ALLOW_OPACITY_MICROMAP_UPDATE_EXT: Self = Self(1 << 6);
    pub const ALLOW_DISABLE_OPACITY_MICROMAPS_EXT: Self = Self(1 << 7);
    pub const ALLOW_OPACITY_MICROMAP_DATA_UPDATE_EXT: Self = Self(1 << 8);
    pub const ALLOW_DISPLACEMENT_MICROMAP_UPDATE_NV: Self = Self(1 << 9);
    pub const ALLOW_DATA_ACCESS_KHR: Self = Self(1 << 11);
    pub const ALLOW_CLUSTER_OPACITY_MICROMAPS_NV: Self = Self(1 << 12);
    pub const ALLOW_COMPACTION_NV: Self = Self::ALLOW_COMPACTION_KHR;
    pub const ALLOW_UPDATE_NV: Self = Self::ALLOW_UPDATE_KHR;
    pub const LOW_MEMORY_NV: Self = Self::LOW_MEMORY_KHR;
    pub const PREFER_FAST_BUILD_NV: Self = Self::PREFER_FAST_BUILD_KHR;
    pub const PREFER_FAST_TRACE_NV: Self = Self::PREFER_FAST_TRACE_KHR;
}
bitflags! {
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Default)]
    pub struct AccelerationStructureCreateFlagsKHR: Flags {
        const DEVICE_ADDRESS_CAPTURE_REPLAY_KHR = AccelerationStructureCreateFlagBitsKHR::DEVICE_ADDRESS_CAPTURE_REPLAY_KHR.0;
        const MOTION_NV = AccelerationStructureCreateFlagBitsKHR::MOTION_NV.0;
        const DESCRIPTOR_BUFFER_CAPTURE_REPLAY_EXT = AccelerationStructureCreateFlagBitsKHR::DESCRIPTOR_BUFFER_CAPTURE_REPLAY_EXT.0;
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct AccelerationStructureCreateFlagBitsKHR(u32);
impl AccelerationStructureCreateFlagBitsKHR {
    pub const DEVICE_ADDRESS_CAPTURE_REPLAY_KHR: Self = Self(1 << 0);
    pub const MOTION_NV: Self = Self(1 << 2);
    pub const DESCRIPTOR_BUFFER_CAPTURE_REPLAY_EXT: Self = Self(1 << 3);
}
pub type PFN_vkDestroyAccelerationStructureKHR = unsafe extern "system" fn(
    device: Device,
    acceleration_structure: AccelerationStructureKHR,
    p_allocator: *const AllocationCallbacks,
);
pub type PFN_vkCmdCopyAccelerationStructureKHR = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    p_info: *const CopyAccelerationStructureInfoKHR,
);
pub type PFN_vkCopyAccelerationStructureKHR = unsafe extern "system" fn(
    device: Device,
    deferred_operation: DeferredOperationKHR,
    p_info: *const CopyAccelerationStructureInfoKHR,
) -> Result;
pub type PFN_vkCmdCopyAccelerationStructureToMemoryKHR = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    p_info: *const CopyAccelerationStructureToMemoryInfoKHR,
);
pub type PFN_vkCopyAccelerationStructureToMemoryKHR = unsafe extern "system" fn(
    device: Device,
    deferred_operation: DeferredOperationKHR,
    p_info: *const CopyAccelerationStructureToMemoryInfoKHR,
) -> Result;
pub type PFN_vkCmdCopyMemoryToAccelerationStructureKHR = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    p_info: *const CopyMemoryToAccelerationStructureInfoKHR,
);
pub type PFN_vkCopyMemoryToAccelerationStructureKHR = unsafe extern "system" fn(
    device: Device,
    deferred_operation: DeferredOperationKHR,
    p_info: *const CopyMemoryToAccelerationStructureInfoKHR,
) -> Result;
pub type PFN_vkCmdWriteAccelerationStructuresPropertiesKHR = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    acceleration_structure_count: u32,
    p_acceleration_structures: *const AccelerationStructureKHR,
    query_type: QueryType,
    query_pool: QueryPool,
    first_query: u32,
);
pub type PFN_vkWriteAccelerationStructuresPropertiesKHR = unsafe extern "system" fn(
    device: Device,
    acceleration_structure_count: u32,
    p_acceleration_structures: *const AccelerationStructureKHR,
    query_type: QueryType,
    data_size: usize,
    p_data: *mut c_void,
    stride: usize,
) -> Result;
pub type PFN_vkGetDeviceAccelerationStructureCompatibilityKHR = unsafe extern "system" fn(
    device: Device,
    p_version_info: *const AccelerationStructureVersionInfoKHR,
    p_compatibility: *mut AccelerationStructureCompatibilityKHR,
);
pub type PFN_vkCreateAccelerationStructureKHR = unsafe extern "system" fn(
    device: Device,
    p_create_info: *const AccelerationStructureCreateInfoKHR,
    p_allocator: *const AllocationCallbacks,
    p_acceleration_structure: *mut AccelerationStructureKHR,
) -> Result;
pub type PFN_vkCmdBuildAccelerationStructuresKHR = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    info_count: u32,
    p_infos: *const AccelerationStructureBuildGeometryInfoKHR,
    pp_build_range_infos: *const *const AccelerationStructureBuildRangeInfoKHR,
);
pub type PFN_vkCmdBuildAccelerationStructuresIndirectKHR = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    info_count: u32,
    p_infos: *const AccelerationStructureBuildGeometryInfoKHR,
    p_indirect_device_addresses: *const DeviceAddress,
    p_indirect_strides: *const u32,
    pp_max_primitive_counts: *const *const u32,
);
pub type PFN_vkBuildAccelerationStructuresKHR = unsafe extern "system" fn(
    device: Device,
    deferred_operation: DeferredOperationKHR,
    info_count: u32,
    p_infos: *const AccelerationStructureBuildGeometryInfoKHR,
    pp_build_range_infos: *const *const AccelerationStructureBuildRangeInfoKHR,
) -> Result;
pub type PFN_vkGetAccelerationStructureDeviceAddressKHR =
    unsafe extern "system" fn(
        device: Device,
        p_info: *const AccelerationStructureDeviceAddressInfoKHR,
    ) -> DeviceAddress;
pub type PFN_vkGetAccelerationStructureBuildSizesKHR = unsafe extern "system" fn(
    device: Device,
    build_type: AccelerationStructureBuildTypeKHR,
    p_build_info: *const AccelerationStructureBuildGeometryInfoKHR,
    p_max_primitive_counts: *const u32,
    p_size_info: *mut AccelerationStructureBuildSizesInfoKHR,
);
