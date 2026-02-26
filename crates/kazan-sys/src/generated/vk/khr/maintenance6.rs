#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{c_char, c_int, c_void};
use core::marker::PhantomData;
pub type PhysicalDeviceMaintenance6FeaturesKHR<'a> = PhysicalDeviceMaintenance6Features<'a>;
pub type PhysicalDeviceMaintenance6PropertiesKHR<'a> = PhysicalDeviceMaintenance6Properties<'a>;
pub type BindMemoryStatusKHR<'a> = BindMemoryStatus<'a>;
pub type BindDescriptorSetsInfoKHR<'a> = BindDescriptorSetsInfo<'a>;
pub type PushConstantsInfoKHR<'a> = PushConstantsInfo<'a>;
pub type PushDescriptorSetInfoKHR<'a> = PushDescriptorSetInfo<'a>;
pub type PushDescriptorSetWithTemplateInfoKHR<'a> = PushDescriptorSetWithTemplateInfo<'a>;
pub type PFN_vkCmdBindDescriptorSets2KHR = PFN_vkCmdBindDescriptorSets2;
pub type PFN_vkCmdPushConstants2KHR = PFN_vkCmdPushConstants2;
pub type PFN_vkCmdPushDescriptorSet2KHR = PFN_vkCmdPushDescriptorSet2;
pub type PFN_vkCmdPushDescriptorSetWithTemplate2KHR = PFN_vkCmdPushDescriptorSetWithTemplate2;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SetDescriptorBufferOffsetsInfoEXT<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub stage_flags: ShaderStageFlags,
    pub layout: PipelineLayout,
    pub first_set: u32,
    pub set_count: u32,
    pub p_buffer_indices: *const u32,
    pub p_offsets: *const DeviceSize,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for SetDescriptorBufferOffsetsInfoEXT<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::SET_DESCRIPTOR_BUFFER_OFFSETS_INFO_EXT,
            p_next: core::ptr::null(),
            stage_flags: Default::default(),
            layout: Default::default(),
            first_set: Default::default(),
            set_count: Default::default(),
            p_buffer_indices: core::ptr::null(),
            p_offsets: core::ptr::null(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct BindDescriptorBufferEmbeddedSamplersInfoEXT<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub stage_flags: ShaderStageFlags,
    pub layout: PipelineLayout,
    pub set: u32,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for BindDescriptorBufferEmbeddedSamplersInfoEXT<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::BIND_DESCRIPTOR_BUFFER_EMBEDDED_SAMPLERS_INFO_EXT,
            p_next: core::ptr::null(),
            stage_flags: Default::default(),
            layout: Default::default(),
            set: Default::default(),
            _marker: PhantomData,
        }
    }
}
pub type PFN_vkCmdSetDescriptorBufferOffsets2EXT = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    p_set_descriptor_buffer_offsets_info: *const SetDescriptorBufferOffsetsInfoEXT<'_>,
);
pub type PFN_vkCmdBindDescriptorBufferEmbeddedSamplers2EXT = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    p_bind_descriptor_buffer_embedded_samplers_info: *const BindDescriptorBufferEmbeddedSamplersInfoEXT<'_>,
);
