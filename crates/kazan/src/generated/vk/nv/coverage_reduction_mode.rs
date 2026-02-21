#![allow(unused_imports)]
use crate::*;
use core::ffi::{c_char, c_int, c_void, CStr};
use kazan_sys::{vk::*, *};
pub struct InstanceFn {
    get_physical_device_supported_framebuffer_mixed_samples_combinations_nv:
        PFN_vkGetPhysicalDeviceSupportedFramebufferMixedSamplesCombinationsNV,
}
impl InstanceFn {
    pub unsafe fn get_physical_device_supported_framebuffer_mixed_samples_combinations_nv(
        &self,
        physical_device: PhysicalDevice,
        combinations: impl ExtendUninit<FramebufferMixedSamplesCombinationNV>,
    ) -> Result {
        unsafe {
            try_extend_uninit(combinations, |combination_count, combinations| {
                (self.get_physical_device_supported_framebuffer_mixed_samples_combinations_nv)(
                    physical_device,
                    combination_count,
                    combinations as _,
                )
            })
        }
    }
}
