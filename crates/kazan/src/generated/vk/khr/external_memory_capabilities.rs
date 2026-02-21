#![allow(unused_imports)]
use crate::*;
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
use kazan_sys::{vk::*, *};
pub struct InstanceFn {
    get_physical_device_external_buffer_properties: PFN_vkGetPhysicalDeviceExternalBufferProperties,
}
impl InstanceFn {
    pub unsafe fn load(
        load: impl Fn(&CStr) -> Option<PFN_vkVoidFunction>,
    ) -> core::result::Result<Self, LoadingError> {
        unsafe {
            Ok(Self {
                get_physical_device_external_buffer_properties: transmute(
                    load(c"vkGetPhysicalDeviceExternalBufferPropertiesKHR").ok_or(LoadingError)?,
                ),
            })
        }
    }
}
impl InstanceFn {
    pub unsafe fn get_physical_device_external_buffer_properties_khr(
        &self,
        physical_device: PhysicalDevice,
        external_buffer_info: &PhysicalDeviceExternalBufferInfo,
        external_buffer_properties: &mut ExternalBufferProperties,
    ) {
        unsafe {
            (self.get_physical_device_external_buffer_properties)(
                physical_device,
                external_buffer_info,
                external_buffer_properties,
            )
        }
    }
}
