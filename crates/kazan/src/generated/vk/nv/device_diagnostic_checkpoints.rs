#![allow(unused_imports)]
use crate::{vk::Result as VkResult, vk::*, *};
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
pub(super) mod defs {
    #![allow(non_camel_case_types, unused_imports)]
    use crate::{vk::*, *};
    use bitflags::bitflags;
    use core::ffi::{CStr, c_char, c_int, c_void};
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
    impl<'a> QueueFamilyCheckpointPropertiesNV<'a> {
        pub fn checkpoint_execution_stage_mask(
            mut self,
            checkpoint_execution_stage_mask: PipelineStageFlags,
        ) -> Self {
            self.checkpoint_execution_stage_mask = checkpoint_execution_stage_mask;
            self
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
    impl<'a> CheckpointDataNV<'a> {
        pub fn stage(mut self, stage: PipelineStageFlagBits) -> Self {
            self.stage = stage;
            self
        }
        pub fn checkpoint_marker(mut self, checkpoint_marker: &'a mut c_void) -> Self {
            self.p_checkpoint_marker = checkpoint_marker;
            self
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
    impl<'a> QueueFamilyCheckpointProperties2NV<'a> {
        pub fn checkpoint_execution_stage_mask(
            mut self,
            checkpoint_execution_stage_mask: PipelineStageFlags2,
        ) -> Self {
            self.checkpoint_execution_stage_mask = checkpoint_execution_stage_mask;
            self
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
    impl<'a> CheckpointData2NV<'a> {
        pub fn stage(mut self, stage: PipelineStageFlags2) -> Self {
            self.stage = stage;
            self
        }
        pub fn checkpoint_marker(mut self, checkpoint_marker: &'a mut c_void) -> Self {
            self.p_checkpoint_marker = checkpoint_marker;
            self
        }
    }
    pub type PFN_vkCmdSetCheckpointNV = unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        p_checkpoint_marker: *const c_void,
    );
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
}
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
