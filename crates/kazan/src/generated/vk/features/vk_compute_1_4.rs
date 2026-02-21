#![allow(unused_imports)]
use crate::*;
use core::ffi::{c_char, c_int, c_void, CStr};
use kazan_sys::{vk::*, *};
pub struct DeviceFn {
    cmd_push_descriptor_set: PFN_vkCmdPushDescriptorSet,
    cmd_push_descriptor_set_with_template: PFN_vkCmdPushDescriptorSetWithTemplate,
    cmd_bind_descriptor_sets2: PFN_vkCmdBindDescriptorSets2,
    cmd_push_constants2: PFN_vkCmdPushConstants2,
    cmd_push_descriptor_set2: PFN_vkCmdPushDescriptorSet2,
    cmd_push_descriptor_set_with_template2: PFN_vkCmdPushDescriptorSetWithTemplate2,
}
impl DeviceFn {
    pub unsafe fn cmd_push_descriptor_set(
        &self,
        command_buffer: CommandBuffer,
        pipeline_bind_point: PipelineBindPoint,
        layout: PipelineLayout,
        set: u32,
        descriptor_writes: &[WriteDescriptorSet],
    ) {
        unsafe {
            (self.cmd_push_descriptor_set)(
                command_buffer,
                pipeline_bind_point,
                layout,
                set,
                descriptor_writes.len().try_into().unwrap(),
                descriptor_writes.as_ptr() as _,
            )
        }
    }
    pub unsafe fn cmd_push_descriptor_set_with_template(
        &self,
        command_buffer: CommandBuffer,
        descriptor_update_template: DescriptorUpdateTemplate,
        layout: PipelineLayout,
        set: u32,
        data: &c_void,
    ) {
        unsafe {
            (self.cmd_push_descriptor_set_with_template)(
                command_buffer,
                descriptor_update_template,
                layout,
                set,
                data,
            )
        }
    }
    pub unsafe fn cmd_bind_descriptor_sets2(
        &self,
        command_buffer: CommandBuffer,
        bind_descriptor_sets_info: &BindDescriptorSetsInfo,
    ) {
        unsafe { (self.cmd_bind_descriptor_sets2)(command_buffer, bind_descriptor_sets_info) }
    }
    pub unsafe fn cmd_push_constants2(
        &self,
        command_buffer: CommandBuffer,
        push_constants_info: &PushConstantsInfo,
    ) {
        unsafe { (self.cmd_push_constants2)(command_buffer, push_constants_info) }
    }
    pub unsafe fn cmd_push_descriptor_set2(
        &self,
        command_buffer: CommandBuffer,
        push_descriptor_set_info: &PushDescriptorSetInfo,
    ) {
        unsafe { (self.cmd_push_descriptor_set2)(command_buffer, push_descriptor_set_info) }
    }
    pub unsafe fn cmd_push_descriptor_set_with_template2(
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
}
