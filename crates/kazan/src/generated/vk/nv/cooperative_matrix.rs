#![allow(unused_imports)]
use crate::*;
use kazan_sys::{vk::*, *};
use std::ffi::{c_char, c_int, c_void, CStr};
pub struct InstanceFn {
    get_physical_device_cooperative_matrix_properties_nv:
        PFN_vkGetPhysicalDeviceCooperativeMatrixPropertiesNV,
}
impl InstanceFn {
    pub unsafe fn get_physical_device_cooperative_matrix_properties_nv(
        &self,
        physical_device: PhysicalDevice,
        properties: impl ExtendUninit<CooperativeMatrixPropertiesNV>,
    ) -> Result {
        unsafe {
            try_extend_uninit(properties, |property_count, properties| {
                (self.get_physical_device_cooperative_matrix_properties_nv)(
                    physical_device,
                    property_count,
                    properties as _,
                )
            })
        }
    }
}
