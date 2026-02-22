#![allow(unused_imports)]
use crate::*;
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
use kazan_sys::{vk::Result as VkResult, vk::*, *};
pub struct DeviceFn {
    get_image_subresource_layout2_ext: PFN_vkGetImageSubresourceLayout2,
}
impl DeviceFn {
    pub unsafe fn load(
        load: impl Fn(&CStr) -> Option<PFN_vkVoidFunction>,
    ) -> core::result::Result<Self, LoadingError> {
        unsafe {
            Ok(Self {
                get_image_subresource_layout2_ext: transmute(
                    load(c"vkGetImageSubresourceLayout2EXT").ok_or(LoadingError)?,
                ),
            })
        }
    }
}
impl DeviceFn {
    pub unsafe fn get_image_subresource_layout2_ext(
        &self,
        device: Device,
        image: Image,
        subresource: &ImageSubresource2,
    ) -> SubresourceLayout2 {
        unsafe {
            let mut layout = core::mem::MaybeUninit::uninit();
            (self.get_image_subresource_layout2_ext)(
                device,
                image,
                subresource,
                layout.as_mut_ptr(),
            );
            layout.assume_init()
        }
    }
}
