#![allow(unused_imports)]
use crate::*;
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
use kazan_sys::{vk::*, *};
pub struct DeviceFn {
    cmd_write_buffer_marker_amd: PFN_vkCmdWriteBufferMarkerAMD,
    cmd_write_buffer_marker2_amd: Option<PFN_vkCmdWriteBufferMarker2AMD>,
}
impl DeviceFn {
    pub unsafe fn load(
        load: impl Fn(&CStr) -> Option<PFN_vkVoidFunction>,
    ) -> core::result::Result<Self, LoadingError> {
        unsafe {
            Ok(Self {
                cmd_write_buffer_marker_amd: transmute(
                    load(c"vkCmdWriteBufferMarkerAMD").ok_or(LoadingError)?,
                ),
                cmd_write_buffer_marker2_amd: transmute(load(c"vkCmdWriteBufferMarker2AMD")),
            })
        }
    }
}
impl DeviceFn {
    pub unsafe fn cmd_write_buffer_marker_amd(
        &self,
        command_buffer: CommandBuffer,
        pipeline_stage: PipelineStageFlags,
        dst_buffer: Buffer,
        dst_offset: DeviceSize,
        marker: u32,
    ) {
        unsafe {
            (self.cmd_write_buffer_marker_amd)(
                command_buffer,
                pipeline_stage,
                dst_buffer,
                dst_offset,
                marker,
            )
        }
    }
    pub unsafe fn cmd_write_buffer_marker2_amd(
        &self,
        command_buffer: CommandBuffer,
        stage: PipelineStageFlags2,
        dst_buffer: Buffer,
        dst_offset: DeviceSize,
        marker: u32,
    ) {
        unsafe {
            (self.cmd_write_buffer_marker2_amd.unwrap())(
                command_buffer,
                stage,
                dst_buffer,
                dst_offset,
                marker,
            )
        }
    }
}
