#![allow(unused_imports)]
use crate::*;
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
use kazan_sys::{vk::Result as VkResult, vk::*, *};
pub struct DeviceFn {
    get_pipeline_indirect_memory_requirements_nv: PFN_vkGetPipelineIndirectMemoryRequirementsNV,
    cmd_update_pipeline_indirect_buffer_nv: PFN_vkCmdUpdatePipelineIndirectBufferNV,
    get_pipeline_indirect_device_address_nv: PFN_vkGetPipelineIndirectDeviceAddressNV,
}
impl DeviceFn {
    pub unsafe fn load(
        load: impl Fn(&CStr) -> Option<PFN_vkVoidFunction>,
    ) -> core::result::Result<Self, LoadingError> {
        unsafe {
            Ok(Self {
                get_pipeline_indirect_memory_requirements_nv: transmute(
                    load(c"vkGetPipelineIndirectMemoryRequirementsNV").ok_or(LoadingError)?,
                ),
                cmd_update_pipeline_indirect_buffer_nv: transmute(
                    load(c"vkCmdUpdatePipelineIndirectBufferNV").ok_or(LoadingError)?,
                ),
                get_pipeline_indirect_device_address_nv: transmute(
                    load(c"vkGetPipelineIndirectDeviceAddressNV").ok_or(LoadingError)?,
                ),
            })
        }
    }
}
impl DeviceFn {
    pub unsafe fn get_pipeline_indirect_memory_requirements_nv(
        &self,
        device: Device,
        create_info: &ComputePipelineCreateInfo<'_>,
    ) -> MemoryRequirements2<'_> {
        unsafe {
            let mut memory_requirements = core::mem::MaybeUninit::uninit();
            (self.get_pipeline_indirect_memory_requirements_nv)(
                device,
                create_info,
                memory_requirements.as_mut_ptr(),
            );
            memory_requirements.assume_init()
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
        info: &PipelineIndirectDeviceAddressInfoNV<'_>,
    ) -> DeviceAddress {
        unsafe { (self.get_pipeline_indirect_device_address_nv)(device, info) }
    }
}
