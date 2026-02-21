#![allow(unused_imports)]
use crate::*;
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
use kazan_sys::{vk::*, *};
pub struct DeviceFn {
    set_hdr_metadata_ext: PFN_vkSetHdrMetadataEXT,
}
impl DeviceFn {
    pub unsafe fn load(
        load: impl Fn(&CStr) -> Option<PFN_vkVoidFunction>,
    ) -> core::result::Result<Self, LoadingError> {
        unsafe {
            Ok(Self {
                set_hdr_metadata_ext: transmute(load(c"vkSetHdrMetadataEXT").ok_or(LoadingError)?),
            })
        }
    }
}
impl DeviceFn {
    pub unsafe fn set_hdr_metadata_ext(
        &self,
        device: Device,
        swapchains: &[SwapchainKHR],
        metadata: &[HdrMetadataEXT],
    ) {
        unsafe {
            (self.set_hdr_metadata_ext)(
                device,
                swapchains.len().try_into().unwrap(),
                swapchains.as_ptr() as _,
                metadata.as_ptr() as _,
            )
        }
    }
}
