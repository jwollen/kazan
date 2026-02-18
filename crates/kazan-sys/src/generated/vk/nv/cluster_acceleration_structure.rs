#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use std::ffi::{c_char, c_int, c_void};
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceClusterAccelerationStructureFeaturesNV {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub cluster_acceleration_structure: Bool32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceClusterAccelerationStructurePropertiesNV {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub max_vertices_per_cluster: u32,
    pub max_triangles_per_cluster: u32,
    pub cluster_scratch_byte_alignment: u32,
    pub cluster_byte_alignment: u32,
    pub cluster_template_byte_alignment: u32,
    pub cluster_bottom_level_byte_alignment: u32,
    pub cluster_template_bounds_byte_alignment: u32,
    pub max_cluster_geometry_index: u32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct StridedDeviceAddressNV {
    pub start_address: DeviceAddress,
    pub stride_in_bytes: DeviceSize,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct RayTracingPipelineClusterAccelerationStructureCreateInfoNV {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub allow_cluster_acceleration_structure: Bool32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ClusterAccelerationStructureGeometryIndexAndGeometryFlagsNV {
    pub geometry_index: u32,
    pub reserved: u32,
    pub geometry_flags: u32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ClusterAccelerationStructureMoveObjectsInfoNV {
    pub src_acceleration_structure: DeviceAddress,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ClusterAccelerationStructureBuildClustersBottomLevelInfoNV {
    pub cluster_references_count: u32,
    pub cluster_references_stride: u32,
    pub cluster_references: DeviceAddress,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ClusterAccelerationStructureGetTemplateIndicesInfoNV {
    pub cluster_template_address: DeviceAddress,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ClusterAccelerationStructureBuildTriangleClusterInfoNV {
    pub cluster_id: u32,
    pub cluster_flags: ClusterAccelerationStructureClusterFlagsNV,
    pub triangle_count: u32,
    pub vertex_count: u32,
    pub position_truncate_bit_count: u32,
    pub index_type: u32,
    pub opacity_micromap_index_type: u32,
    pub base_geometry_index_and_geometry_flags:
        ClusterAccelerationStructureGeometryIndexAndGeometryFlagsNV,
    pub index_buffer_stride: u16,
    pub vertex_buffer_stride: u16,
    pub geometry_index_and_flags_buffer_stride: u16,
    pub opacity_micromap_index_buffer_stride: u16,
    pub index_buffer: DeviceAddress,
    pub vertex_buffer: DeviceAddress,
    pub geometry_index_and_flags_buffer: DeviceAddress,
    pub opacity_micromap_array: DeviceAddress,
    pub opacity_micromap_index_buffer: DeviceAddress,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ClusterAccelerationStructureBuildTriangleClusterTemplateInfoNV {
    pub cluster_id: u32,
    pub cluster_flags: ClusterAccelerationStructureClusterFlagsNV,
    pub triangle_count: u32,
    pub vertex_count: u32,
    pub position_truncate_bit_count: u32,
    pub index_type: u32,
    pub opacity_micromap_index_type: u32,
    pub base_geometry_index_and_geometry_flags:
        ClusterAccelerationStructureGeometryIndexAndGeometryFlagsNV,
    pub index_buffer_stride: u16,
    pub vertex_buffer_stride: u16,
    pub geometry_index_and_flags_buffer_stride: u16,
    pub opacity_micromap_index_buffer_stride: u16,
    pub index_buffer: DeviceAddress,
    pub vertex_buffer: DeviceAddress,
    pub geometry_index_and_flags_buffer: DeviceAddress,
    pub opacity_micromap_array: DeviceAddress,
    pub opacity_micromap_index_buffer: DeviceAddress,
    pub instantiation_bounding_box_limit: DeviceAddress,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ClusterAccelerationStructureInstantiateClusterInfoNV {
    pub cluster_id_offset: u32,
    pub geometry_index_offset: u32,
    pub reserved: u32,
    pub cluster_template_address: DeviceAddress,
    pub vertex_buffer: StridedDeviceAddressNV,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ClusterAccelerationStructureClustersBottomLevelInputNV {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub max_total_cluster_count: u32,
    pub max_cluster_count_per_acceleration_structure: u32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ClusterAccelerationStructureTriangleClusterInputNV {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub vertex_format: Format,
    pub max_geometry_index_value: u32,
    pub max_cluster_unique_geometry_count: u32,
    pub max_cluster_triangle_count: u32,
    pub max_cluster_vertex_count: u32,
    pub max_total_triangle_count: u32,
    pub max_total_vertex_count: u32,
    pub min_position_truncate_bit_count: u32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ClusterAccelerationStructureMoveObjectsInputNV {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub ty: ClusterAccelerationStructureTypeNV,
    pub no_move_overlap: Bool32,
    pub max_moved_bytes: DeviceSize,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ClusterAccelerationStructureInputInfoNV {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub max_acceleration_structure_count: u32,
    pub flags: BuildAccelerationStructureFlagsKHR,
    pub op_type: ClusterAccelerationStructureOpTypeNV,
    pub op_mode: ClusterAccelerationStructureOpModeNV,
    pub op_input: ClusterAccelerationStructureOpInputNV,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ClusterAccelerationStructureCommandsInfoNV {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub input: ClusterAccelerationStructureInputInfoNV,
    pub dst_implicit_data: DeviceAddress,
    pub scratch_data: DeviceAddress,
    pub dst_addresses_array: StridedDeviceAddressRegionKHR,
    pub dst_sizes_array: StridedDeviceAddressRegionKHR,
    pub src_infos_array: StridedDeviceAddressRegionKHR,
    pub src_infos_count: DeviceAddress,
    pub address_resolution_flags: ClusterAccelerationStructureAddressResolutionFlagsNV,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union ClusterAccelerationStructureOpInputNV {
    pub p_clusters_bottom_level: *mut ClusterAccelerationStructureClustersBottomLevelInputNV,
    pub p_triangle_clusters: *mut ClusterAccelerationStructureTriangleClusterInputNV,
    pub p_move_objects: *mut ClusterAccelerationStructureMoveObjectsInputNV,
}
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ClusterAccelerationStructureTypeNV(i32);
impl ClusterAccelerationStructureTypeNV {
    pub const CLUSTERS_BOTTOM_LEVEL_NV: Self = Self(0);
    pub const TRIANGLE_CLUSTER_NV: Self = Self(1);
    pub const TRIANGLE_CLUSTER_TEMPLATE_NV: Self = Self(2);
}
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ClusterAccelerationStructureOpTypeNV(i32);
impl ClusterAccelerationStructureOpTypeNV {
    pub const MOVE_OBJECTS_NV: Self = Self(0);
    pub const BUILD_CLUSTERS_BOTTOM_LEVEL_NV: Self = Self(1);
    pub const BUILD_TRIANGLE_CLUSTER_NV: Self = Self(2);
    pub const BUILD_TRIANGLE_CLUSTER_TEMPLATE_NV: Self = Self(3);
    pub const INSTANTIATE_TRIANGLE_CLUSTER_NV: Self = Self(4);
    pub const GET_CLUSTER_TEMPLATE_INDICES_NV: Self = Self(5);
}
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ClusterAccelerationStructureOpModeNV(i32);
impl ClusterAccelerationStructureOpModeNV {
    pub const IMPLICIT_DESTINATIONS_NV: Self = Self(0);
    pub const EXPLICIT_DESTINATIONS_NV: Self = Self(1);
    pub const COMPUTE_SIZES_NV: Self = Self(2);
}
bitflags! {
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Default)]
    pub struct ClusterAccelerationStructureGeometryFlagsNV: Flags {
    }
}
bitflags! {
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Default)]
    pub struct ClusterAccelerationStructureClusterFlagsNV: Flags {
    }
}
bitflags! {
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Default)]
    pub struct ClusterAccelerationStructureAddressResolutionFlagsNV: Flags {
    }
}
bitflags! {
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Default)]
    pub struct ClusterAccelerationStructureIndexFormatFlagsNV: Flags {
    }
}
pub type PFN_vkGetClusterAccelerationStructureBuildSizesNV = unsafe extern "system" fn(
    device: Device,
    p_info: *const ClusterAccelerationStructureInputInfoNV,
    p_size_info: *mut AccelerationStructureBuildSizesInfoKHR,
);
pub type PFN_vkCmdBuildClusterAccelerationStructureIndirectNV = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    p_command_infos: *const ClusterAccelerationStructureCommandsInfoNV,
);
