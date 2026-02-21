#![allow(unused_imports)]
use crate::*;
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
use kazan_sys::{vk::*, *};
pub struct DeviceFn {
    cmd_set_rendering_attachment_locations_khr: PFN_vkCmdSetRenderingAttachmentLocations,
    cmd_set_rendering_input_attachment_indices_khr: PFN_vkCmdSetRenderingInputAttachmentIndices,
}
impl DeviceFn {
    pub unsafe fn load(
        load: impl Fn(&CStr) -> Option<PFN_vkVoidFunction>,
    ) -> core::result::Result<Self, LoadingError> {
        unsafe {
            Ok(Self {
                cmd_set_rendering_attachment_locations_khr: transmute(
                    load(c"vkCmdSetRenderingAttachmentLocationsKHR").ok_or(LoadingError)?,
                ),
                cmd_set_rendering_input_attachment_indices_khr: transmute(
                    load(c"vkCmdSetRenderingInputAttachmentIndicesKHR").ok_or(LoadingError)?,
                ),
            })
        }
    }
}
impl DeviceFn {
    pub unsafe fn cmd_set_rendering_attachment_locations_khr(
        &self,
        command_buffer: CommandBuffer,
        location_info: &RenderingAttachmentLocationInfo,
    ) {
        unsafe { (self.cmd_set_rendering_attachment_locations_khr)(command_buffer, location_info) }
    }
    pub unsafe fn cmd_set_rendering_input_attachment_indices_khr(
        &self,
        command_buffer: CommandBuffer,
        input_attachment_index_info: &RenderingInputAttachmentIndexInfo,
    ) {
        unsafe {
            (self.cmd_set_rendering_input_attachment_indices_khr)(
                command_buffer,
                input_attachment_index_info,
            )
        }
    }
}
