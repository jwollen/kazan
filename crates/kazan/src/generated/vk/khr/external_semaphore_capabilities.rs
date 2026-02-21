#![allow(unused_imports)]
use crate::*;
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
use kazan_sys::{vk::*, *};
pub struct InstanceFn {
    get_physical_device_external_semaphore_properties:
        PFN_vkGetPhysicalDeviceExternalSemaphoreProperties,
}
impl InstanceFn {
    pub unsafe fn load(
        load: impl Fn(&CStr) -> Option<PFN_vkVoidFunction>,
    ) -> core::result::Result<Self, LoadingError> {
        unsafe {
            Ok(Self {
                get_physical_device_external_semaphore_properties: transmute(
                    load(c"vkGetPhysicalDeviceExternalSemaphorePropertiesKHR")
                        .ok_or(LoadingError)?,
                ),
            })
        }
    }
}
impl InstanceFn {
    pub unsafe fn get_physical_device_external_semaphore_properties_khr(
        &self,
        physical_device: PhysicalDevice,
        external_semaphore_info: &PhysicalDeviceExternalSemaphoreInfo,
        external_semaphore_properties: &mut ExternalSemaphoreProperties,
    ) {
        unsafe {
            (self.get_physical_device_external_semaphore_properties)(
                physical_device,
                external_semaphore_info,
                external_semaphore_properties,
            )
        }
    }
}
