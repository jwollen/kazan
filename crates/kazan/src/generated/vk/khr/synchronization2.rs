#![allow(unused_imports)]
use crate::{vk::Result as VkResult, vk::*, *};
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;

pub const EXTENSION_NAME: &CStr = c"VK_KHR_synchronization2";

pub(super) mod defs {
    #![allow(non_camel_case_types, unused_imports)]
    use crate::{vk::*, *};
    use core::ffi::{CStr, c_char, c_int, c_void};
    use core::fmt;
    use core::marker::PhantomData;

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkMemoryBarrier2KHR.html>
    pub type MemoryBarrier2KHR<'a> = MemoryBarrier2<'a>;
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkImageMemoryBarrier2KHR.html>
    pub type ImageMemoryBarrier2KHR<'a> = ImageMemoryBarrier2<'a>;
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkBufferMemoryBarrier2KHR.html>
    pub type BufferMemoryBarrier2KHR<'a> = BufferMemoryBarrier2<'a>;
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkDependencyInfoKHR.html>
    pub type DependencyInfoKHR<'a> = DependencyInfo<'a>;
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkSemaphoreSubmitInfoKHR.html>
    pub type SemaphoreSubmitInfoKHR<'a> = SemaphoreSubmitInfo<'a>;
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkCommandBufferSubmitInfoKHR.html>
    pub type CommandBufferSubmitInfoKHR<'a> = CommandBufferSubmitInfo<'a>;
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkSubmitInfo2KHR.html>
    pub type SubmitInfo2KHR<'a> = SubmitInfo2<'a>;
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPhysicalDeviceSynchronization2FeaturesKHR.html>
    pub type PhysicalDeviceSynchronization2FeaturesKHR<'a> =
        PhysicalDeviceSynchronization2Features<'a>;
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkAccessFlags2KHR.html>
    pub type AccessFlags2KHR = AccessFlags2;
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPipelineStageFlags2KHR.html>
    pub type PipelineStageFlags2KHR = PipelineStageFlags2;
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkSubmitFlagsKHR.html>
    pub type SubmitFlagsKHR = SubmitFlags;
    pub type PFN_vkCmdSetEvent2KHR = PFN_vkCmdSetEvent2;
    pub type PFN_vkCmdResetEvent2KHR = PFN_vkCmdResetEvent2;
    pub type PFN_vkCmdWaitEvents2KHR = PFN_vkCmdWaitEvents2;
    pub type PFN_vkCmdPipelineBarrier2KHR = PFN_vkCmdPipelineBarrier2;
    pub type PFN_vkQueueSubmit2KHR = PFN_vkQueueSubmit2;
    pub type PFN_vkCmdWriteTimestamp2KHR = PFN_vkCmdWriteTimestamp2;
}

pub struct DeviceFn {
    cmd_set_event2_khr: PFN_vkCmdSetEvent2,
    cmd_reset_event2_khr: PFN_vkCmdResetEvent2,
    cmd_wait_events2_khr: PFN_vkCmdWaitEvents2,
    cmd_pipeline_barrier2_khr: PFN_vkCmdPipelineBarrier2,
    cmd_write_timestamp2_khr: PFN_vkCmdWriteTimestamp2,
    queue_submit2_khr: PFN_vkQueueSubmit2,
}

impl DeviceFn {
    pub unsafe fn load(
        load: impl Fn(&CStr) -> Option<PFN_vkVoidFunction>,
    ) -> core::result::Result<Self, MissingEntryPointError> {
        unsafe {
            Ok(Self {
                cmd_set_event2_khr: transmute(
                    load(c"vkCmdSetEvent2KHR").ok_or(MissingEntryPointError)?,
                ),
                cmd_reset_event2_khr: transmute(
                    load(c"vkCmdResetEvent2KHR").ok_or(MissingEntryPointError)?,
                ),
                cmd_wait_events2_khr: transmute(
                    load(c"vkCmdWaitEvents2KHR").ok_or(MissingEntryPointError)?,
                ),
                cmd_pipeline_barrier2_khr: transmute(
                    load(c"vkCmdPipelineBarrier2KHR").ok_or(MissingEntryPointError)?,
                ),
                cmd_write_timestamp2_khr: transmute(
                    load(c"vkCmdWriteTimestamp2KHR").ok_or(MissingEntryPointError)?,
                ),
                queue_submit2_khr: transmute(
                    load(c"vkQueueSubmit2KHR").ok_or(MissingEntryPointError)?,
                ),
            })
        }
    }
}

impl DeviceFn {
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdSetEvent2KHR.html>
    pub unsafe fn cmd_set_event2_khr(
        &self,
        command_buffer: CommandBuffer,
        event: Event,
        dependency_info: &DependencyInfo<'_>,
    ) {
        unsafe { (self.cmd_set_event2_khr)(command_buffer, event, dependency_info) }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdResetEvent2KHR.html>
    pub unsafe fn cmd_reset_event2_khr(
        &self,
        command_buffer: CommandBuffer,
        event: Event,
        stage_mask: PipelineStageFlags2,
    ) {
        unsafe { (self.cmd_reset_event2_khr)(command_buffer, event, stage_mask) }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdWaitEvents2KHR.html>
    pub unsafe fn cmd_wait_events2_khr(
        &self,
        command_buffer: CommandBuffer,
        events: &[Event],
        dependency_infos: &[DependencyInfo<'_>],
    ) {
        unsafe {
            (self.cmd_wait_events2_khr)(
                command_buffer,
                events.len().try_into().unwrap(),
                events.as_ptr() as _,
                dependency_infos.as_ptr() as _,
            )
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdPipelineBarrier2KHR.html>
    pub unsafe fn cmd_pipeline_barrier2_khr(
        &self,
        command_buffer: CommandBuffer,
        dependency_info: &DependencyInfo<'_>,
    ) {
        unsafe { (self.cmd_pipeline_barrier2_khr)(command_buffer, dependency_info) }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdWriteTimestamp2KHR.html>
    pub unsafe fn cmd_write_timestamp2_khr(
        &self,
        command_buffer: CommandBuffer,
        stage: PipelineStageFlags2,
        query_pool: QueryPool,
        query: u32,
    ) {
        unsafe { (self.cmd_write_timestamp2_khr)(command_buffer, stage, query_pool, query) }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkQueueSubmit2KHR.html>
    pub unsafe fn queue_submit2_khr(
        &self,
        queue: Queue,
        submits: &[SubmitInfo2<'_>],
        fence: Fence,
    ) -> crate::Result<()> {
        unsafe {
            let result = (self.queue_submit2_khr)(
                queue,
                submits.len().try_into().unwrap(),
                submits.as_ptr() as _,
                fence,
            );

            match result {
                VkResult::SUCCESS => Ok(()),
                err => Err(err),
            }
        }
    }
}
