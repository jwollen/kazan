#![allow(unused_imports)]
use crate::*;
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
use kazan_sys::{vk::*, *};
pub struct DeviceFn {
    cmd_set_discard_rectangle_ext: PFN_vkCmdSetDiscardRectangleEXT,
    cmd_set_discard_rectangle_enable_ext: PFN_vkCmdSetDiscardRectangleEnableEXT,
    cmd_set_discard_rectangle_mode_ext: PFN_vkCmdSetDiscardRectangleModeEXT,
}
impl DeviceFn {
    pub unsafe fn load(
        load: impl Fn(&CStr) -> Option<PFN_vkVoidFunction>,
    ) -> core::result::Result<Self, LoadingError> {
        unsafe {
            Ok(Self {
                cmd_set_discard_rectangle_ext: transmute(
                    load(c"vkCmdSetDiscardRectangleEXT").ok_or(LoadingError)?,
                ),
                cmd_set_discard_rectangle_enable_ext: transmute(
                    load(c"vkCmdSetDiscardRectangleEnableEXT").ok_or(LoadingError)?,
                ),
                cmd_set_discard_rectangle_mode_ext: transmute(
                    load(c"vkCmdSetDiscardRectangleModeEXT").ok_or(LoadingError)?,
                ),
            })
        }
    }
}
impl DeviceFn {
    pub unsafe fn cmd_set_discard_rectangle_ext(
        &self,
        command_buffer: CommandBuffer,
        first_discard_rectangle: u32,
        discard_rectangles: &[Rect2D],
    ) {
        unsafe {
            (self.cmd_set_discard_rectangle_ext)(
                command_buffer,
                first_discard_rectangle,
                discard_rectangles.len().try_into().unwrap(),
                discard_rectangles.as_ptr() as _,
            )
        }
    }
    pub unsafe fn cmd_set_discard_rectangle_enable_ext(
        &self,
        command_buffer: CommandBuffer,
        discard_rectangle_enable: Bool32,
    ) {
        unsafe {
            (self.cmd_set_discard_rectangle_enable_ext)(command_buffer, discard_rectangle_enable)
        }
    }
    pub unsafe fn cmd_set_discard_rectangle_mode_ext(
        &self,
        command_buffer: CommandBuffer,
        discard_rectangle_mode: DiscardRectangleModeEXT,
    ) {
        unsafe { (self.cmd_set_discard_rectangle_mode_ext)(command_buffer, discard_rectangle_mode) }
    }
}
