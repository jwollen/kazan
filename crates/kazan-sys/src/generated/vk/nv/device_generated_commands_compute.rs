#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{c_char, c_int, c_void};
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ComputePipelineIndirectBufferInfoNV {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub device_address: DeviceAddress,
    pub size: DeviceSize,
    pub pipeline_device_address_capture_replay: DeviceAddress,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceDeviceGeneratedCommandsComputeFeaturesNV {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub device_generated_compute: Bool32,
    pub device_generated_compute_pipelines: Bool32,
    pub device_generated_compute_capture_replay: Bool32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PipelineIndirectDeviceAddressInfoNV {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub pipeline_bind_point: PipelineBindPoint,
    pub pipeline: Pipeline,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct BindPipelineIndirectCommandNV {
    pub pipeline_address: DeviceAddress,
}
pub type PFN_vkCmdUpdatePipelineIndirectBufferNV = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    pipeline_bind_point: PipelineBindPoint,
    pipeline: Pipeline,
);
pub type PFN_vkGetPipelineIndirectMemoryRequirementsNV = unsafe extern "system" fn(
    device: Device,
    p_create_info: *const ComputePipelineCreateInfo,
    p_memory_requirements: *mut MemoryRequirements2,
);
pub type PFN_vkGetPipelineIndirectDeviceAddressNV = unsafe extern "system" fn(
    device: Device,
    p_info: *const PipelineIndirectDeviceAddressInfoNV,
) -> DeviceAddress;
