#![allow(unused_imports)]
use crate::*;
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
use kazan_sys::{vk::Result as VkResult, vk::*, *};
pub struct DeviceFn {
    get_screen_buffer_properties_qnx: PFN_vkGetScreenBufferPropertiesQNX,
}
impl DeviceFn {
    pub unsafe fn load(
        load: impl Fn(&CStr) -> Option<PFN_vkVoidFunction>,
    ) -> core::result::Result<Self, LoadingError> {
        unsafe {
            Ok(Self {
                get_screen_buffer_properties_qnx: transmute(
                    load(c"vkGetScreenBufferPropertiesQNX").ok_or(LoadingError)?,
                ),
            })
        }
    }
}
impl DeviceFn {
    pub unsafe fn get_screen_buffer_properties_qnx(
        &self,
        device: Device,
        buffer: *const _screen_buffer,
    ) -> crate::Result<ScreenBufferPropertiesQNX> {
        unsafe {
            let mut properties = core::mem::MaybeUninit::uninit();
            let result =
                (self.get_screen_buffer_properties_qnx)(device, buffer, properties.as_mut_ptr());

            match result {
                VkResult::SUCCESS => Ok(properties.assume_init()),
                err => Err(err),
            }
        }
    }
}
