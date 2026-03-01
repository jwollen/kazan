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
    pub type PhysicalDeviceExternalFenceInfoKHR<'a> = PhysicalDeviceExternalFenceInfo<'a>;
    pub type ExternalFencePropertiesKHR<'a> = ExternalFenceProperties<'a>;
    pub type ExternalFenceHandleTypeFlagsKHR = ExternalFenceHandleTypeFlags;
    pub type ExternalFenceFeatureFlagsKHR = ExternalFenceFeatureFlags;
    pub type PFN_vkGetPhysicalDeviceExternalFencePropertiesKHR =
        PFN_vkGetPhysicalDeviceExternalFenceProperties;
}
pub struct InstanceFn {
    get_physical_device_external_fence_properties_khr:
        PFN_vkGetPhysicalDeviceExternalFenceProperties,
}
impl InstanceFn {
    pub unsafe fn load(
        load: impl Fn(&CStr) -> Option<PFN_vkVoidFunction>,
    ) -> core::result::Result<Self, LoadingError> {
        unsafe {
            Ok(Self {
                get_physical_device_external_fence_properties_khr: transmute(
                    load(c"vkGetPhysicalDeviceExternalFencePropertiesKHR").ok_or(LoadingError)?,
                ),
            })
        }
    }
}
impl InstanceFn {
    pub unsafe fn get_physical_device_external_fence_properties_khr(
        &self,
        physical_device: PhysicalDevice,
        external_fence_info: &PhysicalDeviceExternalFenceInfo<'_>,
    ) -> ExternalFenceProperties<'_> {
        unsafe {
            let mut external_fence_properties = core::mem::MaybeUninit::uninit();
            (self.get_physical_device_external_fence_properties_khr)(
                physical_device,
                external_fence_info,
                external_fence_properties.as_mut_ptr(),
            );
            external_fence_properties.assume_init()
        }
    }
}
