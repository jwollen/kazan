//! <https://registry.khronos.org/vulkan/specs/latest/man/html/VK_KHR_external_fence_capabilities.html>
#![allow(unused_imports)]
use crate::{vk::Result as VkResult, vk::*, *};
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
use core::ptr;

pub const EXTENSION_NAME: &CStr = c"VK_KHR_external_fence_capabilities";

pub(super) mod defs {
    #![allow(non_camel_case_types, unused_imports)]
    use crate::{vk::*, *};
    use core::ffi::{CStr, c_char, c_int, c_void};
    use core::fmt;
    use core::marker::PhantomData;
    use core::ptr;

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPhysicalDeviceExternalFenceInfoKHR.html>
    pub type PhysicalDeviceExternalFenceInfoKHR<'a> = PhysicalDeviceExternalFenceInfo<'a>;
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkExternalFencePropertiesKHR.html>
    pub type ExternalFencePropertiesKHR<'a> = ExternalFenceProperties<'a>;
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkExternalFenceHandleTypeFlagsKHR.html>
    pub type ExternalFenceHandleTypeFlagsKHR = ExternalFenceHandleTypeFlags;
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkExternalFenceFeatureFlagsKHR.html>
    pub type ExternalFenceFeatureFlagsKHR = ExternalFenceFeatureFlags;
    pub type PFN_vkGetPhysicalDeviceExternalFencePropertiesKHR =
        PFN_vkGetPhysicalDeviceExternalFenceProperties;
}

#[cfg(feature = "ffi")]
pub(super) mod ffi {
    #![allow(non_camel_case_types)]
    use super::defs::*;

    pub type VkPhysicalDeviceExternalFenceInfoKHR = PhysicalDeviceExternalFenceInfoKHR<'static>;
    pub type VkExternalFencePropertiesKHR = ExternalFencePropertiesKHR<'static>;
    pub type VkExternalFenceHandleTypeFlagsKHR = ExternalFenceHandleTypeFlagsKHR;
    pub type VkExternalFenceFeatureFlagsKHR = ExternalFenceFeatureFlagsKHR;
}

pub struct InstanceFn {
    get_physical_device_external_fence_properties: PFN_vkGetPhysicalDeviceExternalFenceProperties,
}

impl LoadInstanceFn for InstanceFn {
    unsafe fn load_with(
        load: impl Fn(&CStr) -> Option<PFN_vkVoidFunction>,
    ) -> core::result::Result<Self, MissingEntryPointError> {
        unsafe {
            Ok(Self {
                get_physical_device_external_fence_properties: transmute(
                    load(c"vkGetPhysicalDeviceExternalFencePropertiesKHR")
                        .ok_or(MissingEntryPointError)?,
                ),
            })
        }
    }
}

impl InstanceFn {
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkGetPhysicalDeviceExternalFencePropertiesKHR.html>
    #[inline]
    pub unsafe fn get_physical_device_external_fence_properties(
        &self,
        physical_device: PhysicalDevice,
        external_fence_info: &PhysicalDeviceExternalFenceInfo<'_>,
        external_fence_properties: &mut ExternalFenceProperties<'_>,
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
