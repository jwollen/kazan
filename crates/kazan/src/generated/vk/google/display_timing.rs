#![allow(unused_imports)]
use crate::*;
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
use kazan_sys::{vk::*, *};
pub struct DeviceFn {
    get_refresh_cycle_duration_google: PFN_vkGetRefreshCycleDurationGOOGLE,
    get_past_presentation_timing_google: PFN_vkGetPastPresentationTimingGOOGLE,
}
impl DeviceFn {
    pub unsafe fn load(
        load: impl Fn(&CStr) -> Option<PFN_vkVoidFunction>,
    ) -> core::result::Result<Self, LoadingError> {
        unsafe {
            Ok(Self {
                get_refresh_cycle_duration_google: transmute(
                    load(c"vkGetRefreshCycleDurationGOOGLE").ok_or(LoadingError)?,
                ),
                get_past_presentation_timing_google: transmute(
                    load(c"vkGetPastPresentationTimingGOOGLE").ok_or(LoadingError)?,
                ),
            })
        }
    }
}
impl DeviceFn {
    pub unsafe fn get_refresh_cycle_duration_google(
        &self,
        device: Device,
        swapchain: SwapchainKHR,
        display_timing_properties: &mut RefreshCycleDurationGOOGLE,
    ) -> crate::Result<()> {
        unsafe {
            result((self.get_refresh_cycle_duration_google)(
                device,
                swapchain,
                display_timing_properties,
            ))
        }
    }
    pub unsafe fn get_past_presentation_timing_google(
        &self,
        device: Device,
        swapchain: SwapchainKHR,
        presentation_timings: impl ExtendUninit<PastPresentationTimingGOOGLE>,
    ) -> crate::Result<()> {
        unsafe {
            try_extend_uninit(
                presentation_timings,
                |presentation_timing_count, presentation_timings| {
                    result((self.get_past_presentation_timing_google)(
                        device,
                        swapchain,
                        presentation_timing_count,
                        presentation_timings as _,
                    ))
                },
            )
        }
    }
}
