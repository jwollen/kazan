#![allow(unused_imports)]
use crate::*;
use core::ffi::{c_char, c_int, c_void, CStr};
use kazan_sys::{vk::*, *};
pub struct DeviceFn {
    cmd_push_descriptor_set: PFN_vkCmdPushDescriptorSet,
    cmd_push_descriptor_set_with_template: PFN_vkCmdPushDescriptorSetWithTemplate,
}
impl DeviceFn {
    pub unsafe fn cmd_push_descriptor_set_khr(
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
    pub unsafe fn cmd_push_descriptor_set_with_template_khr(
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
}
