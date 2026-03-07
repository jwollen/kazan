#![allow(unused_imports)]
use crate::{vk::Result as VkResult, vk::*, *};
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;

pub(super) mod defs {
    #![allow(non_camel_case_types, unused_imports)]
    use crate::{vk::*, *};
    use core::ffi::{CStr, c_char, c_int, c_void};
    use core::fmt;
    use core::marker::PhantomData;

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkDescriptorUpdateTemplateTypeKHR.html>
    pub type DescriptorUpdateTemplateTypeKHR = DescriptorUpdateTemplateType;
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkDescriptorUpdateTemplateKHR.html>
    pub type DescriptorUpdateTemplateKHR = DescriptorUpdateTemplate;
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkDescriptorUpdateTemplateEntryKHR.html>
    pub type DescriptorUpdateTemplateEntryKHR = DescriptorUpdateTemplateEntry;
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkDescriptorUpdateTemplateCreateInfoKHR.html>
    pub type DescriptorUpdateTemplateCreateInfoKHR<'a> = DescriptorUpdateTemplateCreateInfo<'a>;
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkDescriptorUpdateTemplateCreateFlagsKHR.html>
    pub type DescriptorUpdateTemplateCreateFlagsKHR = DescriptorUpdateTemplateCreateFlags;
    pub type PFN_vkCreateDescriptorUpdateTemplateKHR = PFN_vkCreateDescriptorUpdateTemplate;
    pub type PFN_vkDestroyDescriptorUpdateTemplateKHR = PFN_vkDestroyDescriptorUpdateTemplate;
    pub type PFN_vkUpdateDescriptorSetWithTemplateKHR = PFN_vkUpdateDescriptorSetWithTemplate;
}

pub struct DeviceFn {
    create_descriptor_update_template_khr: PFN_vkCreateDescriptorUpdateTemplate,
    destroy_descriptor_update_template_khr: PFN_vkDestroyDescriptorUpdateTemplate,
    update_descriptor_set_with_template_khr: PFN_vkUpdateDescriptorSetWithTemplate,
    cmd_push_descriptor_set_with_template_khr: Option<PFN_vkCmdPushDescriptorSetWithTemplate>,
}

impl DeviceFn {
    pub unsafe fn load(
        load: impl Fn(&CStr) -> Option<PFN_vkVoidFunction>,
    ) -> core::result::Result<Self, MissingEntryPointError> {
        unsafe {
            Ok(Self {
                create_descriptor_update_template_khr: transmute(
                    load(c"vkCreateDescriptorUpdateTemplateKHR").ok_or(MissingEntryPointError)?,
                ),
                destroy_descriptor_update_template_khr: transmute(
                    load(c"vkDestroyDescriptorUpdateTemplateKHR").ok_or(MissingEntryPointError)?,
                ),
                update_descriptor_set_with_template_khr: transmute(
                    load(c"vkUpdateDescriptorSetWithTemplateKHR").ok_or(MissingEntryPointError)?,
                ),
                cmd_push_descriptor_set_with_template_khr: transmute(load(
                    c"vkCmdPushDescriptorSetWithTemplateKHR",
                )),
            })
        }
    }
}

impl DeviceFn {
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCreateDescriptorUpdateTemplateKHR.html>
    pub unsafe fn create_descriptor_update_template_khr(
        &self,
        device: Device,
        create_info: &DescriptorUpdateTemplateCreateInfo<'_>,
        allocator: Option<&AllocationCallbacks<'_>>,
    ) -> crate::Result<DescriptorUpdateTemplate> {
        unsafe {
            let mut descriptor_update_template = core::mem::MaybeUninit::uninit();
            let result = (self.create_descriptor_update_template_khr)(
                device,
                create_info,
                allocator.to_raw_ptr(),
                descriptor_update_template.as_mut_ptr(),
            );

            match result {
                VkResult::SUCCESS => Ok(descriptor_update_template.assume_init()),
                err => Err(err),
            }
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkDestroyDescriptorUpdateTemplateKHR.html>
    pub unsafe fn destroy_descriptor_update_template_khr(
        &self,
        device: Device,
        descriptor_update_template: DescriptorUpdateTemplate,
        allocator: Option<&AllocationCallbacks<'_>>,
    ) {
        unsafe {
            (self.destroy_descriptor_update_template_khr)(
                device,
                descriptor_update_template,
                allocator.to_raw_ptr(),
            )
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkUpdateDescriptorSetWithTemplateKHR.html>
    pub unsafe fn update_descriptor_set_with_template_khr(
        &self,
        device: Device,
        descriptor_set: DescriptorSet,
        descriptor_update_template: DescriptorUpdateTemplate,
        data: *const c_void,
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

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdPushDescriptorSetWithTemplateKHR.html>
    pub unsafe fn cmd_push_descriptor_set_with_template_khr(
        &self,
        command_buffer: CommandBuffer,
        descriptor_update_template: DescriptorUpdateTemplate,
        layout: PipelineLayout,
        set: u32,
        data: *const c_void,
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
