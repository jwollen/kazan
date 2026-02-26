#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{c_char, c_int, c_void};
use core::marker::PhantomData;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceMeshShaderFeaturesNV<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub task_shader: Bool32,
    pub mesh_shader: Bool32,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for PhysicalDeviceMeshShaderFeaturesNV<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::PHYSICAL_DEVICE_MESH_SHADER_FEATURES_NV,
            p_next: core::ptr::null_mut(),
            task_shader: Default::default(),
            mesh_shader: Default::default(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceMeshShaderPropertiesNV<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub max_draw_mesh_tasks_count: u32,
    pub max_task_work_group_invocations: u32,
    pub max_task_work_group_size: [u32; 3],
    pub max_task_total_memory_size: u32,
    pub max_task_output_count: u32,
    pub max_mesh_work_group_invocations: u32,
    pub max_mesh_work_group_size: [u32; 3],
    pub max_mesh_total_memory_size: u32,
    pub max_mesh_output_vertices: u32,
    pub max_mesh_output_primitives: u32,
    pub max_mesh_multiview_view_count: u32,
    pub mesh_output_per_vertex_granularity: u32,
    pub mesh_output_per_primitive_granularity: u32,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for PhysicalDeviceMeshShaderPropertiesNV<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::PHYSICAL_DEVICE_MESH_SHADER_PROPERTIES_NV,
            p_next: core::ptr::null_mut(),
            max_draw_mesh_tasks_count: Default::default(),
            max_task_work_group_invocations: Default::default(),
            max_task_work_group_size: [Default::default(); _],
            max_task_total_memory_size: Default::default(),
            max_task_output_count: Default::default(),
            max_mesh_work_group_invocations: Default::default(),
            max_mesh_work_group_size: [Default::default(); _],
            max_mesh_total_memory_size: Default::default(),
            max_mesh_output_vertices: Default::default(),
            max_mesh_output_primitives: Default::default(),
            max_mesh_multiview_view_count: Default::default(),
            mesh_output_per_vertex_granularity: Default::default(),
            mesh_output_per_primitive_granularity: Default::default(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct DrawMeshTasksIndirectCommandNV {
    pub task_count: u32,
    pub first_task: u32,
}
pub type PFN_vkCmdDrawMeshTasksNV =
    unsafe extern "system" fn(command_buffer: CommandBuffer, task_count: u32, first_task: u32);
pub type PFN_vkCmdDrawMeshTasksIndirectNV = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    buffer: Buffer,
    offset: DeviceSize,
    draw_count: u32,
    stride: u32,
);
pub type PFN_vkCmdDrawMeshTasksIndirectCountNV = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    buffer: Buffer,
    offset: DeviceSize,
    count_buffer: Buffer,
    count_buffer_offset: DeviceSize,
    max_draw_count: u32,
    stride: u32,
);
