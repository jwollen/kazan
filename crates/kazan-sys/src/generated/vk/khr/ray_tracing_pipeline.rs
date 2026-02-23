#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{c_char, c_int, c_void};
pub const SHADER_UNUSED_KHR: u32 = !0;
#[repr(C)]
pub struct RayTracingShaderGroupCreateInfoKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub ty: RayTracingShaderGroupTypeKHR,
    pub general_shader: u32,
    pub closest_hit_shader: u32,
    pub any_hit_shader: u32,
    pub intersection_shader: u32,
    pub p_shader_group_capture_replay_handle: *const c_void,
}
#[repr(C)]
pub struct RayTracingPipelineCreateInfoKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: PipelineCreateFlags,
    pub stage_count: u32,
    pub p_stages: *const PipelineShaderStageCreateInfo,
    pub group_count: u32,
    pub p_groups: *const RayTracingShaderGroupCreateInfoKHR,
    pub max_pipeline_ray_recursion_depth: u32,
    pub p_library_info: *const PipelineLibraryCreateInfoKHR,
    pub p_library_interface: *const RayTracingPipelineInterfaceCreateInfoKHR,
    pub p_dynamic_state: *const PipelineDynamicStateCreateInfo,
    pub layout: PipelineLayout,
    pub base_pipeline_handle: Pipeline,
    pub base_pipeline_index: i32,
}
#[repr(C)]
pub struct PhysicalDeviceRayTracingPipelineFeaturesKHR {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub ray_tracing_pipeline: Bool32,
    pub ray_tracing_pipeline_shader_group_handle_capture_replay: Bool32,
    pub ray_tracing_pipeline_shader_group_handle_capture_replay_mixed: Bool32,
    pub ray_tracing_pipeline_trace_rays_indirect: Bool32,
    pub ray_traversal_primitive_culling: Bool32,
}
#[repr(C)]
pub struct PhysicalDeviceRayTracingPipelinePropertiesKHR {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub shader_group_handle_size: u32,
    pub max_ray_recursion_depth: u32,
    pub max_shader_group_stride: u32,
    pub shader_group_base_alignment: u32,
    pub shader_group_handle_capture_replay_size: u32,
    pub max_ray_dispatch_invocation_count: u32,
    pub shader_group_handle_alignment: u32,
    pub max_ray_hit_attribute_size: u32,
}
#[repr(C)]
pub struct StridedDeviceAddressRegionKHR {
    pub device_address: DeviceAddress,
    pub stride: DeviceSize,
    pub size: DeviceSize,
}
#[repr(C)]
pub struct TraceRaysIndirectCommandKHR {
    pub width: u32,
    pub height: u32,
    pub depth: u32,
}
#[repr(C)]
pub struct RayTracingPipelineInterfaceCreateInfoKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub max_pipeline_ray_payload_size: u32,
    pub max_pipeline_ray_hit_attribute_size: u32,
}
#[repr(transparent)]
#[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct RayTracingShaderGroupTypeKHR(i32);
impl RayTracingShaderGroupTypeKHR {
    pub const GENERAL_KHR: Self = Self(0);
    pub const TRIANGLES_HIT_GROUP_KHR: Self = Self(1);
    pub const PROCEDURAL_HIT_GROUP_KHR: Self = Self(2);
    pub const GENERAL_NV: Self = Self::GENERAL_KHR;
    pub const PROCEDURAL_HIT_GROUP_NV: Self = Self::PROCEDURAL_HIT_GROUP_KHR;
    pub const TRIANGLES_HIT_GROUP_NV: Self = Self::TRIANGLES_HIT_GROUP_KHR;
}
#[repr(transparent)]
#[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ShaderGroupShaderKHR(i32);
impl ShaderGroupShaderKHR {
    pub const GENERAL_KHR: Self = Self(0);
    pub const CLOSEST_HIT_KHR: Self = Self(1);
    pub const ANY_HIT_KHR: Self = Self(2);
    pub const INTERSECTION_KHR: Self = Self(3);
}
pub type PFN_vkCmdTraceRaysKHR = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    p_raygen_shader_binding_table: *const StridedDeviceAddressRegionKHR,
    p_miss_shader_binding_table: *const StridedDeviceAddressRegionKHR,
    p_hit_shader_binding_table: *const StridedDeviceAddressRegionKHR,
    p_callable_shader_binding_table: *const StridedDeviceAddressRegionKHR,
    width: u32,
    height: u32,
    depth: u32,
);
pub type PFN_vkGetRayTracingShaderGroupHandlesKHR = unsafe extern "system" fn(
    device: Device,
    pipeline: Pipeline,
    first_group: u32,
    group_count: u32,
    data_size: usize,
    p_data: *mut c_void,
) -> Result;
pub type PFN_vkGetRayTracingCaptureReplayShaderGroupHandlesKHR =
    unsafe extern "system" fn(
        device: Device,
        pipeline: Pipeline,
        first_group: u32,
        group_count: u32,
        data_size: usize,
        p_data: *mut c_void,
    ) -> Result;
pub type PFN_vkCreateRayTracingPipelinesKHR = unsafe extern "system" fn(
    device: Device,
    deferred_operation: DeferredOperationKHR,
    pipeline_cache: PipelineCache,
    create_info_count: u32,
    p_create_infos: *const RayTracingPipelineCreateInfoKHR,
    p_allocator: *const AllocationCallbacks,
    p_pipelines: *mut Pipeline,
) -> Result;
pub type PFN_vkCmdTraceRaysIndirectKHR = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    p_raygen_shader_binding_table: *const StridedDeviceAddressRegionKHR,
    p_miss_shader_binding_table: *const StridedDeviceAddressRegionKHR,
    p_hit_shader_binding_table: *const StridedDeviceAddressRegionKHR,
    p_callable_shader_binding_table: *const StridedDeviceAddressRegionKHR,
    indirect_device_address: DeviceAddress,
);
pub type PFN_vkGetRayTracingShaderGroupStackSizeKHR = unsafe extern "system" fn(
    device: Device,
    pipeline: Pipeline,
    group: u32,
    group_shader: ShaderGroupShaderKHR,
) -> DeviceSize;
pub type PFN_vkCmdSetRayTracingPipelineStackSizeKHR =
    unsafe extern "system" fn(command_buffer: CommandBuffer, pipeline_stack_size: u32);
