#![allow(unused_imports)]
use crate::*;
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
use kazan_sys::{vk::*, *};
pub struct InstanceFn {
    get_physical_device_external_fence_properties: PFN_vkGetPhysicalDeviceExternalFenceProperties,
}
impl InstanceFn {
    pub unsafe fn load(
        load: impl Fn(&CStr) -> Option<PFN_vkVoidFunction>,
    ) -> core::result::Result<Self, LoadingError> {
        unsafe {
            Ok(Self {
                get_physical_device_external_fence_properties: transmute(
                    load(c"vkGetPhysicalDeviceExternalFencePropertiesKHR").ok_or(LoadingError)?,
                ),
            })
        }
    }
}
impl InstanceFn {
    pub unsafe fn get_physical_device_external_fence_properties_khr(
        &self,
        physical_device: PhysicalDevice,
        external_fence_info: &PhysicalDeviceExternalFenceInfo,
        external_fence_properties: &mut ExternalFenceProperties,
    ) {
        unsafe {
            (self.get_physical_device_external_fence_properties)(
                physical_device,
                external_fence_info,
                external_fence_properties,
            )
        }
    }
}
