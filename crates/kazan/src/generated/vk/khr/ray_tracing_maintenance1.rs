#![allow(unused_imports)]
use crate::*;
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
use kazan_sys::{vk::Result as VkResult, vk::*, *};
pub struct DeviceFn {
    cmd_trace_rays_indirect2_khr: Option<PFN_vkCmdTraceRaysIndirect2KHR>,
}
impl DeviceFn {
    pub unsafe fn load(
        load: impl Fn(&CStr) -> Option<PFN_vkVoidFunction>,
    ) -> core::result::Result<Self, LoadingError> {
        unsafe {
            Ok(Self {
                cmd_trace_rays_indirect2_khr: transmute(load(c"vkCmdTraceRaysIndirect2KHR")),
            })
        }
    }
}
impl DeviceFn {
    pub unsafe fn cmd_trace_rays_indirect2_khr(
        &self,
        command_buffer: CommandBuffer,
        indirect_device_address: DeviceAddress,
    ) {
        unsafe {
            (self.cmd_trace_rays_indirect2_khr.unwrap())(command_buffer, indirect_device_address)
        }
    }
}
