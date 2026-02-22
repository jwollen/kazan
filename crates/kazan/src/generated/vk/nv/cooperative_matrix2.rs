#![allow(unused_imports)]
use crate::*;
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
use kazan_sys::{vk::Result as VkResult, vk::*, *};
pub struct InstanceFn {
    get_physical_device_cooperative_matrix_flexible_dimensions_properties_nv:
        PFN_vkGetPhysicalDeviceCooperativeMatrixFlexibleDimensionsPropertiesNV,
}
impl InstanceFn {
    pub unsafe fn load(
        load: impl Fn(&CStr) -> Option<PFN_vkVoidFunction>,
    ) -> core::result::Result<Self, LoadingError> {
        unsafe {
            Ok(Self {
                get_physical_device_cooperative_matrix_flexible_dimensions_properties_nv: transmute(
                    load(c"vkGetPhysicalDeviceCooperativeMatrixFlexibleDimensionsPropertiesNV")
                        .ok_or(LoadingError)?,
                ),
            })
        }
    }
}
impl InstanceFn {
    pub unsafe fn get_physical_device_cooperative_matrix_flexible_dimensions_properties_nv(
        &self,
        physical_device: PhysicalDevice,
        properties: impl ExtendUninit<CooperativeMatrixFlexibleDimensionsPropertiesNV>,
    ) -> crate::Result<()> {
        unsafe {
            try_extend_uninit(properties, |property_count, properties| {
                let result = (self
                    .get_physical_device_cooperative_matrix_flexible_dimensions_properties_nv)(
                    physical_device,
                    property_count,
                    properties as _,
                );

                match result {
                    VkResult::SUCCESS => Ok(()),
                    VkResult::INCOMPLETE => Ok(()),
                    err => Err(err),
                }
            })
        }
    }
}
