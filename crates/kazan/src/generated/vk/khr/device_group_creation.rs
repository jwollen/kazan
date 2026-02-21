#![allow(unused_imports)]
use crate::*;
use core::ffi::{c_char, c_int, c_void, CStr};
use kazan_sys::{vk::*, *};
pub struct InstanceFn {
    enumerate_physical_device_groups: PFN_vkEnumeratePhysicalDeviceGroups,
}
impl InstanceFn {
    pub unsafe fn enumerate_physical_device_groups(
        &self,
        instance: Instance,
        physical_device_group_properties: impl ExtendUninit<PhysicalDeviceGroupProperties>,
    ) -> Result {
        unsafe {
            try_extend_uninit(
                physical_device_group_properties,
                |physical_device_group_count, physical_device_group_properties| {
                    (self.enumerate_physical_device_groups)(
                        instance,
                        physical_device_group_count,
                        physical_device_group_properties as _,
                    )
                },
            )
        }
    }
}
