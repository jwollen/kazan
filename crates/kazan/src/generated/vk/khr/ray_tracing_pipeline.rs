#![allow(unused_imports)]
use crate::*;
use core::ffi::{c_char, c_int, c_void, CStr};
use kazan_sys::{vk::*, *};
pub struct DeviceFn {
    cmd_trace_rays_khr: PFN_vkCmdTraceRaysKHR,
    get_ray_tracing_shader_group_handles_khr: PFN_vkGetRayTracingShaderGroupHandlesKHR,
    get_ray_tracing_capture_replay_shader_group_handles_khr:
        PFN_vkGetRayTracingCaptureReplayShaderGroupHandlesKHR,
    create_ray_tracing_pipelines_khr: PFN_vkCreateRayTracingPipelinesKHR,
    cmd_trace_rays_indirect_khr: PFN_vkCmdTraceRaysIndirectKHR,
    get_ray_tracing_shader_group_stack_size_khr: PFN_vkGetRayTracingShaderGroupStackSizeKHR,
    cmd_set_ray_tracing_pipeline_stack_size_khr: PFN_vkCmdSetRayTracingPipelineStackSizeKHR,
}
impl DeviceFn {
    pub unsafe fn cmd_trace_rays_khr(
        &self,
        command_buffer: CommandBuffer,
        raygen_shader_binding_table: &StridedDeviceAddressRegionKHR,
        miss_shader_binding_table: &StridedDeviceAddressRegionKHR,
        hit_shader_binding_table: &StridedDeviceAddressRegionKHR,
        callable_shader_binding_table: &StridedDeviceAddressRegionKHR,
        width: u32,
        height: u32,
        depth: u32,
    ) {
        unsafe {
            (self.cmd_trace_rays_khr)(
                command_buffer,
                raygen_shader_binding_table,
                miss_shader_binding_table,
                hit_shader_binding_table,
                callable_shader_binding_table,
                width,
                height,
                depth,
            )
        }
    }
    pub unsafe fn get_ray_tracing_shader_group_handles_khr(
        &self,
        device: Device,
        pipeline: Pipeline,
        first_group: u32,
        group_count: u32,
        data: &mut [u8],
    ) -> Result {
        unsafe {
            (self.get_ray_tracing_shader_group_handles_khr)(
                device,
                pipeline,
                first_group,
                group_count,
                data.len().try_into().unwrap(),
                data.as_mut_ptr() as _,
            )
        }
    }
    pub unsafe fn get_ray_tracing_capture_replay_shader_group_handles_khr(
        &self,
        device: Device,
        pipeline: Pipeline,
        first_group: u32,
        group_count: u32,
        data: &mut [u8],
    ) -> Result {
        unsafe {
            (self.get_ray_tracing_capture_replay_shader_group_handles_khr)(
                device,
                pipeline,
                first_group,
                group_count,
                data.len().try_into().unwrap(),
                data.as_mut_ptr() as _,
            )
        }
    }
    pub unsafe fn create_ray_tracing_pipelines_khr(
        &self,
        device: Device,
        deferred_operation: DeferredOperationKHR,
        pipeline_cache: PipelineCache,
        create_infos: &[RayTracingPipelineCreateInfoKHR],
        allocator: Option<&AllocationCallbacks>,
        pipelines: &mut [Pipeline],
    ) -> Result {
        unsafe {
            (self.create_ray_tracing_pipelines_khr)(
                device,
                deferred_operation,
                pipeline_cache,
                create_infos.len().try_into().unwrap(),
                create_infos.as_ptr() as _,
                allocator.to_raw_ptr(),
                pipelines.as_mut_ptr() as _,
            )
        }
    }
    pub unsafe fn cmd_trace_rays_indirect_khr(
        &self,
        command_buffer: CommandBuffer,
        raygen_shader_binding_table: &StridedDeviceAddressRegionKHR,
        miss_shader_binding_table: &StridedDeviceAddressRegionKHR,
        hit_shader_binding_table: &StridedDeviceAddressRegionKHR,
        callable_shader_binding_table: &StridedDeviceAddressRegionKHR,
        indirect_device_address: DeviceAddress,
    ) {
        unsafe {
            (self.cmd_trace_rays_indirect_khr)(
                command_buffer,
                raygen_shader_binding_table,
                miss_shader_binding_table,
                hit_shader_binding_table,
                callable_shader_binding_table,
                indirect_device_address,
            )
        }
    }
    pub unsafe fn get_ray_tracing_shader_group_stack_size_khr(
        &self,
        device: Device,
        pipeline: Pipeline,
        group: u32,
        group_shader: ShaderGroupShaderKHR,
    ) -> DeviceSize {
        unsafe {
            (self.get_ray_tracing_shader_group_stack_size_khr)(
                device,
                pipeline,
                group,
                group_shader,
            )
        }
    }
    pub unsafe fn cmd_set_ray_tracing_pipeline_stack_size_khr(
        &self,
        command_buffer: CommandBuffer,
        pipeline_stack_size: u32,
    ) {
        unsafe {
            (self.cmd_set_ray_tracing_pipeline_stack_size_khr)(command_buffer, pipeline_stack_size)
        }
    }
}
