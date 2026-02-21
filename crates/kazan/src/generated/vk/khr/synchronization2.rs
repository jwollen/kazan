#![allow(unused_imports)]
use crate::*;
use core::ffi::{c_char, c_int, c_void, CStr};
use kazan_sys::{vk::*, *};
pub struct DeviceFn {
    cmd_set_event2: PFN_vkCmdSetEvent2,
    cmd_reset_event2: PFN_vkCmdResetEvent2,
    cmd_wait_events2: PFN_vkCmdWaitEvents2,
    cmd_pipeline_barrier2: PFN_vkCmdPipelineBarrier2,
    cmd_write_timestamp2: PFN_vkCmdWriteTimestamp2,
    queue_submit2: PFN_vkQueueSubmit2,
}
impl DeviceFn {
    pub unsafe fn cmd_set_event2_khr(
        &self,
        command_buffer: CommandBuffer,
        event: Event,
        dependency_info: &DependencyInfo,
    ) {
        unsafe { (self.cmd_set_event2)(command_buffer, event, dependency_info) }
    }
    pub unsafe fn cmd_reset_event2_khr(
        &self,
        command_buffer: CommandBuffer,
        event: Event,
        stage_mask: PipelineStageFlags2,
    ) {
        unsafe { (self.cmd_reset_event2)(command_buffer, event, stage_mask) }
    }
    pub unsafe fn cmd_wait_events2_khr(
        &self,
        command_buffer: CommandBuffer,
        events: &[Event],
        dependency_infos: &[DependencyInfo],
    ) {
        unsafe {
            (self.cmd_wait_events2)(
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
        unsafe { (self.cmd_pipeline_barrier2)(command_buffer, dependency_info) }
    }
    pub unsafe fn cmd_write_timestamp2_khr(
        &self,
        command_buffer: CommandBuffer,
        stage: PipelineStageFlags2,
        query_pool: QueryPool,
        query: u32,
    ) {
        unsafe { (self.cmd_write_timestamp2)(command_buffer, stage, query_pool, query) }
    }
    pub unsafe fn queue_submit2_khr(
        &self,
        queue: Queue,
        submits: &[SubmitInfo2],
        fence: Fence,
    ) -> Result {
        unsafe {
            (self.queue_submit2)(
                queue,
                submits.len().try_into().unwrap(),
                submits.as_ptr() as _,
                fence,
            )
        }
    }
}
