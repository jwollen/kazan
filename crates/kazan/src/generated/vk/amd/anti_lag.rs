#![allow(unused_imports)]
use crate::*;
use kazan_sys::{vk::*, *};
use std::ffi::{c_char, c_int, c_void, CStr};
pub struct DeviceFn {
    anti_lag_update_amd: PFN_vkAntiLagUpdateAMD,
}
impl DeviceFn {
    pub unsafe fn anti_lag_update_amd(&self, device: Device, data: &AntiLagDataAMD) {
        unsafe { (self.anti_lag_update_amd)(device, data) }
    }
}
