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
    pub type PhysicalDeviceSwapchainMaintenance1FeaturesEXT<'a> =
        PhysicalDeviceSwapchainMaintenance1FeaturesKHR<'a>;
    pub type SwapchainPresentFenceInfoEXT<'a> = SwapchainPresentFenceInfoKHR<'a>;
    pub type SwapchainPresentModesCreateInfoEXT<'a> = SwapchainPresentModesCreateInfoKHR<'a>;
    pub type SwapchainPresentModeInfoEXT<'a> = SwapchainPresentModeInfoKHR<'a>;
    pub type SwapchainPresentScalingCreateInfoEXT<'a> = SwapchainPresentScalingCreateInfoKHR<'a>;
    pub type ReleaseSwapchainImagesInfoEXT<'a> = ReleaseSwapchainImagesInfoKHR<'a>;
    pub type PFN_vkReleaseSwapchainImagesEXT = PFN_vkReleaseSwapchainImagesKHR;
}
pub struct DeviceFn {
    release_swapchain_images_ext: PFN_vkReleaseSwapchainImagesKHR,
}
impl DeviceFn {
    pub unsafe fn load(
        load: impl Fn(&CStr) -> Option<PFN_vkVoidFunction>,
    ) -> core::result::Result<Self, LoadingError> {
        unsafe {
            Ok(Self {
                release_swapchain_images_ext: transmute(
                    load(c"vkReleaseSwapchainImagesEXT").ok_or(LoadingError)?,
                ),
            })
        }
    }
}
impl DeviceFn {
    pub unsafe fn release_swapchain_images_ext(
        &self,
        device: Device,
        release_info: &ReleaseSwapchainImagesInfoKHR<'_>,
    ) -> crate::Result<()> {
        unsafe {
            let result = (self.release_swapchain_images_ext)(device, release_info);

            match result {
                VkResult::SUCCESS => Ok(()),
                err => Err(err),
            }
        }
    }
}
