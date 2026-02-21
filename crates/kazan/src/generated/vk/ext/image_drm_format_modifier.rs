#![allow(unused_imports)]
use crate::*;
use core::ffi::{c_char, c_int, c_void, CStr};
use kazan_sys::{vk::*, *};
pub struct DeviceFn {
    get_image_drm_format_modifier_properties_ext: PFN_vkGetImageDrmFormatModifierPropertiesEXT,
}
impl DeviceFn {
    pub unsafe fn get_image_drm_format_modifier_properties_ext(
        &self,
        device: Device,
        image: Image,
        properties: &mut ImageDrmFormatModifierPropertiesEXT,
    ) -> Result {
        unsafe { (self.get_image_drm_format_modifier_properties_ext)(device, image, properties) }
    }
}
