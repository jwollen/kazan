#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{c_char, c_int, c_void};
pub type PFN_vkCmdWriteBufferMarkerAMD = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    pipeline_stage: PipelineStageFlagBits,
    dst_buffer: Buffer,
    dst_offset: DeviceSize,
    marker: u32,
);
pub type PFN_vkCmdWriteBufferMarker2AMD = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    stage: PipelineStageFlags2,
    dst_buffer: Buffer,
    dst_offset: DeviceSize,
    marker: u32,
);
