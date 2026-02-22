#![allow(unused_imports)]
use crate::*;
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
use kazan_sys::{vk::Result as VkResult, vk::*, *};
pub struct InstanceFn {
    get_physical_device_supported_framebuffer_mixed_samples_combinations_nv:
        PFN_vkGetPhysicalDeviceSupportedFramebufferMixedSamplesCombinationsNV,
}
impl InstanceFn {
    pub unsafe fn load(
        load: impl Fn(&CStr) -> Option<PFN_vkVoidFunction>,
    ) -> core::result::Result<Self, LoadingError> {
        unsafe {
            Ok(Self {
                get_physical_device_supported_framebuffer_mixed_samples_combinations_nv: transmute(
                    load(c"vkGetPhysicalDeviceSupportedFramebufferMixedSamplesCombinationsNV")
                        .ok_or(LoadingError)?,
                ),
            })
        }
    }
}
impl InstanceFn {
    pub unsafe fn get_physical_device_supported_framebuffer_mixed_samples_combinations_nv(
        &self,
        physical_device: PhysicalDevice,
        combinations: impl ExtendUninit<FramebufferMixedSamplesCombinationNV>,
    ) -> crate::Result<()> {
        unsafe {
            try_extend_uninit(combinations, |combination_count, combinations| {
                let result = (self
                    .get_physical_device_supported_framebuffer_mixed_samples_combinations_nv)(
                    physical_device,
                    combination_count,
                    combinations as _,
                );

                match result {
                    VkResult::SUCCESS => Ok(()),
                    VkResult::INCOMPLETE => Ok(()),
                    err => Err(err),
                }
            })
        }
    }
}
