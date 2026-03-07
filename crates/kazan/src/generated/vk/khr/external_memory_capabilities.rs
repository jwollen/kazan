#![allow(unused_imports)]
use crate::{vk::Result as VkResult, vk::*, *};
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;

pub const EXTENSION_NAME: &CStr = c"VK_KHR_external_memory_capabilities";

pub(super) mod defs {
    #![allow(non_camel_case_types, unused_imports)]
    use crate::{vk::*, *};
    use core::ffi::{CStr, c_char, c_int, c_void};
    use core::fmt;
    use core::marker::PhantomData;

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkExternalMemoryPropertiesKHR.html>
    pub type ExternalMemoryPropertiesKHR = ExternalMemoryProperties;
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPhysicalDeviceExternalImageFormatInfoKHR.html>
    pub type PhysicalDeviceExternalImageFormatInfoKHR<'a> =
        PhysicalDeviceExternalImageFormatInfo<'a>;
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkExternalImageFormatPropertiesKHR.html>
    pub type ExternalImageFormatPropertiesKHR<'a> = ExternalImageFormatProperties<'a>;
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPhysicalDeviceExternalBufferInfoKHR.html>
    pub type PhysicalDeviceExternalBufferInfoKHR<'a> = PhysicalDeviceExternalBufferInfo<'a>;
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkExternalBufferPropertiesKHR.html>
    pub type ExternalBufferPropertiesKHR<'a> = ExternalBufferProperties<'a>;
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPhysicalDeviceIDPropertiesKHR.html>
    pub type PhysicalDeviceIDPropertiesKHR<'a> = PhysicalDeviceIDProperties<'a>;
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkExternalMemoryHandleTypeFlagsKHR.html>
    pub type ExternalMemoryHandleTypeFlagsKHR = ExternalMemoryHandleTypeFlags;
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkExternalMemoryFeatureFlagsKHR.html>
    pub type ExternalMemoryFeatureFlagsKHR = ExternalMemoryFeatureFlags;
    pub type PFN_vkGetPhysicalDeviceExternalBufferPropertiesKHR =
        PFN_vkGetPhysicalDeviceExternalBufferProperties;
}

pub struct InstanceFn {
    get_physical_device_external_buffer_properties_khr:
        PFN_vkGetPhysicalDeviceExternalBufferProperties,
}

impl InstanceFn {
    pub unsafe fn load(
        load: impl Fn(&CStr) -> Option<PFN_vkVoidFunction>,
    ) -> core::result::Result<Self, MissingEntryPointError> {
        unsafe {
            Ok(Self {
                get_physical_device_external_buffer_properties_khr: transmute(
                    load(c"vkGetPhysicalDeviceExternalBufferPropertiesKHR")
                        .ok_or(MissingEntryPointError)?,
                ),
            })
        }
    }
}

impl InstanceFn {
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkGetPhysicalDeviceExternalBufferPropertiesKHR.html>
    pub unsafe fn get_physical_device_external_buffer_properties_khr(
        &self,
        physical_device: PhysicalDevice,
        external_buffer_info: &PhysicalDeviceExternalBufferInfo<'_>,
        external_buffer_properties: &mut ExternalBufferProperties<'_>,
    ) {
        unsafe {
            (self.get_physical_device_external_buffer_properties_khr)(
                physical_device,
                external_buffer_info,
                external_buffer_properties,
            )
        }
    }
}
