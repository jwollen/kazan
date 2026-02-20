#![allow(unused_imports)]
use crate::*;
use kazan_sys::{vk::*, *};
use std::ffi::{c_char, c_int, c_void, CStr};
pub struct DeviceFn {
    compile_deferred_nv: PFN_vkCompileDeferredNV,
    create_acceleration_structure_nv: PFN_vkCreateAccelerationStructureNV,
    destroy_acceleration_structure_nv: PFN_vkDestroyAccelerationStructureNV,
    get_acceleration_structure_memory_requirements_nv:
        PFN_vkGetAccelerationStructureMemoryRequirementsNV,
    bind_acceleration_structure_memory_nv: PFN_vkBindAccelerationStructureMemoryNV,
    cmd_copy_acceleration_structure_nv: PFN_vkCmdCopyAccelerationStructureNV,
    cmd_write_acceleration_structures_properties_nv:
        PFN_vkCmdWriteAccelerationStructuresPropertiesNV,
    cmd_build_acceleration_structure_nv: PFN_vkCmdBuildAccelerationStructureNV,
    cmd_trace_rays_nv: PFN_vkCmdTraceRaysNV,
    get_acceleration_structure_handle_nv: PFN_vkGetAccelerationStructureHandleNV,
    create_ray_tracing_pipelines_nv: PFN_vkCreateRayTracingPipelinesNV,
}
impl DeviceFn {
    pub unsafe fn compile_deferred_nv(
        &self,
        device: Device,
        pipeline: Pipeline,
        shader: u32,
    ) -> Result {
        unsafe { (self.compile_deferred_nv)(device, pipeline, shader) }
    }
    pub unsafe fn create_acceleration_structure_nv(
        &self,
        device: Device,
        create_info: &AccelerationStructureCreateInfoNV,
        allocator: Option<&AllocationCallbacks>,
        acceleration_structure: &mut AccelerationStructureNV,
    ) -> Result {
        unsafe {
            (self.create_acceleration_structure_nv)(
                device,
                create_info,
                allocator.to_raw_ptr(),
                acceleration_structure,
            )
        }
    }
    pub unsafe fn destroy_acceleration_structure_nv(
        &self,
        device: Device,
        acceleration_structure: AccelerationStructureNV,
        allocator: Option<&AllocationCallbacks>,
    ) {
        unsafe {
            (self.destroy_acceleration_structure_nv)(
                device,
                acceleration_structure,
                allocator.to_raw_ptr(),
            )
        }
    }
    pub unsafe fn get_acceleration_structure_memory_requirements_nv(
        &self,
        device: Device,
        info: &AccelerationStructureMemoryRequirementsInfoNV,
        memory_requirements: &mut MemoryRequirements2KHR,
    ) {
        unsafe {
            (self.get_acceleration_structure_memory_requirements_nv)(
                device,
                info,
                memory_requirements,
            )
        }
    }
    pub unsafe fn bind_acceleration_structure_memory_nv(
        &self,
        device: Device,
        bind_infos: &[BindAccelerationStructureMemoryInfoNV],
    ) -> Result {
        unsafe {
            (self.bind_acceleration_structure_memory_nv)(
                device,
                bind_infos.len().try_into().unwrap(),
                bind_infos.as_ptr() as _,
            )
        }
    }
    pub unsafe fn cmd_copy_acceleration_structure_nv(
        &self,
        command_buffer: CommandBuffer,
        dst: AccelerationStructureNV,
        src: AccelerationStructureNV,
        mode: CopyAccelerationStructureModeKHR,
    ) {
        unsafe { (self.cmd_copy_acceleration_structure_nv)(command_buffer, dst, src, mode) }
    }
    pub unsafe fn cmd_write_acceleration_structures_properties_nv(
        &self,
        command_buffer: CommandBuffer,
        acceleration_structures: &[AccelerationStructureNV],
        query_type: QueryType,
        query_pool: QueryPool,
        first_query: u32,
    ) {
        unsafe {
            (self.cmd_write_acceleration_structures_properties_nv)(
                command_buffer,
                acceleration_structures.len().try_into().unwrap(),
                acceleration_structures.as_ptr() as _,
                query_type,
                query_pool,
                first_query,
            )
        }
    }
    pub unsafe fn cmd_build_acceleration_structure_nv(
        &self,
        command_buffer: CommandBuffer,
        info: &AccelerationStructureInfoNV,
        instance_data: Buffer,
        instance_offset: DeviceSize,
        update: Bool32,
        dst: AccelerationStructureNV,
        src: AccelerationStructureNV,
        scratch: Buffer,
        scratch_offset: DeviceSize,
    ) {
        unsafe {
            (self.cmd_build_acceleration_structure_nv)(
                command_buffer,
                info,
                instance_data,
                instance_offset,
                update,
                dst,
                src,
                scratch,
                scratch_offset,
            )
        }
    }
    pub unsafe fn cmd_trace_rays_nv(
        &self,
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
    ) {
        unsafe {
            (self.cmd_trace_rays_nv)(
                command_buffer,
                raygen_shader_binding_table_buffer,
                raygen_shader_binding_offset,
                miss_shader_binding_table_buffer,
                miss_shader_binding_offset,
                miss_shader_binding_stride,
                hit_shader_binding_table_buffer,
                hit_shader_binding_offset,
                hit_shader_binding_stride,
                callable_shader_binding_table_buffer,
                callable_shader_binding_offset,
                callable_shader_binding_stride,
                width,
                height,
                depth,
            )
        }
    }
    pub unsafe fn get_acceleration_structure_handle_nv(
        &self,
        device: Device,
        acceleration_structure: AccelerationStructureNV,
        data: &mut [u8],
    ) -> Result {
        unsafe {
            (self.get_acceleration_structure_handle_nv)(
                device,
                acceleration_structure,
                data.len().try_into().unwrap(),
                data.as_mut_ptr() as _,
            )
        }
    }
    pub unsafe fn create_ray_tracing_pipelines_nv(
        &self,
        device: Device,
        pipeline_cache: PipelineCache,
        create_infos: &[RayTracingPipelineCreateInfoNV],
        allocator: Option<&AllocationCallbacks>,
        pipelines: &mut [Pipeline],
    ) -> Result {
        unsafe {
            (self.create_ray_tracing_pipelines_nv)(
                device,
                pipeline_cache,
                create_infos.len().try_into().unwrap(),
                create_infos.as_ptr() as _,
                allocator.to_raw_ptr(),
                pipelines.as_mut_ptr() as _,
            )
        }
    }
}
