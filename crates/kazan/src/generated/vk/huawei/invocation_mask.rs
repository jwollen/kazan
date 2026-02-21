#![allow(unused_imports)]
use crate::*;
use core::ffi::{c_char, c_int, c_void, CStr};
use kazan_sys::{vk::*, *};
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
