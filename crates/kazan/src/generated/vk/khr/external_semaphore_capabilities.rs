#![allow(unused_imports)]
use crate::*;
use core::ffi::{c_char, c_int, c_void, CStr};
use kazan_sys::{vk::*, *};
pub struct InstanceFn {
    get_physical_device_external_semaphore_properties:
        PFN_vkGetPhysicalDeviceExternalSemaphoreProperties,
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
