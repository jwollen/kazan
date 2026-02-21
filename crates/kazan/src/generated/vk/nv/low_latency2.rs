#![allow(unused_imports)]
use crate::*;
use core::ffi::{c_char, c_int, c_void, CStr};
use kazan_sys::{vk::*, *};
pub struct DeviceFn {
    set_latency_sleep_mode_nv: PFN_vkSetLatencySleepModeNV,
    latency_sleep_nv: PFN_vkLatencySleepNV,
    set_latency_marker_nv: PFN_vkSetLatencyMarkerNV,
    get_latency_timings_nv: PFN_vkGetLatencyTimingsNV,
    queue_notify_out_of_band_nv: PFN_vkQueueNotifyOutOfBandNV,
}
impl DeviceFn {
    pub unsafe fn set_latency_sleep_mode_nv(
        &self,
        device: Device,
        swapchain: SwapchainKHR,
        sleep_mode_info: &LatencySleepModeInfoNV,
    ) -> Result {
        unsafe { (self.set_latency_sleep_mode_nv)(device, swapchain, sleep_mode_info) }
    }
    pub unsafe fn latency_sleep_nv(
        &self,
        device: Device,
        swapchain: SwapchainKHR,
        sleep_info: &LatencySleepInfoNV,
    ) -> Result {
        unsafe { (self.latency_sleep_nv)(device, swapchain, sleep_info) }
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
        latency_marker_info: &mut GetLatencyMarkerInfoNV,
    ) {
        unsafe { (self.get_latency_timings_nv)(device, swapchain, latency_marker_info) }
    }
    pub unsafe fn queue_notify_out_of_band_nv(
        &self,
        queue: Queue,
        queue_type_info: &OutOfBandQueueTypeInfoNV,
    ) {
        unsafe { (self.queue_notify_out_of_band_nv)(queue, queue_type_info) }
    }
}
