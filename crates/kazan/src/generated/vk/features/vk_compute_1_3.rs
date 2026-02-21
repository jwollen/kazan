#![allow(unused_imports)]
use crate::*;
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
use kazan_sys::{vk::*, *};
pub struct DeviceFn {
    cmd_set_event2: PFN_vkCmdSetEvent2,
    cmd_reset_event2: PFN_vkCmdResetEvent2,
    cmd_wait_events2: PFN_vkCmdWaitEvents2,
}
impl DeviceFn {
    pub unsafe fn load(
        load: impl Fn(&CStr) -> Option<PFN_vkVoidFunction>,
    ) -> core::result::Result<Self, LoadingError> {
        unsafe {
            Ok(Self {
                cmd_set_event2: transmute(load(c"vkCmdSetEvent2").ok_or(LoadingError)?),
                cmd_reset_event2: transmute(load(c"vkCmdResetEvent2").ok_or(LoadingError)?),
                cmd_wait_events2: transmute(load(c"vkCmdWaitEvents2").ok_or(LoadingError)?),
            })
        }
    }
}
impl DeviceFn {
    pub unsafe fn cmd_set_event2(
        &self,
        command_buffer: CommandBuffer,
        event: Event,
        dependency_info: &DependencyInfo,
    ) {
        unsafe { (self.cmd_set_event2)(command_buffer, event, dependency_info) }
    }
    pub unsafe fn cmd_reset_event2(
        &self,
        command_buffer: CommandBuffer,
        event: Event,
        stage_mask: PipelineStageFlags2,
    ) {
        unsafe { (self.cmd_reset_event2)(command_buffer, event, stage_mask) }
    }
    pub unsafe fn cmd_wait_events2(
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
}
