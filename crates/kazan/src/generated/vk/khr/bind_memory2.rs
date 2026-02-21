#![allow(unused_imports)]
use crate::*;
use core::ffi::{c_char, c_int, c_void, CStr};
use kazan_sys::{vk::*, *};
pub struct DeviceFn {
    bind_buffer_memory2: PFN_vkBindBufferMemory2,
    bind_image_memory2: PFN_vkBindImageMemory2,
}
impl DeviceFn {
    pub unsafe fn bind_buffer_memory2(
        &self,
        device: Device,
        bind_infos: &[BindBufferMemoryInfo],
    ) -> Result {
        unsafe {
            (self.bind_buffer_memory2)(
                device,
                bind_infos.len().try_into().unwrap(),
                bind_infos.as_ptr() as _,
            )
        }
    }
    pub unsafe fn bind_image_memory2(
        &self,
        device: Device,
        bind_infos: &[BindImageMemoryInfo],
    ) -> Result {
        unsafe {
            (self.bind_image_memory2)(
                device,
                bind_infos.len().try_into().unwrap(),
                bind_infos.as_ptr() as _,
            )
        }
    }
}
