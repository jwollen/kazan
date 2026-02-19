#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use std::ffi::{c_char, c_int, c_void};
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
}
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct GeometryTypeKHR(i32);
impl GeometryTypeKHR {
    pub const TRIANGLES_KHR: Self = Self(0);
    pub const AABBS_KHR: Self = Self(1);
    pub const INSTANCES_KHR: Self = Self(2);
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
        const OPAQUE_KHR = 1 << 0;
        const NO_DUPLICATE_ANY_HIT_INVOCATION_KHR = 1 << 1;
    }
}
bitflags! {
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Default)]
    pub struct GeometryInstanceFlagsKHR: Flags {
        const TRIANGLE_FACING_CULL_DISABLE_KHR = 1 << 0;
        const TRIANGLE_FLIP_FACING_KHR = 1 << 1;
        const FORCE_OPAQUE_KHR = 1 << 2;
        const FORCE_NO_OPAQUE_KHR = 1 << 3;
    }
}
bitflags! {
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Default)]
    pub struct BuildAccelerationStructureFlagsKHR: Flags {
        const ALLOW_UPDATE_KHR = 1 << 0;
        const ALLOW_COMPACTION_KHR = 1 << 1;
        const PREFER_FAST_TRACE_KHR = 1 << 2;
        const PREFER_FAST_BUILD_KHR = 1 << 3;
        const LOW_MEMORY_KHR = 1 << 4;
    }
}
bitflags! {
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Default)]
    pub struct AccelerationStructureCreateFlagsKHR: Flags {
        const DEVICE_ADDRESS_CAPTURE_REPLAY_KHR = 1 << 0;
    }
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
