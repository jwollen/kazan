#![allow(unused_imports)]
use crate::*;
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
use kazan_sys::{vk::*, *};
pub struct DeviceFn {
    get_generated_commands_memory_requirements_ext: PFN_vkGetGeneratedCommandsMemoryRequirementsEXT,
    cmd_preprocess_generated_commands_ext: PFN_vkCmdPreprocessGeneratedCommandsEXT,
    cmd_execute_generated_commands_ext: PFN_vkCmdExecuteGeneratedCommandsEXT,
    create_indirect_commands_layout_ext: PFN_vkCreateIndirectCommandsLayoutEXT,
    destroy_indirect_commands_layout_ext: PFN_vkDestroyIndirectCommandsLayoutEXT,
    create_indirect_execution_set_ext: PFN_vkCreateIndirectExecutionSetEXT,
    destroy_indirect_execution_set_ext: PFN_vkDestroyIndirectExecutionSetEXT,
    update_indirect_execution_set_pipeline_ext: PFN_vkUpdateIndirectExecutionSetPipelineEXT,
    update_indirect_execution_set_shader_ext: PFN_vkUpdateIndirectExecutionSetShaderEXT,
}
impl DeviceFn {
    pub unsafe fn load(
        load: impl Fn(&CStr) -> Option<PFN_vkVoidFunction>,
    ) -> core::result::Result<Self, LoadingError> {
        unsafe {
            Ok(Self {
                get_generated_commands_memory_requirements_ext: transmute(
                    load(c"vkGetGeneratedCommandsMemoryRequirementsEXT").ok_or(LoadingError)?,
                ),
                cmd_preprocess_generated_commands_ext: transmute(
                    load(c"vkCmdPreprocessGeneratedCommandsEXT").ok_or(LoadingError)?,
                ),
                cmd_execute_generated_commands_ext: transmute(
                    load(c"vkCmdExecuteGeneratedCommandsEXT").ok_or(LoadingError)?,
                ),
                create_indirect_commands_layout_ext: transmute(
                    load(c"vkCreateIndirectCommandsLayoutEXT").ok_or(LoadingError)?,
                ),
                destroy_indirect_commands_layout_ext: transmute(
                    load(c"vkDestroyIndirectCommandsLayoutEXT").ok_or(LoadingError)?,
                ),
                create_indirect_execution_set_ext: transmute(
                    load(c"vkCreateIndirectExecutionSetEXT").ok_or(LoadingError)?,
                ),
                destroy_indirect_execution_set_ext: transmute(
                    load(c"vkDestroyIndirectExecutionSetEXT").ok_or(LoadingError)?,
                ),
                update_indirect_execution_set_pipeline_ext: transmute(
                    load(c"vkUpdateIndirectExecutionSetPipelineEXT").ok_or(LoadingError)?,
                ),
                update_indirect_execution_set_shader_ext: transmute(
                    load(c"vkUpdateIndirectExecutionSetShaderEXT").ok_or(LoadingError)?,
                ),
            })
        }
    }
}
impl DeviceFn {
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
    pub unsafe fn create_indirect_commands_layout_ext(
        &self,
        device: Device,
        create_info: &IndirectCommandsLayoutCreateInfoEXT,
        allocator: Option<&AllocationCallbacks>,
        indirect_commands_layout: &mut IndirectCommandsLayoutEXT,
    ) -> Result {
        unsafe {
            (self.create_indirect_commands_layout_ext)(
                device,
                create_info,
                allocator.to_raw_ptr(),
                indirect_commands_layout,
            )
        }
    }
    pub unsafe fn destroy_indirect_commands_layout_ext(
        &self,
        device: Device,
        indirect_commands_layout: IndirectCommandsLayoutEXT,
        allocator: Option<&AllocationCallbacks>,
    ) {
        unsafe {
            (self.destroy_indirect_commands_layout_ext)(
                device,
                indirect_commands_layout,
                allocator.to_raw_ptr(),
            )
        }
    }
    pub unsafe fn create_indirect_execution_set_ext(
        &self,
        device: Device,
        create_info: &IndirectExecutionSetCreateInfoEXT,
        allocator: Option<&AllocationCallbacks>,
        indirect_execution_set: &mut IndirectExecutionSetEXT,
    ) -> Result {
        unsafe {
            (self.create_indirect_execution_set_ext)(
                device,
                create_info,
                allocator.to_raw_ptr(),
                indirect_execution_set,
            )
        }
    }
    pub unsafe fn destroy_indirect_execution_set_ext(
        &self,
        device: Device,
        indirect_execution_set: IndirectExecutionSetEXT,
        allocator: Option<&AllocationCallbacks>,
    ) {
        unsafe {
            (self.destroy_indirect_execution_set_ext)(
                device,
                indirect_execution_set,
                allocator.to_raw_ptr(),
            )
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
