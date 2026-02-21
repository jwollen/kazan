#![allow(unused_imports)]
use crate::*;
use core::ffi::{c_char, c_int, c_void, CStr};
use kazan_sys::{vk::*, *};
pub struct DeviceFn {
    create_descriptor_update_template: PFN_vkCreateDescriptorUpdateTemplate,
    destroy_descriptor_update_template: PFN_vkDestroyDescriptorUpdateTemplate,
    update_descriptor_set_with_template: PFN_vkUpdateDescriptorSetWithTemplate,
    cmd_push_descriptor_set_with_template: PFN_vkCmdPushDescriptorSetWithTemplate,
}
impl DeviceFn {
    pub unsafe fn create_descriptor_update_template_khr(
        &self,
        device: Device,
        create_info: &DescriptorUpdateTemplateCreateInfo,
        allocator: Option<&AllocationCallbacks>,
        descriptor_update_template: &mut DescriptorUpdateTemplate,
    ) -> Result {
        unsafe {
            (self.create_descriptor_update_template)(
                device,
                create_info,
                allocator.to_raw_ptr(),
                descriptor_update_template,
            )
        }
    }
    pub unsafe fn destroy_descriptor_update_template_khr(
        &self,
        device: Device,
        descriptor_update_template: DescriptorUpdateTemplate,
        allocator: Option<&AllocationCallbacks>,
    ) {
        unsafe {
            (self.destroy_descriptor_update_template)(
                device,
                descriptor_update_template,
                allocator.to_raw_ptr(),
            )
        }
    }
    pub unsafe fn update_descriptor_set_with_template_khr(
        &self,
        device: Device,
        descriptor_set: DescriptorSet,
        descriptor_update_template: DescriptorUpdateTemplate,
        data: &c_void,
    ) {
        unsafe {
            (self.update_descriptor_set_with_template)(
                device,
                descriptor_set,
                descriptor_update_template,
                data,
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
