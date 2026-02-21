#![allow(unused_imports)]
use crate::*;
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
use kazan_sys::{vk::*, *};
pub struct DeviceFn {
    get_android_hardware_buffer_properties_android: PFN_vkGetAndroidHardwareBufferPropertiesANDROID,
    get_memory_android_hardware_buffer_android: PFN_vkGetMemoryAndroidHardwareBufferANDROID,
}
impl DeviceFn {
    pub unsafe fn load(
        load: impl Fn(&CStr) -> Option<PFN_vkVoidFunction>,
    ) -> core::result::Result<Self, LoadingError> {
        unsafe {
            Ok(Self {
                get_android_hardware_buffer_properties_android: transmute(
                    load(c"vkGetAndroidHardwareBufferPropertiesANDROID").ok_or(LoadingError)?,
                ),
                get_memory_android_hardware_buffer_android: transmute(
                    load(c"vkGetMemoryAndroidHardwareBufferANDROID").ok_or(LoadingError)?,
                ),
            })
        }
    }
}
impl DeviceFn {
    pub unsafe fn get_android_hardware_buffer_properties_android(
        &self,
        device: Device,
        buffer: &AHardwareBuffer,
        properties: &mut AndroidHardwareBufferPropertiesANDROID,
    ) -> Result {
        unsafe { (self.get_android_hardware_buffer_properties_android)(device, buffer, properties) }
    }
    pub unsafe fn get_memory_android_hardware_buffer_android(
        &self,
        device: Device,
        info: &MemoryGetAndroidHardwareBufferInfoANDROID,
        buffer: &mut *mut AHardwareBuffer,
    ) -> Result {
        unsafe { (self.get_memory_android_hardware_buffer_android)(device, info, buffer) }
    }
}
