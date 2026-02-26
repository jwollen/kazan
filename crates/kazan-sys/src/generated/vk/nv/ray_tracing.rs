#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{c_char, c_int, c_void};
use core::marker::PhantomData;
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct AccelerationStructureNV(u64);
pub type CopyAccelerationStructureModeNV = CopyAccelerationStructureModeKHR;
pub type AccelerationStructureTypeNV = AccelerationStructureTypeKHR;
pub type GeometryTypeNV = GeometryTypeKHR;
pub type RayTracingShaderGroupTypeNV = RayTracingShaderGroupTypeKHR;
pub type AabbPositionsNV = AabbPositionsKHR;
pub type TransformMatrixNV = TransformMatrixKHR;
pub type AccelerationStructureInstanceNV = AccelerationStructureInstanceKHR;
pub type GeometryFlagsNV = GeometryFlagsKHR;
pub type GeometryInstanceFlagsNV = GeometryInstanceFlagsKHR;
pub type BuildAccelerationStructureFlagsNV = BuildAccelerationStructureFlagsKHR;
pub type PFN_vkGetRayTracingShaderGroupHandlesNV = PFN_vkGetRayTracingShaderGroupHandlesKHR;
#[repr(C)]
pub struct RayTracingShaderGroupCreateInfoNV<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub ty: RayTracingShaderGroupTypeKHR,
    pub general_shader: u32,
    pub closest_hit_shader: u32,
    pub any_hit_shader: u32,
    pub intersection_shader: u32,
    pub _marker: PhantomData<&'a ()>,
}
#[repr(C)]
pub struct RayTracingPipelineCreateInfoNV<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: PipelineCreateFlags,
    pub stage_count: u32,
    pub p_stages: *const PipelineShaderStageCreateInfo<'a>,
    pub group_count: u32,
    pub p_groups: *const RayTracingShaderGroupCreateInfoNV<'a>,
    pub max_recursion_depth: u32,
    pub layout: PipelineLayout,
    pub base_pipeline_handle: Pipeline,
    pub base_pipeline_index: i32,
    pub _marker: PhantomData<&'a ()>,
}
#[repr(C)]
pub struct GeometryTrianglesNV<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub vertex_data: Buffer,
    pub vertex_offset: DeviceSize,
    pub vertex_count: u32,
    pub vertex_stride: DeviceSize,
    pub vertex_format: Format,
    pub index_data: Buffer,
    pub index_offset: DeviceSize,
    pub index_count: u32,
    pub index_type: IndexType,
    pub transform_data: Buffer,
    pub transform_offset: DeviceSize,
    pub _marker: PhantomData<&'a ()>,
}
#[repr(C)]
pub struct GeometryAABBNV<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub aabb_data: Buffer,
    pub num_aab_bs: u32,
    pub stride: u32,
    pub offset: DeviceSize,
    pub _marker: PhantomData<&'a ()>,
}
#[repr(C)]
pub struct GeometryDataNV<'a> {
    pub triangles: GeometryTrianglesNV<'a>,
    pub aabbs: GeometryAABBNV<'a>,
    pub _marker: PhantomData<&'a ()>,
}
#[repr(C)]
pub struct GeometryNV<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub geometry_type: GeometryTypeKHR,
    pub geometry: GeometryDataNV<'a>,
    pub flags: GeometryFlagsKHR,
    pub _marker: PhantomData<&'a ()>,
}
#[repr(C)]
pub struct AccelerationStructureInfoNV<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub ty: AccelerationStructureTypeNV,
    pub flags: BuildAccelerationStructureFlagsNV,
    pub instance_count: u32,
    pub geometry_count: u32,
    pub p_geometries: *const GeometryNV<'a>,
    pub _marker: PhantomData<&'a ()>,
}
#[repr(C)]
pub struct AccelerationStructureCreateInfoNV<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub compacted_size: DeviceSize,
    pub info: AccelerationStructureInfoNV<'a>,
    pub _marker: PhantomData<&'a ()>,
}
#[repr(C)]
pub struct BindAccelerationStructureMemoryInfoNV<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub acceleration_structure: AccelerationStructureNV,
    pub memory: DeviceMemory,
    pub memory_offset: DeviceSize,
    pub device_index_count: u32,
    pub p_device_indices: *const u32,
    pub _marker: PhantomData<&'a ()>,
}
#[repr(C)]
pub struct WriteDescriptorSetAccelerationStructureNV<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub acceleration_structure_count: u32,
    pub p_acceleration_structures: *const AccelerationStructureNV,
    pub _marker: PhantomData<&'a ()>,
}
#[repr(C)]
pub struct AccelerationStructureMemoryRequirementsInfoNV<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub ty: AccelerationStructureMemoryRequirementsTypeNV,
    pub acceleration_structure: AccelerationStructureNV,
    pub _marker: PhantomData<&'a ()>,
}
#[repr(C)]
pub struct PhysicalDeviceRayTracingPropertiesNV<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub shader_group_handle_size: u32,
    pub max_recursion_depth: u32,
    pub max_shader_group_stride: u32,
    pub shader_group_base_alignment: u32,
    pub max_geometry_count: u64,
    pub max_instance_count: u64,
    pub max_triangle_count: u64,
    pub max_descriptor_set_acceleration_structures: u32,
    pub _marker: PhantomData<&'a ()>,
}
#[repr(transparent)]
#[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct AccelerationStructureMemoryRequirementsTypeNV(i32);
impl AccelerationStructureMemoryRequirementsTypeNV {
    pub const OBJECT_NV: Self = Self(0);
    pub const BUILD_SCRATCH_NV: Self = Self(1);
    pub const UPDATE_SCRATCH_NV: Self = Self(2);
}
pub type PFN_vkCompileDeferredNV =
    unsafe extern "system" fn(device: Device, pipeline: Pipeline, shader: u32) -> Result;
pub type PFN_vkCreateAccelerationStructureNV = unsafe extern "system" fn(
    device: Device,
    p_create_info: *const AccelerationStructureCreateInfoNV<'_>,
    p_allocator: *const AllocationCallbacks<'_>,
    p_acceleration_structure: *mut AccelerationStructureNV,
) -> Result;
pub type PFN_vkDestroyAccelerationStructureNV = unsafe extern "system" fn(
    device: Device,
    acceleration_structure: AccelerationStructureNV,
    p_allocator: *const AllocationCallbacks<'_>,
);
pub type PFN_vkGetAccelerationStructureMemoryRequirementsNV = unsafe extern "system" fn(
    device: Device,
    p_info: *const AccelerationStructureMemoryRequirementsInfoNV<'_>,
    p_memory_requirements: *mut MemoryRequirements2KHR<'_>,
);
pub type PFN_vkBindAccelerationStructureMemoryNV = unsafe extern "system" fn(
    device: Device,
    bind_info_count: u32,
    p_bind_infos: *const BindAccelerationStructureMemoryInfoNV<'_>,
) -> Result;
pub type PFN_vkCmdCopyAccelerationStructureNV = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    dst: AccelerationStructureNV,
    src: AccelerationStructureNV,
    mode: CopyAccelerationStructureModeKHR,
);
pub type PFN_vkCmdWriteAccelerationStructuresPropertiesNV = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    acceleration_structure_count: u32,
    p_acceleration_structures: *const AccelerationStructureNV,
    query_type: QueryType,
    query_pool: QueryPool,
    first_query: u32,
);
pub type PFN_vkCmdBuildAccelerationStructureNV = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    p_info: *const AccelerationStructureInfoNV<'_>,
    instance_data: Buffer,
    instance_offset: DeviceSize,
    update: Bool32,
    dst: AccelerationStructureNV,
    src: AccelerationStructureNV,
    scratch: Buffer,
    scratch_offset: DeviceSize,
);
pub type PFN_vkCmdTraceRaysNV = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    raygen_shader_binding_table_buffer: Buffer,
    raygen_shader_binding_offset: DeviceSize,
    miss_shader_binding_table_buffer: Buffer,
    miss_shader_binding_offset: DeviceSize,
    miss_shader_binding_stride: DeviceSize,
    hit_shader_binding_table_buffer: Buffer,
    hit_shader_binding_offset: DeviceSize,
    hit_shader_binding_stride: DeviceSize,
    callable_shader_binding_table_buffer: Buffer,
    callable_shader_binding_offset: DeviceSize,
    callable_shader_binding_stride: DeviceSize,
    width: u32,
    height: u32,
    depth: u32,
);
pub type PFN_vkGetAccelerationStructureHandleNV = unsafe extern "system" fn(
    device: Device,
    acceleration_structure: AccelerationStructureNV,
    data_size: usize,
    p_data: *mut c_void,
) -> Result;
pub type PFN_vkCreateRayTracingPipelinesNV = unsafe extern "system" fn(
    device: Device,
    pipeline_cache: PipelineCache,
    create_info_count: u32,
    p_create_infos: *const RayTracingPipelineCreateInfoNV<'_>,
    p_allocator: *const AllocationCallbacks<'_>,
    p_pipelines: *mut Pipeline,
) -> Result;
