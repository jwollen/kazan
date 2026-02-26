#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{c_char, c_int, c_void};
use core::marker::PhantomData;
pub const SHADER_INDEX_UNUSED_AMDX: u32 = !0;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceShaderEnqueuePropertiesAMDX<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub max_execution_graph_depth: u32,
    pub max_execution_graph_shader_output_nodes: u32,
    pub max_execution_graph_shader_payload_size: u32,
    pub max_execution_graph_shader_payload_count: u32,
    pub execution_graph_dispatch_address_alignment: u32,
    pub max_execution_graph_workgroup_count: [u32; 3],
    pub max_execution_graph_workgroups: u32,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for PhysicalDeviceShaderEnqueuePropertiesAMDX<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::PHYSICAL_DEVICE_SHADER_ENQUEUE_PROPERTIES_AMDX,
            p_next: core::ptr::null_mut(),
            max_execution_graph_depth: Default::default(),
            max_execution_graph_shader_output_nodes: Default::default(),
            max_execution_graph_shader_payload_size: Default::default(),
            max_execution_graph_shader_payload_count: Default::default(),
            execution_graph_dispatch_address_alignment: Default::default(),
            max_execution_graph_workgroup_count: [Default::default(); _],
            max_execution_graph_workgroups: Default::default(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceShaderEnqueueFeaturesAMDX<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub shader_enqueue: Bool32,
    pub shader_mesh_enqueue: Bool32,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for PhysicalDeviceShaderEnqueueFeaturesAMDX<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::PHYSICAL_DEVICE_SHADER_ENQUEUE_FEATURES_AMDX,
            p_next: core::ptr::null_mut(),
            shader_enqueue: Default::default(),
            shader_mesh_enqueue: Default::default(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ExecutionGraphPipelineCreateInfoAMDX<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: PipelineCreateFlags,
    pub stage_count: u32,
    pub p_stages: *const PipelineShaderStageCreateInfo<'a>,
    pub p_library_info: *const PipelineLibraryCreateInfoKHR<'a>,
    pub layout: PipelineLayout,
    pub base_pipeline_handle: Pipeline,
    pub base_pipeline_index: i32,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for ExecutionGraphPipelineCreateInfoAMDX<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::EXECUTION_GRAPH_PIPELINE_CREATE_INFO_AMDX,
            p_next: core::ptr::null(),
            flags: Default::default(),
            stage_count: Default::default(),
            p_stages: core::ptr::null(),
            p_library_info: core::ptr::null(),
            layout: Default::default(),
            base_pipeline_handle: Default::default(),
            base_pipeline_index: Default::default(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PipelineShaderStageNodeCreateInfoAMDX<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub p_name: *const c_char,
    pub index: u32,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for PipelineShaderStageNodeCreateInfoAMDX<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::PIPELINE_SHADER_STAGE_NODE_CREATE_INFO_AMDX,
            p_next: core::ptr::null(),
            p_name: core::ptr::null(),
            index: Default::default(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ExecutionGraphPipelineScratchSizeAMDX<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub min_size: DeviceSize,
    pub max_size: DeviceSize,
    pub size_granularity: DeviceSize,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for ExecutionGraphPipelineScratchSizeAMDX<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::EXECUTION_GRAPH_PIPELINE_SCRATCH_SIZE_AMDX,
            p_next: core::ptr::null_mut(),
            min_size: Default::default(),
            max_size: Default::default(),
            size_granularity: Default::default(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct DispatchGraphInfoAMDX<'a> {
    pub node_index: u32,
    pub payload_count: u32,
    pub payloads: DeviceOrHostAddressConstAMDX<'a>,
    pub payload_stride: u64,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for DispatchGraphInfoAMDX<'_> {
    fn default() -> Self {
        Self {
            node_index: Default::default(),
            payload_count: Default::default(),
            payloads: Default::default(),
            payload_stride: Default::default(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct DispatchGraphCountInfoAMDX<'a> {
    pub count: u32,
    pub infos: DeviceOrHostAddressConstAMDX<'a>,
    pub stride: u64,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for DispatchGraphCountInfoAMDX<'_> {
    fn default() -> Self {
        Self {
            count: Default::default(),
            infos: Default::default(),
            stride: Default::default(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union DeviceOrHostAddressConstAMDX<'a> {
    pub device_address: DeviceAddress,
    pub host_address: *const c_void,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for DeviceOrHostAddressConstAMDX<'_> {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type PFN_vkGetExecutionGraphPipelineScratchSizeAMDX = unsafe extern "system" fn(
    device: Device,
    execution_graph: Pipeline,
    p_size_info: *mut ExecutionGraphPipelineScratchSizeAMDX<'_>,
) -> Result;
pub type PFN_vkGetExecutionGraphPipelineNodeIndexAMDX = unsafe extern "system" fn(
    device: Device,
    execution_graph: Pipeline,
    p_node_info: *const PipelineShaderStageNodeCreateInfoAMDX<'_>,
    p_node_index: *mut u32,
) -> Result;
pub type PFN_vkCreateExecutionGraphPipelinesAMDX = unsafe extern "system" fn(
    device: Device,
    pipeline_cache: PipelineCache,
    create_info_count: u32,
    p_create_infos: *const ExecutionGraphPipelineCreateInfoAMDX<'_>,
    p_allocator: *const AllocationCallbacks<'_>,
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
    p_count_info: *const DispatchGraphCountInfoAMDX<'_>,
);
pub type PFN_vkCmdDispatchGraphIndirectAMDX = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    scratch: DeviceAddress,
    scratch_size: DeviceSize,
    p_count_info: *const DispatchGraphCountInfoAMDX<'_>,
);
pub type PFN_vkCmdDispatchGraphIndirectCountAMDX = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    scratch: DeviceAddress,
    scratch_size: DeviceSize,
    count_info: DeviceAddress,
);
