#![allow(unused_imports)]
use crate::*;
use kazan_sys::{vk::*, *};
use std::ffi::{c_char, c_int, c_void, CStr};
pub struct DeviceFn {
    cmd_set_descriptor_buffer_offsets2_ext: PFN_vkCmdSetDescriptorBufferOffsets2EXT,
    cmd_bind_descriptor_buffer_embedded_samplers2_ext:
        PFN_vkCmdBindDescriptorBufferEmbeddedSamplers2EXT,
}
impl DeviceFn {
    pub unsafe fn cmd_set_descriptor_buffer_offsets2_ext(
        &self,
        command_buffer: CommandBuffer,
        set_descriptor_buffer_offsets_info: &SetDescriptorBufferOffsetsInfoEXT,
    ) {
        unsafe {
            (self.cmd_set_descriptor_buffer_offsets2_ext)(
                command_buffer,
                set_descriptor_buffer_offsets_info,
            )
        }
    }
    pub unsafe fn cmd_bind_descriptor_buffer_embedded_samplers2_ext(
        &self,
        command_buffer: CommandBuffer,
        bind_descriptor_buffer_embedded_samplers_info: &BindDescriptorBufferEmbeddedSamplersInfoEXT,
    ) {
        unsafe {
            (self.cmd_bind_descriptor_buffer_embedded_samplers2_ext)(
                command_buffer,
                bind_descriptor_buffer_embedded_samplers_info,
            )
        }
    }
}
