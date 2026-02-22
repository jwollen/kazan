#![allow(unused_imports)]
use crate::*;
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
use kazan_sys::{vk::*, *};
pub struct DeviceFn {
    bind_buffer_memory2_khr: PFN_vkBindBufferMemory2,
    bind_image_memory2_khr: PFN_vkBindImageMemory2,
}
impl DeviceFn {
    pub unsafe fn load(
        load: impl Fn(&CStr) -> Option<PFN_vkVoidFunction>,
    ) -> core::result::Result<Self, LoadingError> {
        unsafe {
            Ok(Self {
                bind_buffer_memory2_khr: transmute(
                    load(c"vkBindBufferMemory2KHR").ok_or(LoadingError)?,
                ),
                bind_image_memory2_khr: transmute(
                    load(c"vkBindImageMemory2KHR").ok_or(LoadingError)?,
                ),
            })
        }
    }
}
impl DeviceFn {
    pub unsafe fn bind_buffer_memory2_khr(
        &self,
        device: Device,
        bind_infos: &[BindBufferMemoryInfo],
    ) -> crate::Result<()> {
        unsafe {
            result((self.bind_buffer_memory2_khr)(
                device,
                bind_infos.len().try_into().unwrap(),
                bind_infos.as_ptr() as _,
            ))
        }
    }
    pub unsafe fn bind_image_memory2_khr(
        &self,
        device: Device,
        bind_infos: &[BindImageMemoryInfo],
    ) -> crate::Result<()> {
        unsafe {
            result((self.bind_image_memory2_khr)(
                device,
                bind_infos.len().try_into().unwrap(),
                bind_infos.as_ptr() as _,
            ))
        }
    }
}
