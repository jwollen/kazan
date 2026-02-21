#![allow(unused_imports)]
use crate::*;
use core::ffi::{c_char, c_int, c_void, CStr};
use kazan_sys::{vk::*, *};
pub struct DeviceFn {
    cmd_set_rendering_attachment_locations: PFN_vkCmdSetRenderingAttachmentLocations,
    cmd_set_rendering_input_attachment_indices: PFN_vkCmdSetRenderingInputAttachmentIndices,
}
impl DeviceFn {
    pub unsafe fn cmd_set_rendering_attachment_locations(
        &self,
        command_buffer: CommandBuffer,
        location_info: &RenderingAttachmentLocationInfo,
    ) {
        unsafe { (self.cmd_set_rendering_attachment_locations)(command_buffer, location_info) }
    }
    pub unsafe fn cmd_set_rendering_input_attachment_indices(
        &self,
        command_buffer: CommandBuffer,
        input_attachment_index_info: &RenderingInputAttachmentIndexInfo,
    ) {
        unsafe {
            (self.cmd_set_rendering_input_attachment_indices)(
                command_buffer,
                input_attachment_index_info,
            )
        }
    }
}
