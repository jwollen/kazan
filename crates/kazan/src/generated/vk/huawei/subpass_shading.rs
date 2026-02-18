#![allow(unused_imports)]
use crate::*;
use kazan_sys::{vk::*, *};
use std::ffi::{c_char, c_int, c_void, CStr};
pub struct DeviceFn {
    get_device_subpass_shading_max_workgroup_size_huawei:
        PFN_vkGetDeviceSubpassShadingMaxWorkgroupSizeHUAWEI,
    cmd_subpass_shading_huawei: PFN_vkCmdSubpassShadingHUAWEI,
}
impl DeviceFn {
    pub unsafe fn get_device_subpass_shading_max_workgroup_size_huawei(
        &self,
        device: Device,
        renderpass: RenderPass,
        max_workgroup_size: &mut Extent2D,
    ) -> Result {
        unsafe {
            (self.get_device_subpass_shading_max_workgroup_size_huawei)(
                device,
                renderpass,
                max_workgroup_size,
            )
        }
    }
    pub unsafe fn cmd_subpass_shading_huawei(&self, command_buffer: CommandBuffer) {
        unsafe { (self.cmd_subpass_shading_huawei)(command_buffer) }
    }
}
