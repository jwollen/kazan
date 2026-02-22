#![allow(unused_imports)]
use crate::*;
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
use kazan_sys::{vk::Result as VkResult, vk::*, *};
pub struct DeviceFn {
    debug_marker_set_object_tag_ext: PFN_vkDebugMarkerSetObjectTagEXT,
    debug_marker_set_object_name_ext: PFN_vkDebugMarkerSetObjectNameEXT,
    cmd_debug_marker_begin_ext: PFN_vkCmdDebugMarkerBeginEXT,
    cmd_debug_marker_end_ext: PFN_vkCmdDebugMarkerEndEXT,
    cmd_debug_marker_insert_ext: PFN_vkCmdDebugMarkerInsertEXT,
}
impl DeviceFn {
    pub unsafe fn load(
        load: impl Fn(&CStr) -> Option<PFN_vkVoidFunction>,
    ) -> core::result::Result<Self, LoadingError> {
        unsafe {
            Ok(Self {
                debug_marker_set_object_tag_ext: transmute(
                    load(c"vkDebugMarkerSetObjectTagEXT").ok_or(LoadingError)?,
                ),
                debug_marker_set_object_name_ext: transmute(
                    load(c"vkDebugMarkerSetObjectNameEXT").ok_or(LoadingError)?,
                ),
                cmd_debug_marker_begin_ext: transmute(
                    load(c"vkCmdDebugMarkerBeginEXT").ok_or(LoadingError)?,
                ),
                cmd_debug_marker_end_ext: transmute(
                    load(c"vkCmdDebugMarkerEndEXT").ok_or(LoadingError)?,
                ),
                cmd_debug_marker_insert_ext: transmute(
                    load(c"vkCmdDebugMarkerInsertEXT").ok_or(LoadingError)?,
                ),
            })
        }
    }
}
impl DeviceFn {
    pub unsafe fn debug_marker_set_object_tag_ext(
        &self,
        device: Device,
        tag_info: &DebugMarkerObjectTagInfoEXT,
    ) -> crate::Result<()> {
        unsafe {
            let result = (self.debug_marker_set_object_tag_ext)(device, tag_info);

            match result {
                VkResult::SUCCESS => Ok(()),
                err => Err(err),
            }
        }
    }
    pub unsafe fn debug_marker_set_object_name_ext(
        &self,
        device: Device,
        name_info: &DebugMarkerObjectNameInfoEXT,
    ) -> crate::Result<()> {
        unsafe {
            let result = (self.debug_marker_set_object_name_ext)(device, name_info);

            match result {
                VkResult::SUCCESS => Ok(()),
                err => Err(err),
            }
        }
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
