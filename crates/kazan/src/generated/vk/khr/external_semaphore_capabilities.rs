#![allow(unused_imports)]
use crate::*;
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
use kazan_sys::{vk::Result as VkResult, vk::*, *};
pub struct InstanceFn {
    get_physical_device_external_semaphore_properties_khr:
        PFN_vkGetPhysicalDeviceExternalSemaphoreProperties,
}
impl InstanceFn {
    pub unsafe fn load(
        load: impl Fn(&CStr) -> Option<PFN_vkVoidFunction>,
    ) -> core::result::Result<Self, LoadingError> {
        unsafe {
            Ok(Self {
                get_physical_device_external_semaphore_properties_khr: transmute(
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
        external_semaphore_info: &PhysicalDeviceExternalSemaphoreInfo<'_>,
    ) -> ExternalSemaphoreProperties<'_> {
        unsafe {
            let mut external_semaphore_properties = core::mem::MaybeUninit::uninit();
            (self.get_physical_device_external_semaphore_properties_khr)(
                physical_device,
                external_semaphore_info,
                external_semaphore_properties.as_mut_ptr(),
            );
            external_semaphore_properties.assume_init()
        }
    }
}
