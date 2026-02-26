#![allow(unused_imports)]
use crate::*;
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
use kazan_sys::{vk::Result as VkResult, vk::*, *};
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
        info: &GeneratedCommandsMemoryRequirementsInfoEXT<'_>,
    ) -> MemoryRequirements2<'_> {
        unsafe {
            let mut memory_requirements = core::mem::MaybeUninit::uninit();
            (self.get_generated_commands_memory_requirements_ext)(
                device,
                info,
                memory_requirements.as_mut_ptr(),
            );
            memory_requirements.assume_init()
        }
    }
    pub unsafe fn cmd_preprocess_generated_commands_ext(
        &self,
        command_buffer: CommandBuffer,
        generated_commands_info: &GeneratedCommandsInfoEXT<'_>,
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
        generated_commands_info: &GeneratedCommandsInfoEXT<'_>,
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
        create_info: &IndirectCommandsLayoutCreateInfoEXT<'_>,
        allocator: Option<&AllocationCallbacks<'_>>,
    ) -> crate::Result<IndirectCommandsLayoutEXT> {
        unsafe {
            let mut indirect_commands_layout = core::mem::MaybeUninit::uninit();
            let result = (self.create_indirect_commands_layout_ext)(
                device,
                create_info,
                allocator.to_raw_ptr(),
                indirect_commands_layout.as_mut_ptr(),
            );

            match result {
                VkResult::SUCCESS => Ok(indirect_commands_layout.assume_init()),
                err => Err(err),
            }
        }
    }
    pub unsafe fn destroy_indirect_commands_layout_ext(
        &self,
        device: Device,
        indirect_commands_layout: IndirectCommandsLayoutEXT,
        allocator: Option<&AllocationCallbacks<'_>>,
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
        create_info: &IndirectExecutionSetCreateInfoEXT<'_>,
        allocator: Option<&AllocationCallbacks<'_>>,
    ) -> crate::Result<IndirectExecutionSetEXT> {
        unsafe {
            let mut indirect_execution_set = core::mem::MaybeUninit::uninit();
            let result = (self.create_indirect_execution_set_ext)(
                device,
                create_info,
                allocator.to_raw_ptr(),
                indirect_execution_set.as_mut_ptr(),
            );

            match result {
                VkResult::SUCCESS => Ok(indirect_execution_set.assume_init()),
                err => Err(err),
            }
        }
    }
    pub unsafe fn destroy_indirect_execution_set_ext(
        &self,
        device: Device,
        indirect_execution_set: IndirectExecutionSetEXT,
        allocator: Option<&AllocationCallbacks<'_>>,
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
        execution_set_writes: &[WriteIndirectExecutionSetPipelineEXT<'_>],
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
        execution_set_writes: &[WriteIndirectExecutionSetShaderEXT<'_>],
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
