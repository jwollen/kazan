#![allow(unused_imports)]
use crate::*;
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
use kazan_sys::{vk::Result as VkResult, vk::*, *};
pub struct DeviceFn {
    set_latency_sleep_mode_nv: PFN_vkSetLatencySleepModeNV,
    latency_sleep_nv: PFN_vkLatencySleepNV,
    set_latency_marker_nv: PFN_vkSetLatencyMarkerNV,
    get_latency_timings_nv: PFN_vkGetLatencyTimingsNV,
    queue_notify_out_of_band_nv: PFN_vkQueueNotifyOutOfBandNV,
}
impl DeviceFn {
    pub unsafe fn load(
        load: impl Fn(&CStr) -> Option<PFN_vkVoidFunction>,
    ) -> core::result::Result<Self, LoadingError> {
        unsafe {
            Ok(Self {
                set_latency_sleep_mode_nv: transmute(
                    load(c"vkSetLatencySleepModeNV").ok_or(LoadingError)?,
                ),
                latency_sleep_nv: transmute(load(c"vkLatencySleepNV").ok_or(LoadingError)?),
                set_latency_marker_nv: transmute(
                    load(c"vkSetLatencyMarkerNV").ok_or(LoadingError)?,
                ),
                get_latency_timings_nv: transmute(
                    load(c"vkGetLatencyTimingsNV").ok_or(LoadingError)?,
                ),
                queue_notify_out_of_band_nv: transmute(
                    load(c"vkQueueNotifyOutOfBandNV").ok_or(LoadingError)?,
                ),
            })
        }
    }
}
impl DeviceFn {
    pub unsafe fn set_latency_sleep_mode_nv(
        &self,
        device: Device,
        swapchain: SwapchainKHR,
        sleep_mode_info: &LatencySleepModeInfoNV,
    ) -> crate::Result<()> {
        unsafe {
            let result = (self.set_latency_sleep_mode_nv)(device, swapchain, sleep_mode_info);

            match result {
                VkResult::SUCCESS => Ok(()),
                err => Err(err),
            }
        }
    }
    pub unsafe fn latency_sleep_nv(
        &self,
        device: Device,
        swapchain: SwapchainKHR,
        sleep_info: &LatencySleepInfoNV,
    ) -> crate::Result<()> {
        unsafe {
            let result = (self.latency_sleep_nv)(device, swapchain, sleep_info);

            match result {
                VkResult::SUCCESS => Ok(()),
                err => Err(err),
            }
        }
    }
    pub unsafe fn set_latency_marker_nv(
        &self,
        device: Device,
        swapchain: SwapchainKHR,
        latency_marker_info: &SetLatencyMarkerInfoNV,
    ) {
        unsafe { (self.set_latency_marker_nv)(device, swapchain, latency_marker_info) }
    }
    pub unsafe fn get_latency_timings_nv(
        &self,
        device: Device,
        swapchain: SwapchainKHR,
    ) -> GetLatencyMarkerInfoNV {
        unsafe {
            let mut latency_marker_info = core::mem::MaybeUninit::uninit();
            (self.get_latency_timings_nv)(device, swapchain, latency_marker_info.as_mut_ptr());
            latency_marker_info.assume_init()
        }
    }
    pub unsafe fn queue_notify_out_of_band_nv(
        &self,
        queue: Queue,
        queue_type_info: &OutOfBandQueueTypeInfoNV,
    ) {
        unsafe { (self.queue_notify_out_of_band_nv)(queue, queue_type_info) }
    }
}
