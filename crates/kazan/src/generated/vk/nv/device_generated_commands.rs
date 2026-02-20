#![allow(unused_imports)]
use crate::*;
use kazan_sys::{vk::*, *};
use std::ffi::{c_char, c_int, c_void, CStr};
pub struct DeviceFn {
    cmd_execute_generated_commands_nv: PFN_vkCmdExecuteGeneratedCommandsNV,
    cmd_preprocess_generated_commands_nv: PFN_vkCmdPreprocessGeneratedCommandsNV,
    cmd_bind_pipeline_shader_group_nv: PFN_vkCmdBindPipelineShaderGroupNV,
    get_generated_commands_memory_requirements_nv: PFN_vkGetGeneratedCommandsMemoryRequirementsNV,
    create_indirect_commands_layout_nv: PFN_vkCreateIndirectCommandsLayoutNV,
    destroy_indirect_commands_layout_nv: PFN_vkDestroyIndirectCommandsLayoutNV,
}
impl DeviceFn {
    pub unsafe fn cmd_execute_generated_commands_nv(
        &self,
        command_buffer: CommandBuffer,
        is_preprocessed: Bool32,
        generated_commands_info: &GeneratedCommandsInfoNV,
    ) {
        unsafe {
            (self.cmd_execute_generated_commands_nv)(
                command_buffer,
                is_preprocessed,
                generated_commands_info,
            )
        }
    }
    pub unsafe fn cmd_preprocess_generated_commands_nv(
        &self,
        command_buffer: CommandBuffer,
        generated_commands_info: &GeneratedCommandsInfoNV,
    ) {
        unsafe {
            (self.cmd_preprocess_generated_commands_nv)(command_buffer, generated_commands_info)
        }
    }
    pub unsafe fn cmd_bind_pipeline_shader_group_nv(
        &self,
        command_buffer: CommandBuffer,
        pipeline_bind_point: PipelineBindPoint,
        pipeline: Pipeline,
        group_index: u32,
    ) {
        unsafe {
            (self.cmd_bind_pipeline_shader_group_nv)(
                command_buffer,
                pipeline_bind_point,
                pipeline,
                group_index,
            )
        }
    }
    pub unsafe fn get_generated_commands_memory_requirements_nv(
        &self,
        device: Device,
        info: &GeneratedCommandsMemoryRequirementsInfoNV,
        memory_requirements: &mut MemoryRequirements2,
    ) {
        unsafe {
            (self.get_generated_commands_memory_requirements_nv)(device, info, memory_requirements)
        }
    }
    pub unsafe fn create_indirect_commands_layout_nv(
        &self,
        device: Device,
        create_info: &IndirectCommandsLayoutCreateInfoNV,
        allocator: Option<&AllocationCallbacks>,
        indirect_commands_layout: &mut IndirectCommandsLayoutNV,
    ) -> Result {
        unsafe {
            (self.create_indirect_commands_layout_nv)(
                device,
                create_info,
                allocator.to_raw_ptr(),
                indirect_commands_layout,
            )
        }
    }
    pub unsafe fn destroy_indirect_commands_layout_nv(
        &self,
        device: Device,
        indirect_commands_layout: IndirectCommandsLayoutNV,
        allocator: Option<&AllocationCallbacks>,
    ) {
        unsafe {
            (self.destroy_indirect_commands_layout_nv)(
                device,
                indirect_commands_layout,
                allocator.to_raw_ptr(),
            )
        }
    }
}
