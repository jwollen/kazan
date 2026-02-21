#![allow(unused_imports)]
use crate::*;
use core::ffi::{c_char, c_int, c_void, CStr};
use kazan_sys::{vk::*, *};
pub struct DeviceFn {
    cmd_bind_descriptor_sets2: PFN_vkCmdBindDescriptorSets2,
    cmd_push_constants2: PFN_vkCmdPushConstants2,
    cmd_push_descriptor_set2: PFN_vkCmdPushDescriptorSet2,
    cmd_push_descriptor_set_with_template2: PFN_vkCmdPushDescriptorSetWithTemplate2,
    cmd_set_descriptor_buffer_offsets2_ext: PFN_vkCmdSetDescriptorBufferOffsets2EXT,
    cmd_bind_descriptor_buffer_embedded_samplers2_ext:
        PFN_vkCmdBindDescriptorBufferEmbeddedSamplers2EXT,
}
impl DeviceFn {
    pub unsafe fn cmd_bind_descriptor_sets2_khr(
        &self,
        command_buffer: CommandBuffer,
        bind_descriptor_sets_info: &BindDescriptorSetsInfo,
    ) {
        unsafe { (self.cmd_bind_descriptor_sets2)(command_buffer, bind_descriptor_sets_info) }
    }
    pub unsafe fn cmd_push_constants2_khr(
        &self,
        command_buffer: CommandBuffer,
        push_constants_info: &PushConstantsInfo,
    ) {
        unsafe { (self.cmd_push_constants2)(command_buffer, push_constants_info) }
    }
    pub unsafe fn cmd_push_descriptor_set2_khr(
        &self,
        command_buffer: CommandBuffer,
        push_descriptor_set_info: &PushDescriptorSetInfo,
    ) {
        unsafe { (self.cmd_push_descriptor_set2)(command_buffer, push_descriptor_set_info) }
    }
    pub unsafe fn cmd_push_descriptor_set_with_template2_khr(
        &self,
        command_buffer: CommandBuffer,
        push_descriptor_set_with_template_info: &PushDescriptorSetWithTemplateInfo,
    ) {
        unsafe {
            (self.cmd_push_descriptor_set_with_template2)(
                command_buffer,
                push_descriptor_set_with_template_info,
            )
        }
    }
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
