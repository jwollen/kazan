#![allow(unused_imports)]
use crate::*;
use kazan_sys::{vk::*, *};
use std::ffi::{c_char, c_int, c_void, CStr};
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
