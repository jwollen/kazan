#![allow(unused_imports)]
use crate::*;
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
use kazan_sys::{vk::*, *};
pub struct DeviceFn {
    anti_lag_update_amd: PFN_vkAntiLagUpdateAMD,
}
impl DeviceFn {
    pub unsafe fn load(
        load: impl Fn(&CStr) -> Option<PFN_vkVoidFunction>,
    ) -> core::result::Result<Self, LoadingError> {
        unsafe {
            Ok(Self {
                anti_lag_update_amd: transmute(load(c"vkAntiLagUpdateAMD").ok_or(LoadingError)?),
            })
        }
    }
}
impl DeviceFn {
    pub unsafe fn anti_lag_update_amd(&self, device: Device, data: &AntiLagDataAMD) {
        unsafe { (self.anti_lag_update_amd)(device, data) }
    }
}
