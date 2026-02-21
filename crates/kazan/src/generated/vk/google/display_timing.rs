#![allow(unused_imports)]
use crate::*;
use core::ffi::{c_char, c_int, c_void, CStr};
use kazan_sys::{vk::*, *};
pub struct DeviceFn {
    get_refresh_cycle_duration_google: PFN_vkGetRefreshCycleDurationGOOGLE,
    get_past_presentation_timing_google: PFN_vkGetPastPresentationTimingGOOGLE,
}
impl DeviceFn {
    pub unsafe fn get_refresh_cycle_duration_google(
        &self,
        device: Device,
        swapchain: SwapchainKHR,
        display_timing_properties: &mut RefreshCycleDurationGOOGLE,
    ) -> Result {
        unsafe {
            (self.get_refresh_cycle_duration_google)(device, swapchain, display_timing_properties)
        }
    }
    pub unsafe fn get_past_presentation_timing_google(
        &self,
        device: Device,
        swapchain: SwapchainKHR,
        presentation_timings: impl ExtendUninit<PastPresentationTimingGOOGLE>,
    ) -> Result {
        unsafe {
            try_extend_uninit(
                presentation_timings,
                |presentation_timing_count, presentation_timings| {
                    (self.get_past_presentation_timing_google)(
                        device,
                        swapchain,
                        presentation_timing_count,
                        presentation_timings as _,
                    )
                },
            )
        }
    }
}
