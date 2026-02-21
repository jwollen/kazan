#![allow(unused_imports)]
use crate::*;
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
use kazan_sys::{vk::*, *};
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
        buffer: &_screen_buffer,
        properties: &mut ScreenBufferPropertiesQNX,
    ) -> Result {
        unsafe { (self.get_screen_buffer_properties_qnx)(device, buffer, properties) }
    }
}
