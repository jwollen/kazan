#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{c_char, c_int, c_void};
#[repr(C)]
pub struct QueueFamilyCheckpointPropertiesNV {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub checkpoint_execution_stage_mask: PipelineStageFlags,
}
#[repr(C)]
pub struct CheckpointDataNV {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub stage: PipelineStageFlagBits,
    pub p_checkpoint_marker: *mut c_void,
}
#[repr(C)]
pub struct QueueFamilyCheckpointProperties2NV {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub checkpoint_execution_stage_mask: PipelineStageFlags2,
}
#[repr(C)]
pub struct CheckpointData2NV {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub stage: PipelineStageFlags2,
    pub p_checkpoint_marker: *mut c_void,
}
pub type PFN_vkCmdSetCheckpointNV =
    unsafe extern "system" fn(command_buffer: CommandBuffer, p_checkpoint_marker: *const c_void);
pub type PFN_vkGetQueueCheckpointDataNV = unsafe extern "system" fn(
    queue: Queue,
    p_checkpoint_data_count: *mut u32,
    p_checkpoint_data: *mut CheckpointDataNV,
);
pub type PFN_vkGetQueueCheckpointData2NV = unsafe extern "system" fn(
    queue: Queue,
    p_checkpoint_data_count: *mut u32,
    p_checkpoint_data: *mut CheckpointData2NV,
);
