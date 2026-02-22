#![allow(unused_imports)]
use crate::*;
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
use kazan_sys::{vk::*, *};
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
    ) -> core::result::Result<Self, LoadingError> {
        unsafe {
            Ok(Self {
                cmd_set_event2_khr: transmute(load(c"vkCmdSetEvent2KHR").ok_or(LoadingError)?),
                cmd_reset_event2_khr: transmute(load(c"vkCmdResetEvent2KHR").ok_or(LoadingError)?),
                cmd_wait_events2_khr: transmute(load(c"vkCmdWaitEvents2KHR").ok_or(LoadingError)?),
                cmd_pipeline_barrier2_khr: transmute(
                    load(c"vkCmdPipelineBarrier2KHR").ok_or(LoadingError)?,
                ),
                cmd_write_timestamp2_khr: transmute(
                    load(c"vkCmdWriteTimestamp2KHR").ok_or(LoadingError)?,
                ),
                queue_submit2_khr: transmute(load(c"vkQueueSubmit2KHR").ok_or(LoadingError)?),
            })
        }
    }
}
impl DeviceFn {
    pub unsafe fn cmd_set_event2_khr(
        &self,
        command_buffer: CommandBuffer,
        event: Event,
        dependency_info: &DependencyInfo,
    ) {
        unsafe { (self.cmd_set_event2_khr)(command_buffer, event, dependency_info) }
    }
    pub unsafe fn cmd_reset_event2_khr(
        &self,
        command_buffer: CommandBuffer,
        event: Event,
        stage_mask: PipelineStageFlags2,
    ) {
        unsafe { (self.cmd_reset_event2_khr)(command_buffer, event, stage_mask) }
    }
    pub unsafe fn cmd_wait_events2_khr(
        &self,
        command_buffer: CommandBuffer,
        events: &[Event],
        dependency_infos: &[DependencyInfo],
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
    pub unsafe fn cmd_pipeline_barrier2_khr(
        &self,
        command_buffer: CommandBuffer,
        dependency_info: &DependencyInfo,
    ) {
        unsafe { (self.cmd_pipeline_barrier2_khr)(command_buffer, dependency_info) }
    }
    pub unsafe fn cmd_write_timestamp2_khr(
        &self,
        command_buffer: CommandBuffer,
        stage: PipelineStageFlags2,
        query_pool: QueryPool,
        query: u32,
    ) {
        unsafe { (self.cmd_write_timestamp2_khr)(command_buffer, stage, query_pool, query) }
    }
    pub unsafe fn queue_submit2_khr(
        &self,
        queue: Queue,
        submits: &[SubmitInfo2],
        fence: Fence,
    ) -> crate::Result<()> {
        unsafe {
            result((self.queue_submit2_khr)(
                queue,
                submits.len().try_into().unwrap(),
                submits.as_ptr() as _,
                fence,
            ))
        }
    }
}
