#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use std::ffi::{c_char, c_int, c_void};
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceMeshShaderFeaturesEXT {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub task_shader: Bool32,
    pub mesh_shader: Bool32,
    pub multiview_mesh_shader: Bool32,
    pub primitive_fragment_shading_rate_mesh_shader: Bool32,
    pub mesh_shader_queries: Bool32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceMeshShaderPropertiesEXT {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub max_task_work_group_total_count: u32,
    pub max_task_work_group_count: u32,
    pub max_task_work_group_invocations: u32,
    pub max_task_work_group_size: u32,
    pub max_task_payload_size: u32,
    pub max_task_shared_memory_size: u32,
    pub max_task_payload_and_shared_memory_size: u32,
    pub max_mesh_work_group_total_count: u32,
    pub max_mesh_work_group_count: u32,
    pub max_mesh_work_group_invocations: u32,
    pub max_mesh_work_group_size: u32,
    pub max_mesh_shared_memory_size: u32,
    pub max_mesh_payload_and_shared_memory_size: u32,
    pub max_mesh_output_memory_size: u32,
    pub max_mesh_payload_and_output_memory_size: u32,
    pub max_mesh_output_components: u32,
    pub max_mesh_output_vertices: u32,
    pub max_mesh_output_primitives: u32,
    pub max_mesh_output_layers: u32,
    pub max_mesh_multiview_view_count: u32,
    pub mesh_output_per_vertex_granularity: u32,
    pub mesh_output_per_primitive_granularity: u32,
    pub max_preferred_task_work_group_invocations: u32,
    pub max_preferred_mesh_work_group_invocations: u32,
    pub prefers_local_invocation_vertex_output: Bool32,
    pub prefers_local_invocation_primitive_output: Bool32,
    pub prefers_compact_vertex_output: Bool32,
    pub prefers_compact_primitive_output: Bool32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct DrawMeshTasksIndirectCommandEXT {
    pub group_count_x: u32,
    pub group_count_y: u32,
    pub group_count_z: u32,
}
pub type PFN_vkCmdDrawMeshTasksEXT = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    group_count_x: u32,
    group_count_y: u32,
    group_count_z: u32,
);
pub type PFN_vkCmdDrawMeshTasksIndirectEXT = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    buffer: Buffer,
    offset: DeviceSize,
    draw_count: u32,
    stride: u32,
);
pub type PFN_vkCmdDrawMeshTasksIndirectCountEXT = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    buffer: Buffer,
    offset: DeviceSize,
    count_buffer: Buffer,
    count_buffer_offset: DeviceSize,
    max_draw_count: u32,
    stride: u32,
);
