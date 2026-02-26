#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{c_char, c_int, c_void};
use core::marker::PhantomData;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceAttachmentFeedbackLoopDynamicStateFeaturesEXT<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub attachment_feedback_loop_dynamic_state: Bool32,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for PhysicalDeviceAttachmentFeedbackLoopDynamicStateFeaturesEXT<'_> {
    fn default() -> Self {
        Self {
            s_type:
                StructureType::PHYSICAL_DEVICE_ATTACHMENT_FEEDBACK_LOOP_DYNAMIC_STATE_FEATURES_EXT,
            p_next: core::ptr::null_mut(),
            attachment_feedback_loop_dynamic_state: Default::default(),
            _marker: PhantomData,
        }
    }
}
pub type PFN_vkCmdSetAttachmentFeedbackLoopEnableEXT =
    unsafe extern "system" fn(command_buffer: CommandBuffer, aspect_mask: ImageAspectFlags);
