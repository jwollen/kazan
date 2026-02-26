#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{c_char, c_int, c_void};
use core::marker::PhantomData;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct LatencySleepModeInfoNV<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub low_latency_mode: Bool32,
    pub low_latency_boost: Bool32,
    pub minimum_interval_us: u32,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for LatencySleepModeInfoNV<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::LATENCY_SLEEP_MODE_INFO_NV,
            p_next: core::ptr::null(),
            low_latency_mode: Default::default(),
            low_latency_boost: Default::default(),
            minimum_interval_us: Default::default(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct LatencySleepInfoNV<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub signal_semaphore: Semaphore,
    pub value: u64,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for LatencySleepInfoNV<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::LATENCY_SLEEP_INFO_NV,
            p_next: core::ptr::null(),
            signal_semaphore: Default::default(),
            value: Default::default(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SetLatencyMarkerInfoNV<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub present_id: u64,
    pub marker: LatencyMarkerNV,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for SetLatencyMarkerInfoNV<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::SET_LATENCY_MARKER_INFO_NV,
            p_next: core::ptr::null(),
            present_id: Default::default(),
            marker: Default::default(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct GetLatencyMarkerInfoNV<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub timing_count: u32,
    pub p_timings: *mut LatencyTimingsFrameReportNV<'a>,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for GetLatencyMarkerInfoNV<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::GET_LATENCY_MARKER_INFO_NV,
            p_next: core::ptr::null(),
            timing_count: Default::default(),
            p_timings: core::ptr::null_mut(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct LatencyTimingsFrameReportNV<'a> {
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
    pub _marker: PhantomData<&'a ()>,
}
impl Default for LatencyTimingsFrameReportNV<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::LATENCY_TIMINGS_FRAME_REPORT_NV,
            p_next: core::ptr::null_mut(),
            present_id: Default::default(),
            input_sample_time_us: Default::default(),
            sim_start_time_us: Default::default(),
            sim_end_time_us: Default::default(),
            render_submit_start_time_us: Default::default(),
            render_submit_end_time_us: Default::default(),
            present_start_time_us: Default::default(),
            present_end_time_us: Default::default(),
            driver_start_time_us: Default::default(),
            driver_end_time_us: Default::default(),
            os_render_queue_start_time_us: Default::default(),
            os_render_queue_end_time_us: Default::default(),
            gpu_render_start_time_us: Default::default(),
            gpu_render_end_time_us: Default::default(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct OutOfBandQueueTypeInfoNV<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub queue_type: OutOfBandQueueTypeNV,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for OutOfBandQueueTypeInfoNV<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::OUT_OF_BAND_QUEUE_TYPE_INFO_NV,
            p_next: core::ptr::null(),
            queue_type: Default::default(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct LatencySubmissionPresentIdNV<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub present_id: u64,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for LatencySubmissionPresentIdNV<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::LATENCY_SUBMISSION_PRESENT_ID_NV,
            p_next: core::ptr::null(),
            present_id: Default::default(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SwapchainLatencyCreateInfoNV<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub latency_mode_enable: Bool32,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for SwapchainLatencyCreateInfoNV<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::SWAPCHAIN_LATENCY_CREATE_INFO_NV,
            p_next: core::ptr::null(),
            latency_mode_enable: Default::default(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct LatencySurfaceCapabilitiesNV<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub present_mode_count: u32,
    pub p_present_modes: *mut PresentModeKHR,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for LatencySurfaceCapabilitiesNV<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::LATENCY_SURFACE_CAPABILITIES_NV,
            p_next: core::ptr::null(),
            present_mode_count: Default::default(),
            p_present_modes: core::ptr::null_mut(),
            _marker: PhantomData,
        }
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
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
#[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct OutOfBandQueueTypeNV(i32);
impl OutOfBandQueueTypeNV {
    pub const RENDER_NV: Self = Self(0);
    pub const PRESENT_NV: Self = Self(1);
}
pub type PFN_vkSetLatencySleepModeNV = unsafe extern "system" fn(
    device: Device,
    swapchain: SwapchainKHR,
    p_sleep_mode_info: *const LatencySleepModeInfoNV<'_>,
) -> Result;
pub type PFN_vkLatencySleepNV = unsafe extern "system" fn(
    device: Device,
    swapchain: SwapchainKHR,
    p_sleep_info: *const LatencySleepInfoNV<'_>,
) -> Result;
pub type PFN_vkSetLatencyMarkerNV = unsafe extern "system" fn(
    device: Device,
    swapchain: SwapchainKHR,
    p_latency_marker_info: *const SetLatencyMarkerInfoNV<'_>,
);
pub type PFN_vkGetLatencyTimingsNV = unsafe extern "system" fn(
    device: Device,
    swapchain: SwapchainKHR,
    p_latency_marker_info: *mut GetLatencyMarkerInfoNV<'_>,
);
pub type PFN_vkQueueNotifyOutOfBandNV =
    unsafe extern "system" fn(queue: Queue, p_queue_type_info: *const OutOfBandQueueTypeInfoNV<'_>);
