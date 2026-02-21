#![allow(unused_imports)]
use crate::*;
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
use kazan_sys::{vk::*, *};
pub struct DeviceFn {
    get_device_subpass_shading_max_workgroup_size_huawei:
        PFN_vkGetDeviceSubpassShadingMaxWorkgroupSizeHUAWEI,
    cmd_subpass_shading_huawei: PFN_vkCmdSubpassShadingHUAWEI,
}
impl DeviceFn {
    pub unsafe fn load(
        load: impl Fn(&CStr) -> Option<PFN_vkVoidFunction>,
    ) -> core::result::Result<Self, LoadingError> {
        unsafe {
            Ok(Self {
                get_device_subpass_shading_max_workgroup_size_huawei: transmute(
                    load(c"vkGetDeviceSubpassShadingMaxWorkgroupSizeHUAWEI").ok_or(LoadingError)?,
                ),
                cmd_subpass_shading_huawei: transmute(
                    load(c"vkCmdSubpassShadingHUAWEI").ok_or(LoadingError)?,
                ),
            })
        }
    }
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
