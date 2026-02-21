#![allow(unused_imports)]
use crate::*;
use core::ffi::{c_char, c_int, c_void, CStr};
use kazan_sys::{vk::*, *};
pub struct DeviceFn {
    cmd_dispatch_base: PFN_vkCmdDispatchBase,
    create_descriptor_update_template: PFN_vkCreateDescriptorUpdateTemplate,
    destroy_descriptor_update_template: PFN_vkDestroyDescriptorUpdateTemplate,
    update_descriptor_set_with_template: PFN_vkUpdateDescriptorSetWithTemplate,
    get_descriptor_set_layout_support: PFN_vkGetDescriptorSetLayoutSupport,
    create_sampler_ycbcr_conversion: PFN_vkCreateSamplerYcbcrConversion,
    destroy_sampler_ycbcr_conversion: PFN_vkDestroySamplerYcbcrConversion,
}
impl DeviceFn {
    pub unsafe fn cmd_dispatch_base(
        &self,
        command_buffer: CommandBuffer,
        base_group_x: u32,
        base_group_y: u32,
        base_group_z: u32,
        group_count_x: u32,
        group_count_y: u32,
        group_count_z: u32,
    ) {
        unsafe {
            (self.cmd_dispatch_base)(
                command_buffer,
                base_group_x,
                base_group_y,
                base_group_z,
                group_count_x,
                group_count_y,
                group_count_z,
            )
        }
    }
    pub unsafe fn create_descriptor_update_template(
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
    pub unsafe fn destroy_descriptor_update_template(
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
    pub unsafe fn update_descriptor_set_with_template(
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
    pub unsafe fn get_descriptor_set_layout_support(
        &self,
        device: Device,
        create_info: &DescriptorSetLayoutCreateInfo,
        support: &mut DescriptorSetLayoutSupport,
    ) {
        unsafe { (self.get_descriptor_set_layout_support)(device, create_info, support) }
    }
    pub unsafe fn create_sampler_ycbcr_conversion(
        &self,
        device: Device,
        create_info: &SamplerYcbcrConversionCreateInfo,
        allocator: Option<&AllocationCallbacks>,
        ycbcr_conversion: &mut SamplerYcbcrConversion,
    ) -> Result {
        unsafe {
            (self.create_sampler_ycbcr_conversion)(
                device,
                create_info,
                allocator.to_raw_ptr(),
                ycbcr_conversion,
            )
        }
    }
    pub unsafe fn destroy_sampler_ycbcr_conversion(
        &self,
        device: Device,
        ycbcr_conversion: SamplerYcbcrConversion,
        allocator: Option<&AllocationCallbacks>,
    ) {
        unsafe {
            (self.destroy_sampler_ycbcr_conversion)(
                device,
                ycbcr_conversion,
                allocator.to_raw_ptr(),
            )
        }
    }
}
