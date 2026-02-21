#![allow(unused_imports)]
use crate::*;
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
use kazan_sys::{vk::*, *};
pub struct DeviceFn {
    create_descriptor_update_template_khr: PFN_vkCreateDescriptorUpdateTemplate,
    destroy_descriptor_update_template_khr: PFN_vkDestroyDescriptorUpdateTemplate,
    update_descriptor_set_with_template_khr: PFN_vkUpdateDescriptorSetWithTemplate,
    cmd_push_descriptor_set_with_template_khr: Option<PFN_vkCmdPushDescriptorSetWithTemplate>,
}
impl DeviceFn {
    pub unsafe fn load(
        load: impl Fn(&CStr) -> Option<PFN_vkVoidFunction>,
    ) -> core::result::Result<Self, LoadingError> {
        unsafe {
            Ok(Self {
                create_descriptor_update_template_khr: transmute(
                    load(c"vkCreateDescriptorUpdateTemplateKHR").ok_or(LoadingError)?,
                ),
                destroy_descriptor_update_template_khr: transmute(
                    load(c"vkDestroyDescriptorUpdateTemplateKHR").ok_or(LoadingError)?,
                ),
                update_descriptor_set_with_template_khr: transmute(
                    load(c"vkUpdateDescriptorSetWithTemplateKHR").ok_or(LoadingError)?,
                ),
                cmd_push_descriptor_set_with_template_khr: transmute(load(
                    c"vkCmdPushDescriptorSetWithTemplateKHR",
                )),
            })
        }
    }
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
            (self.create_descriptor_update_template_khr)(
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
            (self.destroy_descriptor_update_template_khr)(
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
            (self.update_descriptor_set_with_template_khr)(
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
            (self.cmd_push_descriptor_set_with_template_khr.unwrap())(
                command_buffer,
                descriptor_update_template,
                layout,
                set,
                data,
            )
        }
    }
}
