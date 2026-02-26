#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{c_char, c_int, c_void};
use core::marker::PhantomData;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ComputePipelineIndirectBufferInfoNV<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub device_address: DeviceAddress,
    pub size: DeviceSize,
    pub pipeline_device_address_capture_replay: DeviceAddress,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for ComputePipelineIndirectBufferInfoNV<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::COMPUTE_PIPELINE_INDIRECT_BUFFER_INFO_NV,
            p_next: core::ptr::null(),
            device_address: Default::default(),
            size: Default::default(),
            pipeline_device_address_capture_replay: Default::default(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceDeviceGeneratedCommandsComputeFeaturesNV<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub device_generated_compute: Bool32,
    pub device_generated_compute_pipelines: Bool32,
    pub device_generated_compute_capture_replay: Bool32,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for PhysicalDeviceDeviceGeneratedCommandsComputeFeaturesNV<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::PHYSICAL_DEVICE_DEVICE_GENERATED_COMMANDS_COMPUTE_FEATURES_NV,
            p_next: core::ptr::null_mut(),
            device_generated_compute: Default::default(),
            device_generated_compute_pipelines: Default::default(),
            device_generated_compute_capture_replay: Default::default(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PipelineIndirectDeviceAddressInfoNV<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub pipeline_bind_point: PipelineBindPoint,
    pub pipeline: Pipeline,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for PipelineIndirectDeviceAddressInfoNV<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::PIPELINE_INDIRECT_DEVICE_ADDRESS_INFO_NV,
            p_next: core::ptr::null(),
            pipeline_bind_point: Default::default(),
            pipeline: Default::default(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Default)]
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
    p_create_info: *const ComputePipelineCreateInfo<'_>,
    p_memory_requirements: *mut MemoryRequirements2<'_>,
);
pub type PFN_vkGetPipelineIndirectDeviceAddressNV = unsafe extern "system" fn(
    device: Device,
    p_info: *const PipelineIndirectDeviceAddressInfoNV<'_>,
) -> DeviceAddress;
