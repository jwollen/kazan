#![allow(unused_imports)]
use crate::*;
use core::ffi::{c_char, c_int, c_void, CStr};
use kazan_sys::{vk::*, *};
pub struct DeviceFn {
    cmd_trace_rays_indirect2_khr: PFN_vkCmdTraceRaysIndirect2KHR,
}
impl DeviceFn {
    pub unsafe fn cmd_trace_rays_indirect2_khr(
        &self,
        command_buffer: CommandBuffer,
        indirect_device_address: DeviceAddress,
    ) {
        unsafe { (self.cmd_trace_rays_indirect2_khr)(command_buffer, indirect_device_address) }
    }
}
