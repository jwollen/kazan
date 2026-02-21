#![allow(unused_imports)]
use crate::*;
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
use kazan_sys::{vk::*, *};
pub struct DeviceFn {
    get_device_fault_info_ext: PFN_vkGetDeviceFaultInfoEXT,
}
impl DeviceFn {
    pub unsafe fn load(
        load: impl Fn(&CStr) -> Option<PFN_vkVoidFunction>,
    ) -> core::result::Result<Self, LoadingError> {
        unsafe {
            Ok(Self {
                get_device_fault_info_ext: transmute(
                    load(c"vkGetDeviceFaultInfoEXT").ok_or(LoadingError)?,
                ),
            })
        }
    }
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
