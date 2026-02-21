#![allow(unused_imports)]
use crate::*;
use core::ffi::{c_char, c_int, c_void, CStr};
use kazan_sys::{vk::*, *};
pub struct DeviceFn {
    get_screen_buffer_properties_qnx: PFN_vkGetScreenBufferPropertiesQNX,
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
