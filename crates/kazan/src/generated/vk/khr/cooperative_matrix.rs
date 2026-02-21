#![allow(unused_imports)]
use crate::*;
use core::ffi::{c_char, c_int, c_void, CStr};
use kazan_sys::{vk::*, *};
pub struct InstanceFn {
    get_physical_device_cooperative_matrix_properties_khr:
        PFN_vkGetPhysicalDeviceCooperativeMatrixPropertiesKHR,
}
impl InstanceFn {
    pub unsafe fn get_physical_device_cooperative_matrix_properties_khr(
        &self,
        physical_device: PhysicalDevice,
        properties: impl ExtendUninit<CooperativeMatrixPropertiesKHR>,
    ) -> Result {
        unsafe {
            try_extend_uninit(properties, |property_count, properties| {
                (self.get_physical_device_cooperative_matrix_properties_khr)(
                    physical_device,
                    property_count,
                    properties as _,
                )
            })
        }
    }
}
