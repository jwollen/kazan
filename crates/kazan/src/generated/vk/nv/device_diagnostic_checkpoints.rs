#![allow(unused_imports)]
use crate::*;
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
use kazan_sys::{vk::Result as VkResult, vk::*, *};
pub struct DeviceFn {
    cmd_set_checkpoint_nv: PFN_vkCmdSetCheckpointNV,
    get_queue_checkpoint_data_nv: PFN_vkGetQueueCheckpointDataNV,
    get_queue_checkpoint_data2_nv: Option<PFN_vkGetQueueCheckpointData2NV>,
}
impl DeviceFn {
    pub unsafe fn load(
        load: impl Fn(&CStr) -> Option<PFN_vkVoidFunction>,
    ) -> core::result::Result<Self, LoadingError> {
        unsafe {
            Ok(Self {
                cmd_set_checkpoint_nv: transmute(
                    load(c"vkCmdSetCheckpointNV").ok_or(LoadingError)?,
                ),
                get_queue_checkpoint_data_nv: transmute(
                    load(c"vkGetQueueCheckpointDataNV").ok_or(LoadingError)?,
                ),
                get_queue_checkpoint_data2_nv: transmute(load(c"vkGetQueueCheckpointData2NV")),
            })
        }
    }
}
impl DeviceFn {
    pub unsafe fn cmd_set_checkpoint_nv(
        &self,
        command_buffer: CommandBuffer,
        checkpoint_marker: &c_void,
    ) {
        unsafe { (self.cmd_set_checkpoint_nv)(command_buffer, checkpoint_marker) }
    }
    pub unsafe fn get_queue_checkpoint_data_nv<'a>(
        &self,
        queue: Queue,
        checkpoint_data: impl ExtendUninit<CheckpointDataNV<'a>>,
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
    pub unsafe fn get_queue_checkpoint_data2_nv<'a>(
        &self,
        queue: Queue,
        checkpoint_data: impl ExtendUninit<CheckpointData2NV<'a>>,
    ) {
        unsafe {
            extend_uninit(checkpoint_data, |checkpoint_data_count, checkpoint_data| {
                (self.get_queue_checkpoint_data2_nv.unwrap())(
                    queue,
                    checkpoint_data_count,
                    checkpoint_data as _,
                )
            })
        }
    }
}
