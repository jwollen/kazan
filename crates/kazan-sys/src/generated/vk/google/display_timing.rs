#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{c_char, c_int, c_void};
use core::marker::PhantomData;
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct RefreshCycleDurationGOOGLE {
    pub refresh_duration: u64,
}
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct PastPresentationTimingGOOGLE {
    pub present_id: u32,
    pub desired_present_time: u64,
    pub actual_present_time: u64,
    pub earliest_present_time: u64,
    pub present_margin: u64,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PresentTimesInfoGOOGLE<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub swapchain_count: u32,
    pub p_times: *const PresentTimeGOOGLE,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for PresentTimesInfoGOOGLE<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::PRESENT_TIMES_INFO_GOOGLE,
            p_next: core::ptr::null(),
            swapchain_count: Default::default(),
            p_times: core::ptr::null(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct PresentTimeGOOGLE {
    pub present_id: u32,
    pub desired_present_time: u64,
}
pub type PFN_vkGetRefreshCycleDurationGOOGLE = unsafe extern "system" fn(
    device: Device,
    swapchain: SwapchainKHR,
    p_display_timing_properties: *mut RefreshCycleDurationGOOGLE,
) -> Result;
pub type PFN_vkGetPastPresentationTimingGOOGLE = unsafe extern "system" fn(
    device: Device,
    swapchain: SwapchainKHR,
    p_presentation_timing_count: *mut u32,
    p_presentation_timings: *mut PastPresentationTimingGOOGLE,
) -> Result;
