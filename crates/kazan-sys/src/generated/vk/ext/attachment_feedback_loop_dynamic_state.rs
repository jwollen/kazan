#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{c_char, c_int, c_void};
#[repr(C)]
pub struct PhysicalDeviceAttachmentFeedbackLoopDynamicStateFeaturesEXT {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub attachment_feedback_loop_dynamic_state: Bool32,
}
pub type PFN_vkCmdSetAttachmentFeedbackLoopEnableEXT =
    unsafe extern "system" fn(command_buffer: CommandBuffer, aspect_mask: ImageAspectFlags);
