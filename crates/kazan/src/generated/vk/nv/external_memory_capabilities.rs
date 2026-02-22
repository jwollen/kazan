#![allow(unused_imports)]
use crate::*;
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
use kazan_sys::{vk::Result as VkResult, vk::*, *};
pub struct InstanceFn {
    get_physical_device_external_image_format_properties_nv:
        PFN_vkGetPhysicalDeviceExternalImageFormatPropertiesNV,
}
impl InstanceFn {
    pub unsafe fn load(
        load: impl Fn(&CStr) -> Option<PFN_vkVoidFunction>,
    ) -> core::result::Result<Self, LoadingError> {
        unsafe {
            Ok(Self {
                get_physical_device_external_image_format_properties_nv: transmute(
                    load(c"vkGetPhysicalDeviceExternalImageFormatPropertiesNV")
                        .ok_or(LoadingError)?,
                ),
            })
        }
    }
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
    ) -> crate::Result<ExternalImageFormatPropertiesNV> {
        unsafe {
            let mut external_image_format_properties = core::mem::MaybeUninit::uninit();
            let result = (self.get_physical_device_external_image_format_properties_nv)(
                physical_device,
                format,
                ty,
                tiling,
                usage,
                flags,
                external_handle_type,
                external_image_format_properties.as_mut_ptr(),
            );

            match result {
                VkResult::SUCCESS => Ok(external_image_format_properties.assume_init()),
                err => Err(err),
            }
        }
    }
}
