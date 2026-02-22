#![allow(unused_imports)]
use crate::*;
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
use kazan_sys::{vk::*, *};
pub struct DeviceFn {
    get_image_drm_format_modifier_properties_ext: PFN_vkGetImageDrmFormatModifierPropertiesEXT,
}
impl DeviceFn {
    pub unsafe fn load(
        load: impl Fn(&CStr) -> Option<PFN_vkVoidFunction>,
    ) -> core::result::Result<Self, LoadingError> {
        unsafe {
            Ok(Self {
                get_image_drm_format_modifier_properties_ext: transmute(
                    load(c"vkGetImageDrmFormatModifierPropertiesEXT").ok_or(LoadingError)?,
                ),
            })
        }
    }
}
impl DeviceFn {
    pub unsafe fn get_image_drm_format_modifier_properties_ext(
        &self,
        device: Device,
        image: Image,
        properties: &mut ImageDrmFormatModifierPropertiesEXT,
    ) -> crate::Result<()> {
        unsafe {
            result((self.get_image_drm_format_modifier_properties_ext)(
                device, image, properties,
            ))
        }
    }
}
