#![allow(unused_imports)]
use crate::*;
use core::ffi::{c_char, c_int, c_void, CStr};
use kazan_sys::{vk::*, *};
pub struct DeviceFn {
    get_native_buffer_properties_ohos: PFN_vkGetNativeBufferPropertiesOHOS,
    get_memory_native_buffer_ohos: PFN_vkGetMemoryNativeBufferOHOS,
}
impl DeviceFn {
    pub unsafe fn get_native_buffer_properties_ohos(
        &self,
        device: Device,
        buffer: &OH_NativeBuffer,
        properties: &mut NativeBufferPropertiesOHOS,
    ) -> Result {
        unsafe { (self.get_native_buffer_properties_ohos)(device, buffer, properties) }
    }
    pub unsafe fn get_memory_native_buffer_ohos(
        &self,
        device: Device,
        info: &MemoryGetNativeBufferInfoOHOS,
        buffer: &mut *mut OH_NativeBuffer,
    ) -> Result {
        unsafe { (self.get_memory_native_buffer_ohos)(device, info, buffer) }
    }
}
