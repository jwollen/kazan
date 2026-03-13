//! <https://registry.khronos.org/vulkan/specs/latest/man/html/VK_KHR_push_descriptor.html>
#![allow(unused_imports)]
use crate::{vk::Result as VkResult, vk::*, *};
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
use core::ptr;

pub const EXTENSION_NAME: &CStr = c"VK_KHR_push_descriptor";

pub(super) mod defs {
    #![allow(non_camel_case_types, unused_imports)]
    use crate::{vk::*, *};
    use core::ffi::{CStr, c_char, c_int, c_void};
    use core::fmt;
    use core::marker::PhantomData;
    use core::ptr;

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPhysicalDevicePushDescriptorPropertiesKHR.html>
    pub type PhysicalDevicePushDescriptorPropertiesKHR<'a> =
        PhysicalDevicePushDescriptorProperties<'a>;
    pub type PFN_vkCmdPushDescriptorSetKHR = PFN_vkCmdPushDescriptorSet;
    pub type PFN_vkCmdPushDescriptorSetWithTemplateKHR = PFN_vkCmdPushDescriptorSetWithTemplate;
}

#[cfg(feature = "ffi")]
pub(super) mod ffi {
    #![allow(non_camel_case_types)]
    use super::defs::*;

    pub type VkPhysicalDevicePushDescriptorPropertiesKHR =
        PhysicalDevicePushDescriptorPropertiesKHR<'static>;
}

pub struct DeviceFn {
    cmd_push_descriptor_set: PFN_vkCmdPushDescriptorSet,
    cmd_push_descriptor_set_with_template: Option<PFN_vkCmdPushDescriptorSetWithTemplate>,
}

impl LoadDeviceFn for DeviceFn {
    unsafe fn load_with(
        load: impl Fn(&CStr) -> Option<PFN_vkVoidFunction>,
    ) -> core::result::Result<Self, MissingEntryPointError> {
        unsafe {
            Ok(Self {
                cmd_push_descriptor_set: transmute(
                    load(c"vkCmdPushDescriptorSetKHR").ok_or(MissingEntryPointError)?,
                ),
                cmd_push_descriptor_set_with_template: transmute(load(
                    c"vkCmdPushDescriptorSetWithTemplateKHR",
                )),
            })
        }
    }
}

impl DeviceFn {
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdPushDescriptorSetKHR.html>
    #[inline]
    pub unsafe fn cmd_push_descriptor_set(
        &self,
        command_buffer: CommandBuffer,
        pipeline_bind_point: PipelineBindPoint,
        layout: PipelineLayout,
        set: u32,
        descriptor_writes: &[WriteDescriptorSet<'_>],
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

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdPushDescriptorSetWithTemplateKHR.html>
    #[inline]
    pub unsafe fn cmd_push_descriptor_set_with_template(
        &self,
        command_buffer: CommandBuffer,
        descriptor_update_template: DescriptorUpdateTemplate,
        layout: PipelineLayout,
        set: u32,
        data: *const c_void,
    ) {
        unsafe {
            (self.cmd_push_descriptor_set_with_template.unwrap())(
                command_buffer,
                descriptor_update_template,
                layout,
                set,
                data,
            )
        }
    }
}
