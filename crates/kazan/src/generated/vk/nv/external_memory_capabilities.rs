#![allow(unused_imports)]
use crate::*;
use kazan_sys::{vk::*, *};
use std::ffi::{c_char, c_int, c_void, CStr};
pub struct InstanceFn {
    get_physical_device_external_image_format_properties_nv:
        PFN_vkGetPhysicalDeviceExternalImageFormatPropertiesNV,
}
impl InstanceFn {
    pub unsafe fn get_physical_device_external_image_format_properties_nv(
        &self,
        physical_device: PhysicalDevice,
        format: Format,
        ty: ImageType,
        tiling: ImageTiling,
        usage: ImageUsageFlags,
        flags: ImageCreateFlags,
        external_handle_type: ExternalMemoryHandleTypeFlagsNV,
        external_image_format_properties: &mut ExternalImageFormatPropertiesNV,
    ) -> Result {
        unsafe {
            (self.get_physical_device_external_image_format_properties_nv)(
                physical_device,
                format,
                ty,
                tiling,
                usage,
                flags,
                external_handle_type,
                external_image_format_properties,
            )
        }
    }
}
