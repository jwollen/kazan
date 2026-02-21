#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{c_char, c_int, c_void};
pub type PhysicalDeviceMaintenance6FeaturesKHR = PhysicalDeviceMaintenance6Features;
pub type PhysicalDeviceMaintenance6PropertiesKHR = PhysicalDeviceMaintenance6Properties;
pub type BindMemoryStatusKHR = BindMemoryStatus;
pub type BindDescriptorSetsInfoKHR = BindDescriptorSetsInfo;
pub type PushConstantsInfoKHR = PushConstantsInfo;
pub type PushDescriptorSetInfoKHR = PushDescriptorSetInfo;
pub type PushDescriptorSetWithTemplateInfoKHR = PushDescriptorSetWithTemplateInfo;
pub type PFN_vkCmdBindDescriptorSets2KHR = PFN_vkCmdBindDescriptorSets2;
pub type PFN_vkCmdPushConstants2KHR = PFN_vkCmdPushConstants2;
pub type PFN_vkCmdPushDescriptorSet2KHR = PFN_vkCmdPushDescriptorSet2;
pub type PFN_vkCmdPushDescriptorSetWithTemplate2KHR = PFN_vkCmdPushDescriptorSetWithTemplate2;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SetDescriptorBufferOffsetsInfoEXT {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub stage_flags: ShaderStageFlags,
    pub layout: PipelineLayout,
    pub first_set: u32,
    pub set_count: u32,
    pub p_buffer_indices: *const u32,
    pub p_offsets: *const DeviceSize,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct BindDescriptorBufferEmbeddedSamplersInfoEXT {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub stage_flags: ShaderStageFlags,
    pub layout: PipelineLayout,
    pub set: u32,
}
pub type PFN_vkCmdSetDescriptorBufferOffsets2EXT = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    p_set_descriptor_buffer_offsets_info: *const SetDescriptorBufferOffsetsInfoEXT,
);
pub type PFN_vkCmdBindDescriptorBufferEmbeddedSamplers2EXT = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    p_bind_descriptor_buffer_embedded_samplers_info: *const BindDescriptorBufferEmbeddedSamplersInfoEXT,
);
