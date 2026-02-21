#![allow(unused_imports)]
use crate::*;
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
use kazan_sys::{vk::*, *};
pub struct DeviceFn {
    cmd_draw_multi_ext: PFN_vkCmdDrawMultiEXT,
    cmd_draw_multi_indexed_ext: PFN_vkCmdDrawMultiIndexedEXT,
}
impl DeviceFn {
    pub unsafe fn load(
        load: impl Fn(&CStr) -> Option<PFN_vkVoidFunction>,
    ) -> core::result::Result<Self, LoadingError> {
        unsafe {
            Ok(Self {
                cmd_draw_multi_ext: transmute(load(c"vkCmdDrawMultiEXT").ok_or(LoadingError)?),
                cmd_draw_multi_indexed_ext: transmute(
                    load(c"vkCmdDrawMultiIndexedEXT").ok_or(LoadingError)?,
                ),
            })
        }
    }
}
impl DeviceFn {
    pub unsafe fn cmd_draw_multi_ext(
        &self,
        command_buffer: CommandBuffer,
        vertex_info: &[MultiDrawInfoEXT],
        instance_count: u32,
        first_instance: u32,
        stride: u32,
    ) {
        unsafe {
            (self.cmd_draw_multi_ext)(
                command_buffer,
                vertex_info.len().try_into().unwrap(),
                vertex_info.as_ptr() as _,
                instance_count,
                first_instance,
                stride,
            )
        }
    }
    pub unsafe fn cmd_draw_multi_indexed_ext(
        &self,
        command_buffer: CommandBuffer,
        index_info: &[MultiDrawIndexedInfoEXT],
        instance_count: u32,
        first_instance: u32,
        stride: u32,
        vertex_offset: Option<&i32>,
    ) {
        unsafe {
            (self.cmd_draw_multi_indexed_ext)(
                command_buffer,
                index_info.len().try_into().unwrap(),
                index_info.as_ptr() as _,
                instance_count,
                first_instance,
                stride,
                vertex_offset.to_raw_ptr(),
            )
        }
    }
}
