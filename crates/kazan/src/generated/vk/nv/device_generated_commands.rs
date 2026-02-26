#![allow(unused_imports)]
use crate::*;
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
use kazan_sys::{vk::Result as VkResult, vk::*, *};
pub struct DeviceFn {
    get_generated_commands_memory_requirements_nv: PFN_vkGetGeneratedCommandsMemoryRequirementsNV,
    cmd_preprocess_generated_commands_nv: PFN_vkCmdPreprocessGeneratedCommandsNV,
    cmd_execute_generated_commands_nv: PFN_vkCmdExecuteGeneratedCommandsNV,
    cmd_bind_pipeline_shader_group_nv: PFN_vkCmdBindPipelineShaderGroupNV,
    create_indirect_commands_layout_nv: PFN_vkCreateIndirectCommandsLayoutNV,
    destroy_indirect_commands_layout_nv: PFN_vkDestroyIndirectCommandsLayoutNV,
}
impl DeviceFn {
    pub unsafe fn load(
        load: impl Fn(&CStr) -> Option<PFN_vkVoidFunction>,
    ) -> core::result::Result<Self, LoadingError> {
        unsafe {
            Ok(Self {
                get_generated_commands_memory_requirements_nv: transmute(
                    load(c"vkGetGeneratedCommandsMemoryRequirementsNV").ok_or(LoadingError)?,
                ),
                cmd_preprocess_generated_commands_nv: transmute(
                    load(c"vkCmdPreprocessGeneratedCommandsNV").ok_or(LoadingError)?,
                ),
                cmd_execute_generated_commands_nv: transmute(
                    load(c"vkCmdExecuteGeneratedCommandsNV").ok_or(LoadingError)?,
                ),
                cmd_bind_pipeline_shader_group_nv: transmute(
                    load(c"vkCmdBindPipelineShaderGroupNV").ok_or(LoadingError)?,
                ),
                create_indirect_commands_layout_nv: transmute(
                    load(c"vkCreateIndirectCommandsLayoutNV").ok_or(LoadingError)?,
                ),
                destroy_indirect_commands_layout_nv: transmute(
                    load(c"vkDestroyIndirectCommandsLayoutNV").ok_or(LoadingError)?,
                ),
            })
        }
    }
}
impl DeviceFn {
    pub unsafe fn get_generated_commands_memory_requirements_nv(
        &self,
        device: Device,
        info: &GeneratedCommandsMemoryRequirementsInfoNV<'_>,
    ) -> MemoryRequirements2<'_> {
        unsafe {
            let mut memory_requirements = core::mem::MaybeUninit::uninit();
            (self.get_generated_commands_memory_requirements_nv)(
                device,
                info,
                memory_requirements.as_mut_ptr(),
            );
            memory_requirements.assume_init()
        }
    }
    pub unsafe fn cmd_preprocess_generated_commands_nv(
        &self,
        command_buffer: CommandBuffer,
        generated_commands_info: &GeneratedCommandsInfoNV<'_>,
    ) {
        unsafe {
            (self.cmd_preprocess_generated_commands_nv)(command_buffer, generated_commands_info)
        }
    }
    pub unsafe fn cmd_execute_generated_commands_nv(
        &self,
        command_buffer: CommandBuffer,
        is_preprocessed: Bool32,
        generated_commands_info: &GeneratedCommandsInfoNV<'_>,
    ) {
        unsafe {
            (self.cmd_execute_generated_commands_nv)(
                command_buffer,
                is_preprocessed,
                generated_commands_info,
            )
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
    pub unsafe fn create_indirect_commands_layout_nv(
        &self,
        device: Device,
        create_info: &IndirectCommandsLayoutCreateInfoNV<'_>,
        allocator: Option<&AllocationCallbacks<'_>>,
    ) -> crate::Result<IndirectCommandsLayoutNV> {
        unsafe {
            let mut indirect_commands_layout = core::mem::MaybeUninit::uninit();
            let result = (self.create_indirect_commands_layout_nv)(
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
    pub unsafe fn destroy_indirect_commands_layout_nv(
        &self,
        device: Device,
        indirect_commands_layout: IndirectCommandsLayoutNV,
        allocator: Option<&AllocationCallbacks<'_>>,
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
