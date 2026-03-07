//! <https://registry.khronos.org/vulkan/specs/latest/man/html/VK_KHR_device_group_creation.html>
#![allow(unused_imports)]
use crate::{vk::Result as VkResult, vk::*, *};
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;

pub const EXTENSION_NAME: &CStr = c"VK_KHR_device_group_creation";

pub(super) mod defs {
    #![allow(non_camel_case_types, unused_imports)]
    use crate::{vk::*, *};
    use core::ffi::{CStr, c_char, c_int, c_void};
    use core::fmt;
    use core::marker::PhantomData;

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPhysicalDeviceGroupPropertiesKHR.html>
    pub type PhysicalDeviceGroupPropertiesKHR<'a> = PhysicalDeviceGroupProperties<'a>;
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkDeviceGroupDeviceCreateInfoKHR.html>
    pub type DeviceGroupDeviceCreateInfoKHR<'a> = DeviceGroupDeviceCreateInfo<'a>;
    pub type PFN_vkEnumeratePhysicalDeviceGroupsKHR = PFN_vkEnumeratePhysicalDeviceGroups;
}

pub struct InstanceFn {
    enumerate_physical_device_groups_khr: PFN_vkEnumeratePhysicalDeviceGroups,
}

impl InstanceFn {
    pub unsafe fn load(
        load: impl Fn(&CStr) -> Option<PFN_vkVoidFunction>,
    ) -> core::result::Result<Self, MissingEntryPointError> {
        unsafe {
            Ok(Self {
                enumerate_physical_device_groups_khr: transmute(
                    load(c"vkEnumeratePhysicalDeviceGroupsKHR").ok_or(MissingEntryPointError)?,
                ),
            })
        }
    }
}

impl InstanceFn {
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkEnumeratePhysicalDeviceGroupsKHR.html>
    #[inline]
    pub unsafe fn enumerate_physical_device_groups_khr<'a>(
        &self,
        instance: Instance,
        mut physical_device_group_properties: impl ExtendUninit<PhysicalDeviceGroupProperties<'a>>,
    ) -> crate::Result<()> {
        unsafe {
            let call = |physical_device_group_count, physical_device_group_properties| {
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
            };
            let mut len = 0;
            call(&mut len, std::ptr::null_mut())?;
            let capacity = len.try_into().expect("failed to convert `N` to usize");
            let physical_device_group_properties_buf =
                physical_device_group_properties.reserve(capacity);
            len = physical_device_group_properties_buf
                .len()
                .try_into()
                .unwrap();
            let result = call(
                &mut len,
                physical_device_group_properties_buf.as_mut_ptr() as *mut _,
            )?;
            physical_device_group_properties.set_len(len.try_into().unwrap());
            Ok(result)
        }
    }
}
