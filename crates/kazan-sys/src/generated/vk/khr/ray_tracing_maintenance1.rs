#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{c_char, c_int, c_void};
use core::marker::PhantomData;
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct TraceRaysIndirectCommand2KHR {
    pub raygen_shader_record_address: DeviceAddress,
    pub raygen_shader_record_size: DeviceSize,
    pub miss_shader_binding_table_address: DeviceAddress,
    pub miss_shader_binding_table_size: DeviceSize,
    pub miss_shader_binding_table_stride: DeviceSize,
    pub hit_shader_binding_table_address: DeviceAddress,
    pub hit_shader_binding_table_size: DeviceSize,
    pub hit_shader_binding_table_stride: DeviceSize,
    pub callable_shader_binding_table_address: DeviceAddress,
    pub callable_shader_binding_table_size: DeviceSize,
    pub callable_shader_binding_table_stride: DeviceSize,
    pub width: u32,
    pub height: u32,
    pub depth: u32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceRayTracingMaintenance1FeaturesKHR<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub ray_tracing_maintenance1: Bool32,
    pub ray_tracing_pipeline_trace_rays_indirect2: Bool32,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for PhysicalDeviceRayTracingMaintenance1FeaturesKHR<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::PHYSICAL_DEVICE_RAY_TRACING_MAINTENANCE_1_FEATURES_KHR,
            p_next: core::ptr::null_mut(),
            ray_tracing_maintenance1: Default::default(),
            ray_tracing_pipeline_trace_rays_indirect2: Default::default(),
            _marker: PhantomData,
        }
    }
}
pub type PFN_vkCmdTraceRaysIndirect2KHR = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    indirect_device_address: DeviceAddress,
);
