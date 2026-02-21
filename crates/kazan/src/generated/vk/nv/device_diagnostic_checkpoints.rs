#![allow(unused_imports)]
use crate::*;
use core::ffi::{c_char, c_int, c_void, CStr};
use kazan_sys::{vk::*, *};
pub struct DeviceFn {
    cmd_set_checkpoint_nv: PFN_vkCmdSetCheckpointNV,
    get_queue_checkpoint_data_nv: PFN_vkGetQueueCheckpointDataNV,
    get_queue_checkpoint_data2_nv: PFN_vkGetQueueCheckpointData2NV,
}
impl DeviceFn {
    pub unsafe fn cmd_set_checkpoint_nv(
        &self,
        command_buffer: CommandBuffer,
        checkpoint_marker: &c_void,
    ) {
        unsafe { (self.cmd_set_checkpoint_nv)(command_buffer, checkpoint_marker) }
    }
    pub unsafe fn get_queue_checkpoint_data_nv(
        &self,
        queue: Queue,
        checkpoint_data: impl ExtendUninit<CheckpointDataNV>,
    ) {
        unsafe {
            extend_uninit(checkpoint_data, |checkpoint_data_count, checkpoint_data| {
                (self.get_queue_checkpoint_data_nv)(
                    queue,
                    checkpoint_data_count,
                    checkpoint_data as _,
                )
            })
        }
    }
    pub unsafe fn get_queue_checkpoint_data2_nv(
        &self,
        queue: Queue,
        checkpoint_data: impl ExtendUninit<CheckpointData2NV>,
    ) {
        unsafe {
            extend_uninit(checkpoint_data, |checkpoint_data_count, checkpoint_data| {
                (self.get_queue_checkpoint_data2_nv)(
                    queue,
                    checkpoint_data_count,
                    checkpoint_data as _,
                )
            })
        }
    }
}
