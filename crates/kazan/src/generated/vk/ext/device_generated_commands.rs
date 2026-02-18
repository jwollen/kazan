#![allow(unused_imports)]
use crate::*;
use kazan_sys::{vk::*, *};
use std::ffi::{c_char, c_int, c_void, CStr};
pub struct DeviceFn {
    cmd_execute_generated_commands_ext: PFN_vkCmdExecuteGeneratedCommandsEXT,
    cmd_preprocess_generated_commands_ext: PFN_vkCmdPreprocessGeneratedCommandsEXT,
    get_generated_commands_memory_requirements_ext: PFN_vkGetGeneratedCommandsMemoryRequirementsEXT,
    create_indirect_commands_layout_ext: PFN_vkCreateIndirectCommandsLayoutEXT,
    destroy_indirect_commands_layout_ext: PFN_vkDestroyIndirectCommandsLayoutEXT,
    create_indirect_execution_set_ext: PFN_vkCreateIndirectExecutionSetEXT,
    destroy_indirect_execution_set_ext: PFN_vkDestroyIndirectExecutionSetEXT,
    update_indirect_execution_set_pipeline_ext: PFN_vkUpdateIndirectExecutionSetPipelineEXT,
    update_indirect_execution_set_shader_ext: PFN_vkUpdateIndirectExecutionSetShaderEXT,
}
impl DeviceFn {
    pub unsafe fn cmd_execute_generated_commands_ext(
        &self,
        command_buffer: CommandBuffer,
        is_preprocessed: Bool32,
        generated_commands_info: &GeneratedCommandsInfoEXT,
    ) {
        unsafe {
            (self.cmd_execute_generated_commands_ext)(
                command_buffer,
                is_preprocessed,
                generated_commands_info,
            )
        }
    }
    pub unsafe fn cmd_preprocess_generated_commands_ext(
        &self,
        command_buffer: CommandBuffer,
        generated_commands_info: &GeneratedCommandsInfoEXT,
        state_command_buffer: CommandBuffer,
    ) {
        unsafe {
            (self.cmd_preprocess_generated_commands_ext)(
                command_buffer,
                generated_commands_info,
                state_command_buffer,
            )
        }
    }
    pub unsafe fn get_generated_commands_memory_requirements_ext(
        &self,
        device: Device,
        info: &GeneratedCommandsMemoryRequirementsInfoEXT,
        memory_requirements: &mut MemoryRequirements2,
    ) {
        unsafe {
            (self.get_generated_commands_memory_requirements_ext)(device, info, memory_requirements)
        }
    }
    pub unsafe fn create_indirect_commands_layout_ext(
        &self,
        device: Device,
        create_info: &IndirectCommandsLayoutCreateInfoEXT,
        allocator: &AllocationCallbacks,
        indirect_commands_layout: &mut IndirectCommandsLayoutEXT,
    ) -> Result {
        unsafe {
            (self.create_indirect_commands_layout_ext)(
                device,
                create_info,
                allocator,
                indirect_commands_layout,
            )
        }
    }
    pub unsafe fn destroy_indirect_commands_layout_ext(
        &self,
        device: Device,
        indirect_commands_layout: IndirectCommandsLayoutEXT,
        allocator: &AllocationCallbacks,
    ) {
        unsafe {
            (self.destroy_indirect_commands_layout_ext)(device, indirect_commands_layout, allocator)
        }
    }
    pub unsafe fn create_indirect_execution_set_ext(
        &self,
        device: Device,
        create_info: &IndirectExecutionSetCreateInfoEXT,
        allocator: &AllocationCallbacks,
        indirect_execution_set: &mut IndirectExecutionSetEXT,
    ) -> Result {
        unsafe {
            (self.create_indirect_execution_set_ext)(
                device,
                create_info,
                allocator,
                indirect_execution_set,
            )
        }
    }
    pub unsafe fn destroy_indirect_execution_set_ext(
        &self,
        device: Device,
        indirect_execution_set: IndirectExecutionSetEXT,
        allocator: &AllocationCallbacks,
    ) {
        unsafe {
            (self.destroy_indirect_execution_set_ext)(device, indirect_execution_set, allocator)
        }
    }
    pub unsafe fn update_indirect_execution_set_pipeline_ext(
        &self,
        device: Device,
        indirect_execution_set: IndirectExecutionSetEXT,
        execution_set_writes: &[WriteIndirectExecutionSetPipelineEXT],
    ) {
        unsafe {
            (self.update_indirect_execution_set_pipeline_ext)(
                device,
                indirect_execution_set,
                execution_set_writes.len().try_into().unwrap(),
                execution_set_writes.as_ptr() as _,
            )
        }
    }
    pub unsafe fn update_indirect_execution_set_shader_ext(
        &self,
        device: Device,
        indirect_execution_set: IndirectExecutionSetEXT,
        execution_set_writes: &[WriteIndirectExecutionSetShaderEXT],
    ) {
        unsafe {
            (self.update_indirect_execution_set_shader_ext)(
                device,
                indirect_execution_set,
                execution_set_writes.len().try_into().unwrap(),
                execution_set_writes.as_ptr() as _,
            )
        }
    }
}
