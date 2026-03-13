//! <https://registry.khronos.org/vulkan/specs/latest/man/html/VK_NV_device_diagnostic_checkpoints.html>
#![allow(unused_imports)]
use crate::{vk::Result as VkResult, vk::*, *};
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
use core::ptr;

pub const EXTENSION_NAME: &CStr = c"VK_NV_device_diagnostic_checkpoints";

pub(super) mod defs {
    #![allow(non_camel_case_types, unused_imports)]
    use crate::{vk::*, *};
    use core::ffi::{CStr, c_char, c_int, c_void};
    use core::fmt;
    use core::marker::PhantomData;
    use core::ptr;

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkQueueFamilyCheckpointPropertiesNV.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct QueueFamilyCheckpointPropertiesNV<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub checkpoint_execution_stage_mask: PipelineStageFlags,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for QueueFamilyCheckpointPropertiesNV<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("QueueFamilyCheckpointPropertiesNV")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field(
                    "checkpoint_execution_stage_mask",
                    &self.checkpoint_execution_stage_mask,
                )
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for QueueFamilyCheckpointPropertiesNV<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::QUEUE_FAMILY_CHECKPOINT_PROPERTIES_NV;
    }

    unsafe impl Extends<QueueFamilyProperties2<'_>> for QueueFamilyCheckpointPropertiesNV<'_> {}

    impl Default for QueueFamilyCheckpointPropertiesNV<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null_mut(),
                checkpoint_execution_stage_mask: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> QueueFamilyCheckpointPropertiesNV<'a> {
        #[inline]
        pub fn checkpoint_execution_stage_mask(
            mut self,
            checkpoint_execution_stage_mask: PipelineStageFlags,
        ) -> Self {
            self.checkpoint_execution_stage_mask = checkpoint_execution_stage_mask;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkCheckpointDataNV.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct CheckpointDataNV<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub stage: PipelineStageFlagBits,
        pub p_checkpoint_marker: *mut c_void,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for CheckpointDataNV<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("CheckpointDataNV")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("stage", &self.stage)
                .field("p_checkpoint_marker", &self.p_checkpoint_marker)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for CheckpointDataNV<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::CHECKPOINT_DATA_NV;
    }

    impl Default for CheckpointDataNV<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null_mut(),
                stage: Default::default(),
                p_checkpoint_marker: ptr::null_mut(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> CheckpointDataNV<'a> {
        #[inline]
        pub fn stage(mut self, stage: PipelineStageFlagBits) -> Self {
            self.stage = stage;
            self
        }

        #[inline]
        pub fn checkpoint_marker(mut self, checkpoint_marker: *mut c_void) -> Self {
            self.p_checkpoint_marker = checkpoint_marker;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkQueueFamilyCheckpointProperties2NV.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct QueueFamilyCheckpointProperties2NV<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub checkpoint_execution_stage_mask: PipelineStageFlags2,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for QueueFamilyCheckpointProperties2NV<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("QueueFamilyCheckpointProperties2NV")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field(
                    "checkpoint_execution_stage_mask",
                    &self.checkpoint_execution_stage_mask,
                )
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for QueueFamilyCheckpointProperties2NV<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::QUEUE_FAMILY_CHECKPOINT_PROPERTIES_2_NV;
    }

    unsafe impl Extends<QueueFamilyProperties2<'_>> for QueueFamilyCheckpointProperties2NV<'_> {}

    impl Default for QueueFamilyCheckpointProperties2NV<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null_mut(),
                checkpoint_execution_stage_mask: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> QueueFamilyCheckpointProperties2NV<'a> {
        #[inline]
        pub fn checkpoint_execution_stage_mask(
            mut self,
            checkpoint_execution_stage_mask: PipelineStageFlags2,
        ) -> Self {
            self.checkpoint_execution_stage_mask = checkpoint_execution_stage_mask;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkCheckpointData2NV.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct CheckpointData2NV<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub stage: PipelineStageFlags2,
        pub p_checkpoint_marker: *mut c_void,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for CheckpointData2NV<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("CheckpointData2NV")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("stage", &self.stage)
                .field("p_checkpoint_marker", &self.p_checkpoint_marker)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for CheckpointData2NV<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::CHECKPOINT_DATA_2_NV;
    }

    impl Default for CheckpointData2NV<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null_mut(),
                stage: Default::default(),
                p_checkpoint_marker: ptr::null_mut(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> CheckpointData2NV<'a> {
        #[inline]
        pub fn stage(mut self, stage: PipelineStageFlags2) -> Self {
            self.stage = stage;
            self
        }

        #[inline]
        pub fn checkpoint_marker(mut self, checkpoint_marker: *mut c_void) -> Self {
            self.p_checkpoint_marker = checkpoint_marker;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdSetCheckpointNV.html>
    pub type PFN_vkCmdSetCheckpointNV = unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        p_checkpoint_marker: *const c_void,
    );
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkGetQueueCheckpointDataNV.html>
    pub type PFN_vkGetQueueCheckpointDataNV = unsafe extern "system" fn(
        queue: Queue,
        p_checkpoint_data_count: *mut u32,
        p_checkpoint_data: *mut CheckpointDataNV<'_>,
    );
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkGetQueueCheckpointData2NV.html>
    pub type PFN_vkGetQueueCheckpointData2NV = unsafe extern "system" fn(
        queue: Queue,
        p_checkpoint_data_count: *mut u32,
        p_checkpoint_data: *mut CheckpointData2NV<'_>,
    );
}

#[cfg(feature = "ffi")]
pub(super) mod ffi {
    #![allow(non_camel_case_types)]
    use super::defs::*;

    pub type VkQueueFamilyCheckpointPropertiesNV = QueueFamilyCheckpointPropertiesNV<'static>;
    pub type VkCheckpointDataNV = CheckpointDataNV<'static>;
    pub type VkQueueFamilyCheckpointProperties2NV = QueueFamilyCheckpointProperties2NV<'static>;
    pub type VkCheckpointData2NV = CheckpointData2NV<'static>;
    impl QueueFamilyCheckpointPropertiesNV<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkQueueFamilyCheckpointPropertiesNV {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl CheckpointDataNV<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkCheckpointDataNV {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl QueueFamilyCheckpointProperties2NV<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkQueueFamilyCheckpointProperties2NV {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl CheckpointData2NV<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkCheckpointData2NV {
            unsafe { core::mem::transmute(self) }
        }
    }
}

pub struct DeviceFn {
    cmd_set_checkpoint: PFN_vkCmdSetCheckpointNV,
    get_queue_checkpoint_data: PFN_vkGetQueueCheckpointDataNV,
    get_queue_checkpoint_data2: Option<PFN_vkGetQueueCheckpointData2NV>,
}

impl LoadDeviceFn for DeviceFn {
    unsafe fn load_with(
        load: impl Fn(&CStr) -> Option<PFN_vkVoidFunction>,
    ) -> core::result::Result<Self, MissingEntryPointError> {
        unsafe {
            Ok(Self {
                cmd_set_checkpoint: transmute(
                    load(c"vkCmdSetCheckpointNV").ok_or(MissingEntryPointError)?,
                ),
                get_queue_checkpoint_data: transmute(
                    load(c"vkGetQueueCheckpointDataNV").ok_or(MissingEntryPointError)?,
                ),
                get_queue_checkpoint_data2: transmute(load(c"vkGetQueueCheckpointData2NV")),
            })
        }
    }
}

impl DeviceFn {
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdSetCheckpointNV.html>
    #[inline]
    pub unsafe fn cmd_set_checkpoint(
        &self,
        command_buffer: CommandBuffer,
        checkpoint_marker: *const c_void,
    ) {
        unsafe { (self.cmd_set_checkpoint)(command_buffer, checkpoint_marker) }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkGetQueueCheckpointDataNV.html>
    #[inline]
    pub unsafe fn get_queue_checkpoint_data<'a>(
        &self,
        queue: Queue,
        mut checkpoint_data: impl ExtendUninit<CheckpointDataNV<'a>>,
    ) {
        unsafe {
            let call = |checkpoint_data_count, checkpoint_data| {
                (self.get_queue_checkpoint_data)(queue, checkpoint_data_count, checkpoint_data as _)
            };
            let mut len = 0;
            call(&mut len, std::ptr::null_mut());
            let capacity = len.try_into().expect("failed to convert `N` to usize");
            let checkpoint_data_buf = checkpoint_data.reserve(capacity);
            len = checkpoint_data_buf.len().try_into().unwrap();
            call(&mut len, checkpoint_data_buf.as_mut_ptr() as *mut _);
            checkpoint_data.set_len(len.try_into().unwrap());
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkGetQueueCheckpointData2NV.html>
    #[inline]
    pub unsafe fn get_queue_checkpoint_data2<'a>(
        &self,
        queue: Queue,
        mut checkpoint_data: impl ExtendUninit<CheckpointData2NV<'a>>,
    ) {
        unsafe {
            let call = |checkpoint_data_count, checkpoint_data| {
                (self.get_queue_checkpoint_data2.unwrap())(
                    queue,
                    checkpoint_data_count,
                    checkpoint_data as _,
                )
            };
            let mut len = 0;
            call(&mut len, std::ptr::null_mut());
            let capacity = len.try_into().expect("failed to convert `N` to usize");
            let checkpoint_data_buf = checkpoint_data.reserve(capacity);
            len = checkpoint_data_buf.len().try_into().unwrap();
            call(&mut len, checkpoint_data_buf.as_mut_ptr() as *mut _);
            checkpoint_data.set_len(len.try_into().unwrap());
        }
    }
}
