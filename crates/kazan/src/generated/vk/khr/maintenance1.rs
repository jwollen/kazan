#![allow(unused_imports)]
use crate::*;
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
use kazan_sys::{vk::Result as VkResult, vk::*, *};
pub struct DeviceFn {
    trim_command_pool_khr: PFN_vkTrimCommandPool,
}
impl DeviceFn {
    pub unsafe fn load(
        load: impl Fn(&CStr) -> Option<PFN_vkVoidFunction>,
    ) -> core::result::Result<Self, LoadingError> {
        unsafe {
            Ok(Self {
                trim_command_pool_khr: transmute(
                    load(c"vkTrimCommandPoolKHR").ok_or(LoadingError)?,
                ),
            })
        }
    }
}
impl DeviceFn {
    pub unsafe fn trim_command_pool_khr(
        &self,
        device: Device,
        command_pool: CommandPool,
        flags: CommandPoolTrimFlags,
    ) {
        unsafe { (self.trim_command_pool_khr)(device, command_pool, flags) }
    }
}
