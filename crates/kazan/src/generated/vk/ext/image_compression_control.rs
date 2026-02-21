#![allow(unused_imports)]
use crate::*;
use core::ffi::{c_char, c_int, c_void, CStr};
use kazan_sys::{vk::*, *};
pub struct DeviceFn {
    get_image_subresource_layout2: PFN_vkGetImageSubresourceLayout2,
}
impl DeviceFn {
    pub unsafe fn get_image_subresource_layout2_ext(
        &self,
        device: Device,
        image: Image,
        subresource: &ImageSubresource2,
        layout: &mut SubresourceLayout2,
    ) {
        unsafe { (self.get_image_subresource_layout2)(device, image, subresource, layout) }
    }
}
