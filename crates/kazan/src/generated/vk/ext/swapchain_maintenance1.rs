//! <https://registry.khronos.org/vulkan/specs/latest/man/html/VK_EXT_swapchain_maintenance1.html>
#![allow(unused_imports)]
use crate::{vk::Result as VkResult, vk::*, *};
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
use core::ptr;

pub const EXTENSION_NAME: &CStr = c"VK_EXT_swapchain_maintenance1";

pub(super) mod defs {
    #![allow(non_camel_case_types, unused_imports)]
    use crate::{vk::*, *};
    use core::ffi::{CStr, c_char, c_int, c_void};
    use core::fmt;
    use core::marker::PhantomData;
    use core::ptr;

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPhysicalDeviceSwapchainMaintenance1FeaturesEXT.html>
    pub type PhysicalDeviceSwapchainMaintenance1FeaturesEXT<'a> =
        PhysicalDeviceSwapchainMaintenance1FeaturesKHR<'a>;
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkSwapchainPresentFenceInfoEXT.html>
    pub type SwapchainPresentFenceInfoEXT<'a> = SwapchainPresentFenceInfoKHR<'a>;
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkSwapchainPresentModesCreateInfoEXT.html>
    pub type SwapchainPresentModesCreateInfoEXT<'a> = SwapchainPresentModesCreateInfoKHR<'a>;
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkSwapchainPresentModeInfoEXT.html>
    pub type SwapchainPresentModeInfoEXT<'a> = SwapchainPresentModeInfoKHR<'a>;
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkSwapchainPresentScalingCreateInfoEXT.html>
    pub type SwapchainPresentScalingCreateInfoEXT<'a> = SwapchainPresentScalingCreateInfoKHR<'a>;
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkReleaseSwapchainImagesInfoEXT.html>
    pub type ReleaseSwapchainImagesInfoEXT<'a> = ReleaseSwapchainImagesInfoKHR<'a>;
    pub type PFN_vkReleaseSwapchainImagesEXT = PFN_vkReleaseSwapchainImagesKHR;
}

pub struct DeviceFn {
    release_swapchain_images_ext: PFN_vkReleaseSwapchainImagesKHR,
}

impl DeviceFn {
    pub unsafe fn load(
        load: impl Fn(&CStr) -> Option<PFN_vkVoidFunction>,
    ) -> core::result::Result<Self, MissingEntryPointError> {
        unsafe {
            Ok(Self {
                release_swapchain_images_ext: transmute(
                    load(c"vkReleaseSwapchainImagesEXT").ok_or(MissingEntryPointError)?,
                ),
            })
        }
    }
}

impl DeviceFn {
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkReleaseSwapchainImagesEXT.html>
    #[inline]
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
