//! <https://registry.khronos.org/vulkan/specs/latest/man/html/VK_KHR_external_semaphore_capabilities.html>
#![allow(unused_imports)]
use crate::{vk::Result as VkResult, vk::*, *};
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
use core::ptr;

pub const EXTENSION_NAME: &CStr = c"VK_KHR_external_semaphore_capabilities";

pub(super) mod defs {
    #![allow(non_camel_case_types, unused_imports)]
    use crate::{vk::*, *};
    use core::ffi::{CStr, c_char, c_int, c_void};
    use core::fmt;
    use core::marker::PhantomData;
    use core::ptr;

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPhysicalDeviceExternalSemaphoreInfoKHR.html>
    pub type PhysicalDeviceExternalSemaphoreInfoKHR<'a> = PhysicalDeviceExternalSemaphoreInfo<'a>;
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkExternalSemaphorePropertiesKHR.html>
    pub type ExternalSemaphorePropertiesKHR<'a> = ExternalSemaphoreProperties<'a>;
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkExternalSemaphoreHandleTypeFlagsKHR.html>
    pub type ExternalSemaphoreHandleTypeFlagsKHR = ExternalSemaphoreHandleTypeFlags;
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkExternalSemaphoreFeatureFlagsKHR.html>
    pub type ExternalSemaphoreFeatureFlagsKHR = ExternalSemaphoreFeatureFlags;
    pub type PFN_vkGetPhysicalDeviceExternalSemaphorePropertiesKHR =
        PFN_vkGetPhysicalDeviceExternalSemaphoreProperties;
}

#[cfg(feature = "ffi")]
pub(super) mod ffi {
    #![allow(non_camel_case_types)]
    use super::defs::*;

    pub type VkPhysicalDeviceExternalSemaphoreInfoKHR =
        PhysicalDeviceExternalSemaphoreInfoKHR<'static>;
    pub type VkExternalSemaphorePropertiesKHR = ExternalSemaphorePropertiesKHR<'static>;
    pub type VkExternalSemaphoreHandleTypeFlagsKHR = ExternalSemaphoreHandleTypeFlagsKHR;
    pub type VkExternalSemaphoreFeatureFlagsKHR = ExternalSemaphoreFeatureFlagsKHR;
}

pub struct InstanceFn {
    get_physical_device_external_semaphore_properties_khr:
        PFN_vkGetPhysicalDeviceExternalSemaphoreProperties,
}

impl InstanceFn {
    pub unsafe fn load(
        load: impl Fn(&CStr) -> Option<PFN_vkVoidFunction>,
    ) -> core::result::Result<Self, MissingEntryPointError> {
        unsafe {
            Ok(Self {
                get_physical_device_external_semaphore_properties_khr: transmute(
                    load(c"vkGetPhysicalDeviceExternalSemaphorePropertiesKHR")
                        .ok_or(MissingEntryPointError)?,
                ),
            })
        }
    }
}

impl InstanceFn {
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkGetPhysicalDeviceExternalSemaphorePropertiesKHR.html>
    #[inline]
    pub unsafe fn get_physical_device_external_semaphore_properties_khr(
        &self,
        physical_device: PhysicalDevice,
        external_semaphore_info: &PhysicalDeviceExternalSemaphoreInfo<'_>,
        external_semaphore_properties: &mut ExternalSemaphoreProperties<'_>,
    ) {
        unsafe {
            (self.get_physical_device_external_semaphore_properties_khr)(
                physical_device,
                external_semaphore_info,
                external_semaphore_properties,
            )
        }
    }
}
