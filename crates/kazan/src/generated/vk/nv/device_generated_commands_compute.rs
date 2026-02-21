#![allow(unused_imports)]
use crate::*;
use core::ffi::{c_char, c_int, c_void, CStr};
use kazan_sys::{vk::*, *};
pub struct DeviceFn {
    get_pipeline_indirect_memory_requirements_nv: PFN_vkGetPipelineIndirectMemoryRequirementsNV,
    cmd_update_pipeline_indirect_buffer_nv: PFN_vkCmdUpdatePipelineIndirectBufferNV,
    get_pipeline_indirect_device_address_nv: PFN_vkGetPipelineIndirectDeviceAddressNV,
}
impl DeviceFn {
    pub unsafe fn get_pipeline_indirect_memory_requirements_nv(
        &self,
        device: Device,
        create_info: &ComputePipelineCreateInfo,
        memory_requirements: &mut MemoryRequirements2,
    ) {
        unsafe {
            (self.get_pipeline_indirect_memory_requirements_nv)(
                device,
                create_info,
                memory_requirements,
            )
        }
    }
    pub unsafe fn cmd_update_pipeline_indirect_buffer_nv(
        &self,
        command_buffer: CommandBuffer,
        pipeline_bind_point: PipelineBindPoint,
        pipeline: Pipeline,
    ) {
        unsafe {
            (self.cmd_update_pipeline_indirect_buffer_nv)(
                command_buffer,
                pipeline_bind_point,
                pipeline,
            )
        }
    }
    pub unsafe fn get_pipeline_indirect_device_address_nv(
        &self,
        device: Device,
        info: &PipelineIndirectDeviceAddressInfoNV,
    ) -> DeviceAddress {
        unsafe { (self.get_pipeline_indirect_device_address_nv)(device, info) }
    }
}
