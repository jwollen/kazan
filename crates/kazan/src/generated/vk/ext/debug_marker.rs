#![allow(unused_imports)]
use crate::*;
use core::ffi::{c_char, c_int, c_void, CStr};
use kazan_sys::{vk::*, *};
pub struct DeviceFn {
    debug_marker_set_object_tag_ext: PFN_vkDebugMarkerSetObjectTagEXT,
    debug_marker_set_object_name_ext: PFN_vkDebugMarkerSetObjectNameEXT,
    cmd_debug_marker_begin_ext: PFN_vkCmdDebugMarkerBeginEXT,
    cmd_debug_marker_end_ext: PFN_vkCmdDebugMarkerEndEXT,
    cmd_debug_marker_insert_ext: PFN_vkCmdDebugMarkerInsertEXT,
}
impl DeviceFn {
    pub unsafe fn debug_marker_set_object_tag_ext(
        &self,
        device: Device,
        tag_info: &DebugMarkerObjectTagInfoEXT,
    ) -> Result {
        unsafe { (self.debug_marker_set_object_tag_ext)(device, tag_info) }
    }
    pub unsafe fn debug_marker_set_object_name_ext(
        &self,
        device: Device,
        name_info: &DebugMarkerObjectNameInfoEXT,
    ) -> Result {
        unsafe { (self.debug_marker_set_object_name_ext)(device, name_info) }
    }
    pub unsafe fn cmd_debug_marker_begin_ext(
        &self,
        command_buffer: CommandBuffer,
        marker_info: &DebugMarkerMarkerInfoEXT,
    ) {
        unsafe { (self.cmd_debug_marker_begin_ext)(command_buffer, marker_info) }
    }
    pub unsafe fn cmd_debug_marker_end_ext(&self, command_buffer: CommandBuffer) {
        unsafe { (self.cmd_debug_marker_end_ext)(command_buffer) }
    }
    pub unsafe fn cmd_debug_marker_insert_ext(
        &self,
        command_buffer: CommandBuffer,
        marker_info: &DebugMarkerMarkerInfoEXT,
    ) {
        unsafe { (self.cmd_debug_marker_insert_ext)(command_buffer, marker_info) }
    }
}
