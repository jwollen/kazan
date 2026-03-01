#![allow(unused_imports)]
use crate::{vk::Result as VkResult, vk::*, *};
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
pub(super) mod defs {
    #![allow(non_camel_case_types, unused_imports)]
    use crate::{vk::*, *};
    use core::ffi::{CStr, c_char, c_int, c_void};
    use core::marker::PhantomData;
    pub type PhysicalDevicePushDescriptorPropertiesKHR<'a> =
        PhysicalDevicePushDescriptorProperties<'a>;
    pub type PFN_vkCmdPushDescriptorSetKHR = PFN_vkCmdPushDescriptorSet;
    pub type PFN_vkCmdPushDescriptorSetWithTemplateKHR = PFN_vkCmdPushDescriptorSetWithTemplate;
}
pub struct DeviceFn {
    cmd_push_descriptor_set_khr: PFN_vkCmdPushDescriptorSet,
    cmd_push_descriptor_set_with_template_khr: Option<PFN_vkCmdPushDescriptorSetWithTemplate>,
}
impl DeviceFn {
    pub unsafe fn load(
        load: impl Fn(&CStr) -> Option<PFN_vkVoidFunction>,
    ) -> core::result::Result<Self, LoadingError> {
        unsafe {
            Ok(Self {
                cmd_push_descriptor_set_khr: transmute(
                    load(c"vkCmdPushDescriptorSetKHR").ok_or(LoadingError)?,
                ),
                cmd_push_descriptor_set_with_template_khr: transmute(load(
                    c"vkCmdPushDescriptorSetWithTemplateKHR",
                )),
            })
        }
    }
}
impl DeviceFn {
    pub unsafe fn cmd_push_descriptor_set_khr(
        &self,
        command_buffer: CommandBuffer,
        pipeline_bind_point: PipelineBindPoint,
        layout: PipelineLayout,
        set: u32,
        descriptor_writes: &[WriteDescriptorSet<'_>],
    ) {
        unsafe {
            (self.cmd_push_descriptor_set_khr)(
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
