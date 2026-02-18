#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use std::ffi::{c_char, c_int, c_void};
pub const SHADER_INDEX_UNUSED_AMDX: u32 = !0;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceShaderEnqueuePropertiesAMDX {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub max_execution_graph_depth: u32,
    pub max_execution_graph_shader_output_nodes: u32,
    pub max_execution_graph_shader_payload_size: u32,
    pub max_execution_graph_shader_payload_count: u32,
    pub execution_graph_dispatch_address_alignment: u32,
    pub max_execution_graph_workgroup_count: u32,
    pub max_execution_graph_workgroups: u32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceShaderEnqueueFeaturesAMDX {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub shader_enqueue: Bool32,
    pub shader_mesh_enqueue: Bool32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ExecutionGraphPipelineCreateInfoAMDX {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: PipelineCreateFlags,
    pub stage_count: u32,
    pub p_stages: *const PipelineShaderStageCreateInfo,
    pub p_library_info: *const PipelineLibraryCreateInfoKHR,
    pub layout: PipelineLayout,
    pub base_pipeline_handle: Pipeline,
    pub base_pipeline_index: i32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PipelineShaderStageNodeCreateInfoAMDX {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub p_name: *const c_char,
    pub index: u32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ExecutionGraphPipelineScratchSizeAMDX {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub min_size: DeviceSize,
    pub max_size: DeviceSize,
    pub size_granularity: DeviceSize,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct DispatchGraphInfoAMDX {
    pub node_index: u32,
    pub payload_count: u32,
    pub payloads: DeviceOrHostAddressConstAMDX,
    pub payload_stride: u64,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct DispatchGraphCountInfoAMDX {
    pub count: u32,
    pub infos: DeviceOrHostAddressConstAMDX,
    pub stride: u64,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union DeviceOrHostAddressConstAMDX {
    pub device_address: DeviceAddress,
    pub host_address: *const c_void,
}
pub type PFN_vkGetExecutionGraphPipelineScratchSizeAMDX = unsafe extern "system" fn(
    device: Device,
    execution_graph: Pipeline,
    p_size_info: *mut ExecutionGraphPipelineScratchSizeAMDX,
) -> Result;
pub type PFN_vkGetExecutionGraphPipelineNodeIndexAMDX = unsafe extern "system" fn(
    device: Device,
    execution_graph: Pipeline,
    p_node_info: *const PipelineShaderStageNodeCreateInfoAMDX,
    p_node_index: *mut u32,
) -> Result;
pub type PFN_vkCreateExecutionGraphPipelinesAMDX = unsafe extern "system" fn(
    device: Device,
    pipeline_cache: PipelineCache,
    create_info_count: u32,
    p_create_infos: *const ExecutionGraphPipelineCreateInfoAMDX,
    p_allocator: *const AllocationCallbacks,
    p_pipelines: *mut Pipeline,
) -> Result;
pub type PFN_vkCmdInitializeGraphScratchMemoryAMDX = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    execution_graph: Pipeline,
    scratch: DeviceAddress,
    scratch_size: DeviceSize,
);
pub type PFN_vkCmdDispatchGraphAMDX = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    scratch: DeviceAddress,
    scratch_size: DeviceSize,
    p_count_info: *const DispatchGraphCountInfoAMDX,
);
pub type PFN_vkCmdDispatchGraphIndirectAMDX = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    scratch: DeviceAddress,
    scratch_size: DeviceSize,
    p_count_info: *const DispatchGraphCountInfoAMDX,
);
pub type PFN_vkCmdDispatchGraphIndirectCountAMDX = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    scratch: DeviceAddress,
    scratch_size: DeviceSize,
    count_info: DeviceAddress,
);
