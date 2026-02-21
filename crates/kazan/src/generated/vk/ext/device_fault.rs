#![allow(unused_imports)]
use crate::*;
use core::ffi::{c_char, c_int, c_void, CStr};
use kazan_sys::{vk::*, *};
pub struct DeviceFn {
    get_device_fault_info_ext: PFN_vkGetDeviceFaultInfoEXT,
}
impl DeviceFn {
    pub unsafe fn get_device_fault_info_ext(
        &self,
        device: Device,
        fault_counts: &mut DeviceFaultCountsEXT,
        fault_info: Option<&mut DeviceFaultInfoEXT>,
    ) -> Result {
        unsafe {
            (self.get_device_fault_info_ext)(device, fault_counts, fault_info.to_raw_mut_ptr())
        }
    }
}
