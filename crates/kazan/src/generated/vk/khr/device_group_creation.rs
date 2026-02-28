#![allow(unused_imports)]
use crate::{vk::Result as VkResult, vk::*, *};
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
pub(super) mod defs {
    #![allow(non_camel_case_types, unused_imports)]
    use crate::{vk::*, *};
    use bitflags::bitflags;
    use core::ffi::{CStr, c_char, c_int, c_void};
    use core::marker::PhantomData;
    pub type PhysicalDeviceGroupPropertiesKHR<'a> = PhysicalDeviceGroupProperties<'a>;
    pub type DeviceGroupDeviceCreateInfoKHR<'a> = DeviceGroupDeviceCreateInfo<'a>;
    pub type PFN_vkEnumeratePhysicalDeviceGroupsKHR = PFN_vkEnumeratePhysicalDeviceGroups;
}
pub struct InstanceFn {
    enumerate_physical_device_groups_khr: PFN_vkEnumeratePhysicalDeviceGroups,
}
impl InstanceFn {
    pub unsafe fn load(
        load: impl Fn(&CStr) -> Option<PFN_vkVoidFunction>,
    ) -> core::result::Result<Self, LoadingError> {
        unsafe {
            Ok(Self {
                enumerate_physical_device_groups_khr: transmute(
                    load(c"vkEnumeratePhysicalDeviceGroupsKHR").ok_or(LoadingError)?,
                ),
            })
        }
    }
}
impl InstanceFn {
    pub unsafe fn enumerate_physical_device_groups_khr<'a>(
        &self,
        instance: Instance,
        physical_device_group_properties: impl ExtendUninit<PhysicalDeviceGroupProperties<'a>>,
    ) -> crate::Result<()> {
        unsafe {
            try_extend_uninit(
                physical_device_group_properties,
                |physical_device_group_count, physical_device_group_properties| {
                    let result = (self.enumerate_physical_device_groups_khr)(
                        instance,
                        physical_device_group_count,
                        physical_device_group_properties as _,
                    );

                    match result {
                        VkResult::SUCCESS => Ok(()),
                        VkResult::INCOMPLETE => Ok(()),
                        err => Err(err),
                    }
                },
            )
        }
    }
}
