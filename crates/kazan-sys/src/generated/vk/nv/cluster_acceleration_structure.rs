#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{c_char, c_int, c_void};
use core::marker::PhantomData;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceClusterAccelerationStructureFeaturesNV<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub cluster_acceleration_structure: Bool32,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for PhysicalDeviceClusterAccelerationStructureFeaturesNV<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::PHYSICAL_DEVICE_CLUSTER_ACCELERATION_STRUCTURE_FEATURES_NV,
            p_next: core::ptr::null_mut(),
            cluster_acceleration_structure: Default::default(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceClusterAccelerationStructurePropertiesNV<'a> {
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
    pub _marker: PhantomData<&'a ()>,
}
impl Default for PhysicalDeviceClusterAccelerationStructurePropertiesNV<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::PHYSICAL_DEVICE_CLUSTER_ACCELERATION_STRUCTURE_PROPERTIES_NV,
            p_next: core::ptr::null_mut(),
            max_vertices_per_cluster: Default::default(),
            max_triangles_per_cluster: Default::default(),
            cluster_scratch_byte_alignment: Default::default(),
            cluster_byte_alignment: Default::default(),
            cluster_template_byte_alignment: Default::default(),
            cluster_bottom_level_byte_alignment: Default::default(),
            cluster_template_bounds_byte_alignment: Default::default(),
            max_cluster_geometry_index: Default::default(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct StridedDeviceAddressNV {
    pub start_address: DeviceAddress,
    pub stride_in_bytes: DeviceSize,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct RayTracingPipelineClusterAccelerationStructureCreateInfoNV<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub allow_cluster_acceleration_structure: Bool32,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for RayTracingPipelineClusterAccelerationStructureCreateInfoNV<'_> {
    fn default() -> Self {
        Self {
            s_type:
                StructureType::RAY_TRACING_PIPELINE_CLUSTER_ACCELERATION_STRUCTURE_CREATE_INFO_NV,
            p_next: core::ptr::null_mut(),
            allow_cluster_acceleration_structure: Default::default(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct ClusterAccelerationStructureGeometryIndexAndGeometryFlagsNV {
    pub geometry_index: u32,
    pub reserved: u32,
    pub geometry_flags: u32,
}
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct ClusterAccelerationStructureMoveObjectsInfoNV {
    pub src_acceleration_structure: DeviceAddress,
}
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct ClusterAccelerationStructureBuildClustersBottomLevelInfoNV {
    pub cluster_references_count: u32,
    pub cluster_references_stride: u32,
    pub cluster_references: DeviceAddress,
}
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct ClusterAccelerationStructureGetTemplateIndicesInfoNV {
    pub cluster_template_address: DeviceAddress,
}
#[repr(C)]
#[derive(Copy, Clone, Default)]
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
#[derive(Copy, Clone, Default)]
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
#[derive(Copy, Clone, Default)]
pub struct ClusterAccelerationStructureInstantiateClusterInfoNV {
    pub cluster_id_offset: u32,
    pub geometry_index_offset: u32,
    pub reserved: u32,
    pub cluster_template_address: DeviceAddress,
    pub vertex_buffer: StridedDeviceAddressNV,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ClusterAccelerationStructureClustersBottomLevelInputNV<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub max_total_cluster_count: u32,
    pub max_cluster_count_per_acceleration_structure: u32,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for ClusterAccelerationStructureClustersBottomLevelInputNV<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::CLUSTER_ACCELERATION_STRUCTURE_CLUSTERS_BOTTOM_LEVEL_INPUT_NV,
            p_next: core::ptr::null_mut(),
            max_total_cluster_count: Default::default(),
            max_cluster_count_per_acceleration_structure: Default::default(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ClusterAccelerationStructureTriangleClusterInputNV<'a> {
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
    pub _marker: PhantomData<&'a ()>,
}
impl Default for ClusterAccelerationStructureTriangleClusterInputNV<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::CLUSTER_ACCELERATION_STRUCTURE_TRIANGLE_CLUSTER_INPUT_NV,
            p_next: core::ptr::null_mut(),
            vertex_format: Default::default(),
            max_geometry_index_value: Default::default(),
            max_cluster_unique_geometry_count: Default::default(),
            max_cluster_triangle_count: Default::default(),
            max_cluster_vertex_count: Default::default(),
            max_total_triangle_count: Default::default(),
            max_total_vertex_count: Default::default(),
            min_position_truncate_bit_count: Default::default(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ClusterAccelerationStructureMoveObjectsInputNV<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub ty: ClusterAccelerationStructureTypeNV,
    pub no_move_overlap: Bool32,
    pub max_moved_bytes: DeviceSize,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for ClusterAccelerationStructureMoveObjectsInputNV<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::CLUSTER_ACCELERATION_STRUCTURE_MOVE_OBJECTS_INPUT_NV,
            p_next: core::ptr::null_mut(),
            ty: Default::default(),
            no_move_overlap: Default::default(),
            max_moved_bytes: Default::default(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ClusterAccelerationStructureInputInfoNV<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub max_acceleration_structure_count: u32,
    pub flags: BuildAccelerationStructureFlagsKHR,
    pub op_type: ClusterAccelerationStructureOpTypeNV,
    pub op_mode: ClusterAccelerationStructureOpModeNV,
    pub op_input: ClusterAccelerationStructureOpInputNV<'a>,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for ClusterAccelerationStructureInputInfoNV<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::CLUSTER_ACCELERATION_STRUCTURE_INPUT_INFO_NV,
            p_next: core::ptr::null_mut(),
            max_acceleration_structure_count: Default::default(),
            flags: Default::default(),
            op_type: Default::default(),
            op_mode: Default::default(),
            op_input: Default::default(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ClusterAccelerationStructureCommandsInfoNV<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub input: ClusterAccelerationStructureInputInfoNV<'a>,
    pub dst_implicit_data: DeviceAddress,
    pub scratch_data: DeviceAddress,
    pub dst_addresses_array: StridedDeviceAddressRegionKHR,
    pub dst_sizes_array: StridedDeviceAddressRegionKHR,
    pub src_infos_array: StridedDeviceAddressRegionKHR,
    pub src_infos_count: DeviceAddress,
    pub address_resolution_flags: ClusterAccelerationStructureAddressResolutionFlagsNV,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for ClusterAccelerationStructureCommandsInfoNV<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::CLUSTER_ACCELERATION_STRUCTURE_COMMANDS_INFO_NV,
            p_next: core::ptr::null_mut(),
            input: Default::default(),
            dst_implicit_data: Default::default(),
            scratch_data: Default::default(),
            dst_addresses_array: Default::default(),
            dst_sizes_array: Default::default(),
            src_infos_array: Default::default(),
            src_infos_count: Default::default(),
            address_resolution_flags: Default::default(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union ClusterAccelerationStructureOpInputNV<'a> {
    pub p_clusters_bottom_level: *mut ClusterAccelerationStructureClustersBottomLevelInputNV<'a>,
    pub p_triangle_clusters: *mut ClusterAccelerationStructureTriangleClusterInputNV<'a>,
    pub p_move_objects: *mut ClusterAccelerationStructureMoveObjectsInputNV<'a>,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for ClusterAccelerationStructureOpInputNV<'_> {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ClusterAccelerationStructureTypeNV(i32);
impl ClusterAccelerationStructureTypeNV {
    pub const CLUSTERS_BOTTOM_LEVEL_NV: Self = Self(0);
    pub const TRIANGLE_CLUSTER_NV: Self = Self(1);
    pub const TRIANGLE_CLUSTER_TEMPLATE_NV: Self = Self(2);
}
#[repr(transparent)]
#[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
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
#[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ClusterAccelerationStructureOpModeNV(i32);
impl ClusterAccelerationStructureOpModeNV {
    pub const IMPLICIT_DESTINATIONS_NV: Self = Self(0);
    pub const EXPLICIT_DESTINATIONS_NV: Self = Self(1);
    pub const COMPUTE_SIZES_NV: Self = Self(2);
}
bitflags! {
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct ClusterAccelerationStructureGeometryFlagsNV: Flags {
        const CULL_DISABLE_NV = ClusterAccelerationStructureGeometryFlagBitsNV::CULL_DISABLE_NV.0;
        const NO_DUPLICATE_ANYHIT_INVOCATION_NV = ClusterAccelerationStructureGeometryFlagBitsNV::NO_DUPLICATE_ANYHIT_INVOCATION_NV.0;
        const OPAQUE_NV = ClusterAccelerationStructureGeometryFlagBitsNV::OPAQUE_NV.0;
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ClusterAccelerationStructureGeometryFlagBitsNV(u32);
impl ClusterAccelerationStructureGeometryFlagBitsNV {
    pub const CULL_DISABLE_NV: Self = Self(1 << 0);
    pub const NO_DUPLICATE_ANYHIT_INVOCATION_NV: Self = Self(1 << 1);
    pub const OPAQUE_NV: Self = Self(1 << 2);
}
bitflags! {
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct ClusterAccelerationStructureClusterFlagsNV: Flags {
        const ALLOW_DISABLE_OPACITY_MICROMAPS_NV = ClusterAccelerationStructureClusterFlagBitsNV::ALLOW_DISABLE_OPACITY_MICROMAPS_NV.0;
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ClusterAccelerationStructureClusterFlagBitsNV(u32);
impl ClusterAccelerationStructureClusterFlagBitsNV {
    pub const ALLOW_DISABLE_OPACITY_MICROMAPS_NV: Self = Self(1 << 0);
}
bitflags! {
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct ClusterAccelerationStructureAddressResolutionFlagsNV: Flags {
        const INDIRECTED_DST_IMPLICIT_DATA_NV = ClusterAccelerationStructureAddressResolutionFlagBitsNV::INDIRECTED_DST_IMPLICIT_DATA_NV.0;
        const INDIRECTED_SCRATCH_DATA_NV = ClusterAccelerationStructureAddressResolutionFlagBitsNV::INDIRECTED_SCRATCH_DATA_NV.0;
        const INDIRECTED_DST_ADDRESS_ARRAY_NV = ClusterAccelerationStructureAddressResolutionFlagBitsNV::INDIRECTED_DST_ADDRESS_ARRAY_NV.0;
        const INDIRECTED_DST_SIZES_ARRAY_NV = ClusterAccelerationStructureAddressResolutionFlagBitsNV::INDIRECTED_DST_SIZES_ARRAY_NV.0;
        const INDIRECTED_SRC_INFOS_ARRAY_NV = ClusterAccelerationStructureAddressResolutionFlagBitsNV::INDIRECTED_SRC_INFOS_ARRAY_NV.0;
        const INDIRECTED_SRC_INFOS_COUNT_NV = ClusterAccelerationStructureAddressResolutionFlagBitsNV::INDIRECTED_SRC_INFOS_COUNT_NV.0;
        const NONE = 0;
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ClusterAccelerationStructureAddressResolutionFlagBitsNV(u32);
impl ClusterAccelerationStructureAddressResolutionFlagBitsNV {
    pub const INDIRECTED_DST_IMPLICIT_DATA_NV: Self = Self(1 << 0);
    pub const INDIRECTED_SCRATCH_DATA_NV: Self = Self(1 << 1);
    pub const INDIRECTED_DST_ADDRESS_ARRAY_NV: Self = Self(1 << 2);
    pub const INDIRECTED_DST_SIZES_ARRAY_NV: Self = Self(1 << 3);
    pub const INDIRECTED_SRC_INFOS_ARRAY_NV: Self = Self(1 << 4);
    pub const INDIRECTED_SRC_INFOS_COUNT_NV: Self = Self(1 << 5);
}
bitflags! {
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct ClusterAccelerationStructureIndexFormatFlagsNV: Flags {
        const _8BIT_NV = ClusterAccelerationStructureIndexFormatFlagBitsNV::_8BIT_NV.0;
        const _16BIT_NV = ClusterAccelerationStructureIndexFormatFlagBitsNV::_16BIT_NV.0;
        const _32BIT_NV = ClusterAccelerationStructureIndexFormatFlagBitsNV::_32BIT_NV.0;
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ClusterAccelerationStructureIndexFormatFlagBitsNV(u32);
impl ClusterAccelerationStructureIndexFormatFlagBitsNV {
    pub const _8BIT_NV: Self = Self(1 << 0);
    pub const _16BIT_NV: Self = Self(1 << 1);
    pub const _32BIT_NV: Self = Self(1 << 2);
}
pub type PFN_vkGetClusterAccelerationStructureBuildSizesNV = unsafe extern "system" fn(
    device: Device,
    p_info: *const ClusterAccelerationStructureInputInfoNV<'_>,
    p_size_info: *mut AccelerationStructureBuildSizesInfoKHR<'_>,
);
pub type PFN_vkCmdBuildClusterAccelerationStructureIndirectNV = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    p_command_infos: *const ClusterAccelerationStructureCommandsInfoNV<'_>,
);
