#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use std::ffi::{c_char, c_int, c_void};
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDevicePresentTimingFeaturesEXT {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub present_timing: Bool32,
    pub present_at_absolute_time: Bool32,
    pub present_at_relative_time: Bool32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PresentTimingSurfaceCapabilitiesEXT {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub present_timing_supported: Bool32,
    pub present_at_absolute_time_supported: Bool32,
    pub present_at_relative_time_supported: Bool32,
    pub present_stage_queries: PresentStageFlagsEXT,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SwapchainTimingPropertiesEXT {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub refresh_duration: u64,
    pub refresh_interval: u64,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SwapchainTimeDomainPropertiesEXT {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub time_domain_count: u32,
    pub p_time_domains: *mut TimeDomainKHR,
    pub p_time_domain_ids: *mut u64,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PresentStageTimeEXT {
    pub stage: PresentStageFlagsEXT,
    pub time: u64,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PastPresentationTimingInfoEXT {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: PastPresentationTimingFlagsEXT,
    pub swapchain: SwapchainKHR,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PastPresentationTimingPropertiesEXT {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub timing_properties_counter: u64,
    pub time_domains_counter: u64,
    pub presentation_timing_count: u32,
    pub p_presentation_timings: *mut PastPresentationTimingEXT,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PastPresentationTimingEXT {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub present_id: u64,
    pub target_time: u64,
    pub present_stage_count: u32,
    pub p_present_stages: *mut PresentStageTimeEXT,
    pub time_domain: TimeDomainKHR,
    pub time_domain_id: u64,
    pub report_complete: Bool32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PresentTimingsInfoEXT {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub swapchain_count: u32,
    pub p_timing_infos: *const PresentTimingInfoEXT,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PresentTimingInfoEXT {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: PresentTimingInfoFlagsEXT,
    pub target_time: u64,
    pub time_domain_id: u64,
    pub present_stage_queries: PresentStageFlagsEXT,
    pub target_time_domain_present_stage: PresentStageFlagsEXT,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SwapchainCalibratedTimestampInfoEXT {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub swapchain: SwapchainKHR,
    pub present_stage: PresentStageFlagsEXT,
    pub time_domain_id: u64,
}
bitflags! {
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Default)]
    pub struct PresentStageFlagsEXT: Flags {
    }
}
bitflags! {
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Default)]
    pub struct PastPresentationTimingFlagsEXT: Flags {
    }
}
bitflags! {
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Default)]
    pub struct PresentTimingInfoFlagsEXT: Flags {
    }
}
pub type PFN_vkSetSwapchainPresentTimingQueueSizeEXT =
    unsafe extern "system" fn(device: Device, swapchain: SwapchainKHR, size: u32) -> Result;
pub type PFN_vkGetSwapchainTimingPropertiesEXT = unsafe extern "system" fn(
    device: Device,
    swapchain: SwapchainKHR,
    p_swapchain_timing_properties: *mut SwapchainTimingPropertiesEXT,
    p_swapchain_timing_properties_counter: *mut u64,
) -> Result;
pub type PFN_vkGetSwapchainTimeDomainPropertiesEXT = unsafe extern "system" fn(
    device: Device,
    swapchain: SwapchainKHR,
    p_swapchain_time_domain_properties: *mut SwapchainTimeDomainPropertiesEXT,
    p_time_domains_counter: *mut u64,
) -> Result;
pub type PFN_vkGetPastPresentationTimingEXT = unsafe extern "system" fn(
    device: Device,
    p_past_presentation_timing_info: *const PastPresentationTimingInfoEXT,
    p_past_presentation_timing_properties: *mut PastPresentationTimingPropertiesEXT,
) -> Result;
