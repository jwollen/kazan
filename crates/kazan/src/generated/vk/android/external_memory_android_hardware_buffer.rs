#![allow(unused_imports)]
use crate::*;
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
use kazan_sys::{vk::Result as VkResult, vk::*, *};
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
        buffer: *const AHardwareBuffer,
    ) -> crate::Result<AndroidHardwareBufferPropertiesANDROID> {
        unsafe {
            let mut properties = core::mem::MaybeUninit::uninit();
            let result = (self.get_android_hardware_buffer_properties_android)(
                device,
                buffer,
                properties.as_mut_ptr(),
            );

            match result {
                VkResult::SUCCESS => Ok(properties.assume_init()),
                err => Err(err),
            }
        }
    }
    pub unsafe fn get_memory_android_hardware_buffer_android(
        &self,
        device: Device,
        info: &MemoryGetAndroidHardwareBufferInfoANDROID,
        buffer: &mut *mut AHardwareBuffer,
    ) -> crate::Result<()> {
        unsafe {
            let result = (self.get_memory_android_hardware_buffer_android)(device, info, buffer);

            match result {
                VkResult::SUCCESS => Ok(()),
                err => Err(err),
            }
        }
    }
}
