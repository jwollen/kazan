#![allow(unused_imports)]
use crate::{vk::Result as VkResult, vk::*, *};
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
pub(super) mod defs {
    #![allow(non_camel_case_types, unused_imports)]
    use crate::{vk::*, *};
    use core::ffi::{CStr, c_char, c_int, c_void};
    use core::fmt;
    use core::marker::PhantomData;
    pub type ExternalMemoryPropertiesKHR = ExternalMemoryProperties;
    pub type PhysicalDeviceExternalImageFormatInfoKHR<'a> =
        PhysicalDeviceExternalImageFormatInfo<'a>;
    pub type ExternalImageFormatPropertiesKHR<'a> = ExternalImageFormatProperties<'a>;
    pub type PhysicalDeviceExternalBufferInfoKHR<'a> = PhysicalDeviceExternalBufferInfo<'a>;
    pub type ExternalBufferPropertiesKHR<'a> = ExternalBufferProperties<'a>;
    pub type PhysicalDeviceIDPropertiesKHR<'a> = PhysicalDeviceIDProperties<'a>;
    pub type ExternalMemoryHandleTypeFlagsKHR = ExternalMemoryHandleTypeFlags;
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
    ) -> core::result::Result<Self, LoadingError> {
        unsafe {
            Ok(Self {
                get_physical_device_external_buffer_properties_khr: transmute(
                    load(c"vkGetPhysicalDeviceExternalBufferPropertiesKHR").ok_or(LoadingError)?,
                ),
            })
        }
    }
}
impl InstanceFn {
    pub unsafe fn get_physical_device_external_buffer_properties_khr(
        &self,
        physical_device: PhysicalDevice,
        external_buffer_info: &PhysicalDeviceExternalBufferInfo<'_>,
    ) -> ExternalBufferProperties<'_> {
        unsafe {
            let mut external_buffer_properties = core::mem::MaybeUninit::uninit();
            (self.get_physical_device_external_buffer_properties_khr)(
                physical_device,
                external_buffer_info,
                external_buffer_properties.as_mut_ptr(),
            );
            external_buffer_properties.assume_init()
        }
    }
}
