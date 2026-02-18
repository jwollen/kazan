#![allow(unused_imports)]
use crate::*;
use kazan_sys::{vk::*, *};
use std::ffi::{c_char, c_int, c_void, CStr};
pub struct DeviceFn {
    cmd_set_attachment_feedback_loop_enable_ext: PFN_vkCmdSetAttachmentFeedbackLoopEnableEXT,
}
impl DeviceFn {
    pub unsafe fn cmd_set_attachment_feedback_loop_enable_ext(
        &self,
        command_buffer: CommandBuffer,
        aspect_mask: ImageAspectFlags,
    ) {
        unsafe { (self.cmd_set_attachment_feedback_loop_enable_ext)(command_buffer, aspect_mask) }
    }
}
