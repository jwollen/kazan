#![allow(unused_imports)]
use crate::*;
use kazan_sys::{vk::*, *};
use std::ffi::{c_char, c_int, c_void, CStr};
pub struct DeviceFn {
    cmd_bind_invocation_mask_huawei: PFN_vkCmdBindInvocationMaskHUAWEI,
}
impl DeviceFn {
    pub unsafe fn cmd_bind_invocation_mask_huawei(
        &self,
        command_buffer: CommandBuffer,
        image_view: ImageView,
        image_layout: ImageLayout,
    ) {
        unsafe { (self.cmd_bind_invocation_mask_huawei)(command_buffer, image_view, image_layout) }
    }
}
