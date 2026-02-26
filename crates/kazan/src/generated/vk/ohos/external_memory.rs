#![allow(unused_imports)]
use crate::*;
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
use kazan_sys::{vk::Result as VkResult, vk::*, *};
pub struct DeviceFn {
    get_native_buffer_properties_ohos: PFN_vkGetNativeBufferPropertiesOHOS,
    get_memory_native_buffer_ohos: PFN_vkGetMemoryNativeBufferOHOS,
}
impl DeviceFn {
    pub unsafe fn load(
        load: impl Fn(&CStr) -> Option<PFN_vkVoidFunction>,
    ) -> core::result::Result<Self, LoadingError> {
        unsafe {
            Ok(Self {
                get_native_buffer_properties_ohos: transmute(
                    load(c"vkGetNativeBufferPropertiesOHOS").ok_or(LoadingError)?,
                ),
                get_memory_native_buffer_ohos: transmute(
                    load(c"vkGetMemoryNativeBufferOHOS").ok_or(LoadingError)?,
                ),
            })
        }
    }
}
impl DeviceFn {
    pub unsafe fn get_native_buffer_properties_ohos(
        &self,
        device: Device,
        buffer: &OH_NativeBuffer,
    ) -> crate::Result<NativeBufferPropertiesOHOS<'_>> {
        unsafe {
            let mut properties = core::mem::MaybeUninit::uninit();
            let result =
                (self.get_native_buffer_properties_ohos)(device, buffer, properties.as_mut_ptr());

            match result {
                VkResult::SUCCESS => Ok(properties.assume_init()),
                err => Err(err),
            }
        }
    }
    pub unsafe fn get_memory_native_buffer_ohos(
        &self,
        device: Device,
        info: &MemoryGetNativeBufferInfoOHOS<'_>,
        buffer: &mut *mut OH_NativeBuffer,
    ) -> crate::Result<()> {
        unsafe {
            let result = (self.get_memory_native_buffer_ohos)(device, info, buffer);

            match result {
                VkResult::SUCCESS => Ok(()),
                err => Err(err),
            }
        }
    }
}
