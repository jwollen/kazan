#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{c_char, c_int, c_void};
use core::marker::PhantomData;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct QueueFamilyCheckpointPropertiesNV<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub checkpoint_execution_stage_mask: PipelineStageFlags,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for QueueFamilyCheckpointPropertiesNV<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::QUEUE_FAMILY_CHECKPOINT_PROPERTIES_NV,
            p_next: core::ptr::null_mut(),
            checkpoint_execution_stage_mask: Default::default(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct CheckpointDataNV<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub stage: PipelineStageFlagBits,
    pub p_checkpoint_marker: *mut c_void,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for CheckpointDataNV<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::CHECKPOINT_DATA_NV,
            p_next: core::ptr::null_mut(),
            stage: Default::default(),
            p_checkpoint_marker: core::ptr::null_mut(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct QueueFamilyCheckpointProperties2NV<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub checkpoint_execution_stage_mask: PipelineStageFlags2,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for QueueFamilyCheckpointProperties2NV<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::QUEUE_FAMILY_CHECKPOINT_PROPERTIES_2_NV,
            p_next: core::ptr::null_mut(),
            checkpoint_execution_stage_mask: Default::default(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct CheckpointData2NV<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub stage: PipelineStageFlags2,
    pub p_checkpoint_marker: *mut c_void,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for CheckpointData2NV<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::CHECKPOINT_DATA_2_NV,
            p_next: core::ptr::null_mut(),
            stage: Default::default(),
            p_checkpoint_marker: core::ptr::null_mut(),
            _marker: PhantomData,
        }
    }
}
pub type PFN_vkCmdSetCheckpointNV =
    unsafe extern "system" fn(command_buffer: CommandBuffer, p_checkpoint_marker: *const c_void);
pub type PFN_vkGetQueueCheckpointDataNV = unsafe extern "system" fn(
    queue: Queue,
    p_checkpoint_data_count: *mut u32,
    p_checkpoint_data: *mut CheckpointDataNV<'_>,
);
pub type PFN_vkGetQueueCheckpointData2NV = unsafe extern "system" fn(
    queue: Queue,
    p_checkpoint_data_count: *mut u32,
    p_checkpoint_data: *mut CheckpointData2NV<'_>,
);
