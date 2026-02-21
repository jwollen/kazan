#![allow(unused_imports)]
use crate::*;
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
use kazan_sys::{vk::*, *};
pub struct DeviceFn {
    cmd_set_vertex_input_ext: PFN_vkCmdSetVertexInputEXT,
}
impl DeviceFn {
    pub unsafe fn load(
        load: impl Fn(&CStr) -> Option<PFN_vkVoidFunction>,
    ) -> core::result::Result<Self, LoadingError> {
        unsafe {
            Ok(Self {
                cmd_set_vertex_input_ext: transmute(
                    load(c"vkCmdSetVertexInputEXT").ok_or(LoadingError)?,
                ),
            })
        }
    }
}
impl DeviceFn {
    pub unsafe fn cmd_set_vertex_input_ext(
        &self,
        command_buffer: CommandBuffer,
        vertex_binding_descriptions: &[VertexInputBindingDescription2EXT],
        vertex_attribute_descriptions: &[VertexInputAttributeDescription2EXT],
    ) {
        unsafe {
            (self.cmd_set_vertex_input_ext)(
                command_buffer,
                vertex_binding_descriptions.len().try_into().unwrap(),
                vertex_binding_descriptions.as_ptr() as _,
                vertex_attribute_descriptions.len().try_into().unwrap(),
                vertex_attribute_descriptions.as_ptr() as _,
            )
        }
    }
}
