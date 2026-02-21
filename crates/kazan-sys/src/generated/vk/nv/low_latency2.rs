#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{c_char, c_int, c_void};
#[repr(C)]
#[derive(Copy, Clone)]
pub struct LatencySleepModeInfoNV {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub low_latency_mode: Bool32,
    pub low_latency_boost: Bool32,
    pub minimum_interval_us: u32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct LatencySleepInfoNV {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub signal_semaphore: Semaphore,
    pub value: u64,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SetLatencyMarkerInfoNV {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub present_id: u64,
    pub marker: LatencyMarkerNV,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct GetLatencyMarkerInfoNV {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub timing_count: u32,
    pub p_timings: *mut LatencyTimingsFrameReportNV,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct LatencyTimingsFrameReportNV {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub present_id: u64,
    pub input_sample_time_us: u64,
    pub sim_start_time_us: u64,
    pub sim_end_time_us: u64,
    pub render_submit_start_time_us: u64,
    pub render_submit_end_time_us: u64,
    pub present_start_time_us: u64,
    pub present_end_time_us: u64,
    pub driver_start_time_us: u64,
    pub driver_end_time_us: u64,
    pub os_render_queue_start_time_us: u64,
    pub os_render_queue_end_time_us: u64,
    pub gpu_render_start_time_us: u64,
    pub gpu_render_end_time_us: u64,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct OutOfBandQueueTypeInfoNV {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub queue_type: OutOfBandQueueTypeNV,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct LatencySubmissionPresentIdNV {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub present_id: u64,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SwapchainLatencyCreateInfoNV {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub latency_mode_enable: Bool32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct LatencySurfaceCapabilitiesNV {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub present_mode_count: u32,
    pub p_present_modes: *mut PresentModeKHR,
}
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct LatencyMarkerNV(i32);
impl LatencyMarkerNV {
    pub const SIMULATION_START_NV: Self = Self(0);
    pub const SIMULATION_END_NV: Self = Self(1);
    pub const RENDERSUBMIT_START_NV: Self = Self(2);
    pub const RENDERSUBMIT_END_NV: Self = Self(3);
    pub const PRESENT_START_NV: Self = Self(4);
    pub const PRESENT_END_NV: Self = Self(5);
    pub const INPUT_SAMPLE_NV: Self = Self(6);
    pub const TRIGGER_FLASH_NV: Self = Self(7);
    pub const OUT_OF_BAND_RENDERSUBMIT_START_NV: Self = Self(8);
    pub const OUT_OF_BAND_RENDERSUBMIT_END_NV: Self = Self(9);
    pub const OUT_OF_BAND_PRESENT_START_NV: Self = Self(10);
    pub const OUT_OF_BAND_PRESENT_END_NV: Self = Self(11);
}
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct OutOfBandQueueTypeNV(i32);
impl OutOfBandQueueTypeNV {
    pub const RENDER_NV: Self = Self(0);
    pub const PRESENT_NV: Self = Self(1);
}
pub type PFN_vkSetLatencySleepModeNV = unsafe extern "system" fn(
    device: Device,
    swapchain: SwapchainKHR,
    p_sleep_mode_info: *const LatencySleepModeInfoNV,
) -> Result;
pub type PFN_vkLatencySleepNV = unsafe extern "system" fn(
    device: Device,
    swapchain: SwapchainKHR,
    p_sleep_info: *const LatencySleepInfoNV,
) -> Result;
pub type PFN_vkSetLatencyMarkerNV = unsafe extern "system" fn(
    device: Device,
    swapchain: SwapchainKHR,
    p_latency_marker_info: *const SetLatencyMarkerInfoNV,
);
pub type PFN_vkGetLatencyTimingsNV = unsafe extern "system" fn(
    device: Device,
    swapchain: SwapchainKHR,
    p_latency_marker_info: *mut GetLatencyMarkerInfoNV,
);
pub type PFN_vkQueueNotifyOutOfBandNV =
    unsafe extern "system" fn(queue: Queue, p_queue_type_info: *const OutOfBandQueueTypeInfoNV);
