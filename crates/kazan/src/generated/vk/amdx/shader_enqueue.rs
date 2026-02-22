#![allow(unused_imports)]
use crate::*;
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
use kazan_sys::{vk::*, *};
pub struct DeviceFn {
    create_execution_graph_pipelines_amdx: PFN_vkCreateExecutionGraphPipelinesAMDX,
    get_execution_graph_pipeline_scratch_size_amdx: PFN_vkGetExecutionGraphPipelineScratchSizeAMDX,
    get_execution_graph_pipeline_node_index_amdx: PFN_vkGetExecutionGraphPipelineNodeIndexAMDX,
    cmd_initialize_graph_scratch_memory_amdx: PFN_vkCmdInitializeGraphScratchMemoryAMDX,
    cmd_dispatch_graph_amdx: PFN_vkCmdDispatchGraphAMDX,
    cmd_dispatch_graph_indirect_amdx: PFN_vkCmdDispatchGraphIndirectAMDX,
    cmd_dispatch_graph_indirect_count_amdx: PFN_vkCmdDispatchGraphIndirectCountAMDX,
}
impl DeviceFn {
    pub unsafe fn load(
        load: impl Fn(&CStr) -> Option<PFN_vkVoidFunction>,
    ) -> core::result::Result<Self, LoadingError> {
        unsafe {
            Ok(Self {
                create_execution_graph_pipelines_amdx: transmute(
                    load(c"vkCreateExecutionGraphPipelinesAMDX").ok_or(LoadingError)?,
                ),
                get_execution_graph_pipeline_scratch_size_amdx: transmute(
                    load(c"vkGetExecutionGraphPipelineScratchSizeAMDX").ok_or(LoadingError)?,
                ),
                get_execution_graph_pipeline_node_index_amdx: transmute(
                    load(c"vkGetExecutionGraphPipelineNodeIndexAMDX").ok_or(LoadingError)?,
                ),
                cmd_initialize_graph_scratch_memory_amdx: transmute(
                    load(c"vkCmdInitializeGraphScratchMemoryAMDX").ok_or(LoadingError)?,
                ),
                cmd_dispatch_graph_amdx: transmute(
                    load(c"vkCmdDispatchGraphAMDX").ok_or(LoadingError)?,
                ),
                cmd_dispatch_graph_indirect_amdx: transmute(
                    load(c"vkCmdDispatchGraphIndirectAMDX").ok_or(LoadingError)?,
                ),
                cmd_dispatch_graph_indirect_count_amdx: transmute(
                    load(c"vkCmdDispatchGraphIndirectCountAMDX").ok_or(LoadingError)?,
                ),
            })
        }
    }
}
impl DeviceFn {
    pub unsafe fn create_execution_graph_pipelines_amdx(
        &self,
        device: Device,
        pipeline_cache: PipelineCache,
        create_infos: &[ExecutionGraphPipelineCreateInfoAMDX],
        allocator: Option<&AllocationCallbacks>,
        pipelines: &mut [Pipeline],
    ) -> crate::Result<()> {
        unsafe {
            result((self.create_execution_graph_pipelines_amdx)(
                device,
                pipeline_cache,
                create_infos.len().try_into().unwrap(),
                create_infos.as_ptr() as _,
                allocator.to_raw_ptr(),
                pipelines.as_mut_ptr() as _,
            ))
        }
    }
    pub unsafe fn get_execution_graph_pipeline_scratch_size_amdx(
        &self,
        device: Device,
        execution_graph: Pipeline,
        size_info: &mut ExecutionGraphPipelineScratchSizeAMDX,
    ) -> crate::Result<()> {
        unsafe {
            result((self.get_execution_graph_pipeline_scratch_size_amdx)(
                device,
                execution_graph,
                size_info,
            ))
        }
    }
    pub unsafe fn get_execution_graph_pipeline_node_index_amdx(
        &self,
        device: Device,
        execution_graph: Pipeline,
        node_info: &PipelineShaderStageNodeCreateInfoAMDX,
        node_index: &mut u32,
    ) -> crate::Result<()> {
        unsafe {
            result((self.get_execution_graph_pipeline_node_index_amdx)(
                device,
                execution_graph,
                node_info,
                node_index,
            ))
        }
    }
    pub unsafe fn cmd_initialize_graph_scratch_memory_amdx(
        &self,
        command_buffer: CommandBuffer,
        execution_graph: Pipeline,
        scratch: DeviceAddress,
        scratch_size: DeviceSize,
    ) {
        unsafe {
            (self.cmd_initialize_graph_scratch_memory_amdx)(
                command_buffer,
                execution_graph,
                scratch,
                scratch_size,
            )
        }
    }
    pub unsafe fn cmd_dispatch_graph_amdx(
        &self,
        command_buffer: CommandBuffer,
        scratch: DeviceAddress,
        scratch_size: DeviceSize,
        count_info: &DispatchGraphCountInfoAMDX,
    ) {
        unsafe { (self.cmd_dispatch_graph_amdx)(command_buffer, scratch, scratch_size, count_info) }
    }
    pub unsafe fn cmd_dispatch_graph_indirect_amdx(
        &self,
        command_buffer: CommandBuffer,
        scratch: DeviceAddress,
        scratch_size: DeviceSize,
        count_info: &DispatchGraphCountInfoAMDX,
    ) {
        unsafe {
            (self.cmd_dispatch_graph_indirect_amdx)(
                command_buffer,
                scratch,
                scratch_size,
                count_info,
            )
        }
    }
    pub unsafe fn cmd_dispatch_graph_indirect_count_amdx(
        &self,
        command_buffer: CommandBuffer,
        scratch: DeviceAddress,
        scratch_size: DeviceSize,
        count_info: DeviceAddress,
    ) {
        unsafe {
            (self.cmd_dispatch_graph_indirect_count_amdx)(
                command_buffer,
                scratch,
                scratch_size,
                count_info,
            )
        }
    }
}
