#![allow(unused_imports)]
use crate::*;
use core::ffi::{c_char, c_int, c_void, CStr};
use kazan_sys::{vk::*, *};
pub struct DeviceFn {
    set_hdr_metadata_ext: PFN_vkSetHdrMetadataEXT,
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
