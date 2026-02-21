#![allow(unused_imports)]
use crate::*;
use core::ffi::{c_char, c_int, c_void, CStr};
use kazan_sys::{vk::*, *};
pub struct DeviceFn {
    anti_lag_update_amd: PFN_vkAntiLagUpdateAMD,
}
impl DeviceFn {
    pub unsafe fn anti_lag_update_amd(&self, device: Device, data: &AntiLagDataAMD) {
        unsafe { (self.anti_lag_update_amd)(device, data) }
    }
}
