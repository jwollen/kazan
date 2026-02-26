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
#[derive(Copy, Clone)]
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
impl Default for RayTracingShaderGroupCreateInfoNV<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::RAY_TRACING_SHADER_GROUP_CREATE_INFO_NV,
            p_next: core::ptr::null(),
            ty: Default::default(),
            general_shader: Default::default(),
            closest_hit_shader: Default::default(),
            any_hit_shader: Default::default(),
            intersection_shader: Default::default(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
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
impl Default for RayTracingPipelineCreateInfoNV<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::RAY_TRACING_PIPELINE_CREATE_INFO_NV,
            p_next: core::ptr::null(),
            flags: Default::default(),
            stage_count: Default::default(),
            p_stages: core::ptr::null(),
            group_count: Default::default(),
            p_groups: core::ptr::null(),
            max_recursion_depth: Default::default(),
            layout: Default::default(),
            base_pipeline_handle: Default::default(),
            base_pipeline_index: Default::default(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
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
impl Default for GeometryTrianglesNV<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::GEOMETRY_TRIANGLES_NV,
            p_next: core::ptr::null(),
            vertex_data: Default::default(),
            vertex_offset: Default::default(),
            vertex_count: Default::default(),
            vertex_stride: Default::default(),
            vertex_format: Default::default(),
            index_data: Default::default(),
            index_offset: Default::default(),
            index_count: Default::default(),
            index_type: Default::default(),
            transform_data: Default::default(),
            transform_offset: Default::default(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct GeometryAABBNV<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub aabb_data: Buffer,
    pub num_aab_bs: u32,
    pub stride: u32,
    pub offset: DeviceSize,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for GeometryAABBNV<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::GEOMETRY_AABB_NV,
            p_next: core::ptr::null(),
            aabb_data: Default::default(),
            num_aab_bs: Default::default(),
            stride: Default::default(),
            offset: Default::default(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct GeometryDataNV<'a> {
    pub triangles: GeometryTrianglesNV<'a>,
    pub aabbs: GeometryAABBNV<'a>,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for GeometryDataNV<'_> {
    fn default() -> Self {
        Self {
            triangles: Default::default(),
            aabbs: Default::default(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct GeometryNV<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub geometry_type: GeometryTypeKHR,
    pub geometry: GeometryDataNV<'a>,
    pub flags: GeometryFlagsKHR,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for GeometryNV<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::GEOMETRY_NV,
            p_next: core::ptr::null(),
            geometry_type: Default::default(),
            geometry: Default::default(),
            flags: Default::default(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
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
impl Default for AccelerationStructureInfoNV<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::ACCELERATION_STRUCTURE_INFO_NV,
            p_next: core::ptr::null(),
            ty: Default::default(),
            flags: Default::default(),
            instance_count: Default::default(),
            geometry_count: Default::default(),
            p_geometries: core::ptr::null(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct AccelerationStructureCreateInfoNV<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub compacted_size: DeviceSize,
    pub info: AccelerationStructureInfoNV<'a>,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for AccelerationStructureCreateInfoNV<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::ACCELERATION_STRUCTURE_CREATE_INFO_NV,
            p_next: core::ptr::null(),
            compacted_size: Default::default(),
            info: Default::default(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
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
impl Default for BindAccelerationStructureMemoryInfoNV<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::BIND_ACCELERATION_STRUCTURE_MEMORY_INFO_NV,
            p_next: core::ptr::null(),
            acceleration_structure: Default::default(),
            memory: Default::default(),
            memory_offset: Default::default(),
            device_index_count: Default::default(),
            p_device_indices: core::ptr::null(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct WriteDescriptorSetAccelerationStructureNV<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub acceleration_structure_count: u32,
    pub p_acceleration_structures: *const AccelerationStructureNV,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for WriteDescriptorSetAccelerationStructureNV<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::WRITE_DESCRIPTOR_SET_ACCELERATION_STRUCTURE_NV,
            p_next: core::ptr::null(),
            acceleration_structure_count: Default::default(),
            p_acceleration_structures: core::ptr::null(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct AccelerationStructureMemoryRequirementsInfoNV<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub ty: AccelerationStructureMemoryRequirementsTypeNV,
    pub acceleration_structure: AccelerationStructureNV,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for AccelerationStructureMemoryRequirementsInfoNV<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::ACCELERATION_STRUCTURE_MEMORY_REQUIREMENTS_INFO_NV,
            p_next: core::ptr::null(),
            ty: Default::default(),
            acceleration_structure: Default::default(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
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
impl Default for PhysicalDeviceRayTracingPropertiesNV<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::PHYSICAL_DEVICE_RAY_TRACING_PROPERTIES_NV,
            p_next: core::ptr::null_mut(),
            shader_group_handle_size: Default::default(),
            max_recursion_depth: Default::default(),
            max_shader_group_stride: Default::default(),
            shader_group_base_alignment: Default::default(),
            max_geometry_count: Default::default(),
            max_instance_count: Default::default(),
            max_triangle_count: Default::default(),
            max_descriptor_set_acceleration_structures: Default::default(),
            _marker: PhantomData,
        }
    }
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
