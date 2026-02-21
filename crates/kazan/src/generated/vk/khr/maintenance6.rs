#![allow(unused_imports)]
use crate::*;
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
use kazan_sys::{vk::*, *};
pub struct DeviceFn {
    cmd_bind_descriptor_sets2_khr: PFN_vkCmdBindDescriptorSets2,
    cmd_push_constants2_khr: PFN_vkCmdPushConstants2,
    cmd_push_descriptor_set2_khr: Option<PFN_vkCmdPushDescriptorSet2>,
    cmd_push_descriptor_set_with_template2_khr: Option<PFN_vkCmdPushDescriptorSetWithTemplate2>,
    cmd_set_descriptor_buffer_offsets2_ext: Option<PFN_vkCmdSetDescriptorBufferOffsets2EXT>,
    cmd_bind_descriptor_buffer_embedded_samplers2_ext:
        Option<PFN_vkCmdBindDescriptorBufferEmbeddedSamplers2EXT>,
}
impl DeviceFn {
    pub unsafe fn load(
        load: impl Fn(&CStr) -> Option<PFN_vkVoidFunction>,
    ) -> core::result::Result<Self, LoadingError> {
        unsafe {
            Ok(Self {
                cmd_bind_descriptor_sets2_khr: transmute(
                    load(c"vkCmdBindDescriptorSets2KHR").ok_or(LoadingError)?,
                ),
                cmd_push_constants2_khr: transmute(
                    load(c"vkCmdPushConstants2KHR").ok_or(LoadingError)?,
                ),
                cmd_push_descriptor_set2_khr: transmute(load(c"vkCmdPushDescriptorSet2KHR")),
                cmd_push_descriptor_set_with_template2_khr: transmute(load(
                    c"vkCmdPushDescriptorSetWithTemplate2KHR",
                )),
                cmd_set_descriptor_buffer_offsets2_ext: transmute(load(
                    c"vkCmdSetDescriptorBufferOffsets2EXT",
                )),
                cmd_bind_descriptor_buffer_embedded_samplers2_ext: transmute(load(
                    c"vkCmdBindDescriptorBufferEmbeddedSamplers2EXT",
                )),
            })
        }
    }
}
impl DeviceFn {
    pub unsafe fn cmd_bind_descriptor_sets2_khr(
        &self,
        command_buffer: CommandBuffer,
        bind_descriptor_sets_info: &BindDescriptorSetsInfo,
    ) {
        unsafe { (self.cmd_bind_descriptor_sets2_khr)(command_buffer, bind_descriptor_sets_info) }
    }
    pub unsafe fn cmd_push_constants2_khr(
        &self,
        command_buffer: CommandBuffer,
        push_constants_info: &PushConstantsInfo,
    ) {
        unsafe { (self.cmd_push_constants2_khr)(command_buffer, push_constants_info) }
    }
    pub unsafe fn cmd_push_descriptor_set2_khr(
        &self,
        command_buffer: CommandBuffer,
        push_descriptor_set_info: &PushDescriptorSetInfo,
    ) {
        unsafe {
            (self.cmd_push_descriptor_set2_khr.unwrap())(command_buffer, push_descriptor_set_info)
        }
    }
    pub unsafe fn cmd_push_descriptor_set_with_template2_khr(
        &self,
        command_buffer: CommandBuffer,
        push_descriptor_set_with_template_info: &PushDescriptorSetWithTemplateInfo,
    ) {
        unsafe {
            (self.cmd_push_descriptor_set_with_template2_khr.unwrap())(
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
            (self.cmd_set_descriptor_buffer_offsets2_ext.unwrap())(
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
            (self
                .cmd_bind_descriptor_buffer_embedded_samplers2_ext
                .unwrap())(
                command_buffer,
                bind_descriptor_buffer_embedded_samplers_info,
            )
        }
    }
}
