#![allow(unused_imports)]
use crate::*;
use core::ffi::{c_char, c_int, c_void, CStr};
use kazan_sys::{vk::*, *};
pub struct DeviceFn {
    get_android_hardware_buffer_properties_android: PFN_vkGetAndroidHardwareBufferPropertiesANDROID,
    get_memory_android_hardware_buffer_android: PFN_vkGetMemoryAndroidHardwareBufferANDROID,
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
