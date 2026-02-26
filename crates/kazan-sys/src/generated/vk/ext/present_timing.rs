#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{c_char, c_int, c_void};
use core::marker::PhantomData;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDevicePresentTimingFeaturesEXT<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub present_timing: Bool32,
    pub present_at_absolute_time: Bool32,
    pub present_at_relative_time: Bool32,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for PhysicalDevicePresentTimingFeaturesEXT<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::PHYSICAL_DEVICE_PRESENT_TIMING_FEATURES_EXT,
            p_next: core::ptr::null_mut(),
            present_timing: Default::default(),
            present_at_absolute_time: Default::default(),
            present_at_relative_time: Default::default(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PresentTimingSurfaceCapabilitiesEXT<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub present_timing_supported: Bool32,
    pub present_at_absolute_time_supported: Bool32,
    pub present_at_relative_time_supported: Bool32,
    pub present_stage_queries: PresentStageFlagsEXT,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for PresentTimingSurfaceCapabilitiesEXT<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::PRESENT_TIMING_SURFACE_CAPABILITIES_EXT,
            p_next: core::ptr::null_mut(),
            present_timing_supported: Default::default(),
            present_at_absolute_time_supported: Default::default(),
            present_at_relative_time_supported: Default::default(),
            present_stage_queries: Default::default(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SwapchainTimingPropertiesEXT<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub refresh_duration: u64,
    pub refresh_interval: u64,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for SwapchainTimingPropertiesEXT<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::SWAPCHAIN_TIMING_PROPERTIES_EXT,
            p_next: core::ptr::null_mut(),
            refresh_duration: Default::default(),
            refresh_interval: Default::default(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SwapchainTimeDomainPropertiesEXT<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub time_domain_count: u32,
    pub p_time_domains: *mut TimeDomainKHR,
    pub p_time_domain_ids: *mut u64,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for SwapchainTimeDomainPropertiesEXT<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::SWAPCHAIN_TIME_DOMAIN_PROPERTIES_EXT,
            p_next: core::ptr::null_mut(),
            time_domain_count: Default::default(),
            p_time_domains: core::ptr::null_mut(),
            p_time_domain_ids: core::ptr::null_mut(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct PresentStageTimeEXT {
    pub stage: PresentStageFlagsEXT,
    pub time: u64,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PastPresentationTimingInfoEXT<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: PastPresentationTimingFlagsEXT,
    pub swapchain: SwapchainKHR,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for PastPresentationTimingInfoEXT<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::PAST_PRESENTATION_TIMING_INFO_EXT,
            p_next: core::ptr::null(),
            flags: Default::default(),
            swapchain: Default::default(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PastPresentationTimingPropertiesEXT<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub timing_properties_counter: u64,
    pub time_domains_counter: u64,
    pub presentation_timing_count: u32,
    pub p_presentation_timings: *mut PastPresentationTimingEXT<'a>,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for PastPresentationTimingPropertiesEXT<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::PAST_PRESENTATION_TIMING_PROPERTIES_EXT,
            p_next: core::ptr::null_mut(),
            timing_properties_counter: Default::default(),
            time_domains_counter: Default::default(),
            presentation_timing_count: Default::default(),
            p_presentation_timings: core::ptr::null_mut(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PastPresentationTimingEXT<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub present_id: u64,
    pub target_time: u64,
    pub present_stage_count: u32,
    pub p_present_stages: *mut PresentStageTimeEXT,
    pub time_domain: TimeDomainKHR,
    pub time_domain_id: u64,
    pub report_complete: Bool32,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for PastPresentationTimingEXT<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::PAST_PRESENTATION_TIMING_EXT,
            p_next: core::ptr::null_mut(),
            present_id: Default::default(),
            target_time: Default::default(),
            present_stage_count: Default::default(),
            p_present_stages: core::ptr::null_mut(),
            time_domain: Default::default(),
            time_domain_id: Default::default(),
            report_complete: Default::default(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PresentTimingsInfoEXT<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub swapchain_count: u32,
    pub p_timing_infos: *const PresentTimingInfoEXT<'a>,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for PresentTimingsInfoEXT<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::PRESENT_TIMINGS_INFO_EXT,
            p_next: core::ptr::null(),
            swapchain_count: Default::default(),
            p_timing_infos: core::ptr::null(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PresentTimingInfoEXT<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: PresentTimingInfoFlagsEXT,
    pub target_time: u64,
    pub time_domain_id: u64,
    pub present_stage_queries: PresentStageFlagsEXT,
    pub target_time_domain_present_stage: PresentStageFlagsEXT,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for PresentTimingInfoEXT<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::PRESENT_TIMING_INFO_EXT,
            p_next: core::ptr::null(),
            flags: Default::default(),
            target_time: Default::default(),
            time_domain_id: Default::default(),
            present_stage_queries: Default::default(),
            target_time_domain_present_stage: Default::default(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SwapchainCalibratedTimestampInfoEXT<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub swapchain: SwapchainKHR,
    pub present_stage: PresentStageFlagsEXT,
    pub time_domain_id: u64,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for SwapchainCalibratedTimestampInfoEXT<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::SWAPCHAIN_CALIBRATED_TIMESTAMP_INFO_EXT,
            p_next: core::ptr::null(),
            swapchain: Default::default(),
            present_stage: Default::default(),
            time_domain_id: Default::default(),
            _marker: PhantomData,
        }
    }
}
bitflags! {
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct PresentStageFlagsEXT: Flags {
        const QUEUE_OPERATIONS_END_EXT = PresentStageFlagBitsEXT::QUEUE_OPERATIONS_END_EXT.0;
        const REQUEST_DEQUEUED_EXT = PresentStageFlagBitsEXT::REQUEST_DEQUEUED_EXT.0;
        const IMAGE_FIRST_PIXEL_OUT_EXT = PresentStageFlagBitsEXT::IMAGE_FIRST_PIXEL_OUT_EXT.0;
        const IMAGE_FIRST_PIXEL_VISIBLE_EXT = PresentStageFlagBitsEXT::IMAGE_FIRST_PIXEL_VISIBLE_EXT.0;
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PresentStageFlagBitsEXT(u32);
impl PresentStageFlagBitsEXT {
    pub const QUEUE_OPERATIONS_END_EXT: Self = Self(1 << 0);
    pub const REQUEST_DEQUEUED_EXT: Self = Self(1 << 1);
    pub const IMAGE_FIRST_PIXEL_OUT_EXT: Self = Self(1 << 2);
    pub const IMAGE_FIRST_PIXEL_VISIBLE_EXT: Self = Self(1 << 3);
}
bitflags! {
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct PastPresentationTimingFlagsEXT: Flags {
        const ALLOW_PARTIAL_RESULTS_EXT = PastPresentationTimingFlagBitsEXT::ALLOW_PARTIAL_RESULTS_EXT.0;
        const ALLOW_OUT_OF_ORDER_RESULTS_EXT = PastPresentationTimingFlagBitsEXT::ALLOW_OUT_OF_ORDER_RESULTS_EXT.0;
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PastPresentationTimingFlagBitsEXT(u32);
impl PastPresentationTimingFlagBitsEXT {
    pub const ALLOW_PARTIAL_RESULTS_EXT: Self = Self(1 << 0);
    pub const ALLOW_OUT_OF_ORDER_RESULTS_EXT: Self = Self(1 << 1);
}
bitflags! {
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct PresentTimingInfoFlagsEXT: Flags {
        const PRESENT_AT_RELATIVE_TIME_EXT = PresentTimingInfoFlagBitsEXT::PRESENT_AT_RELATIVE_TIME_EXT.0;
        const PRESENT_AT_NEAREST_REFRESH_CYCLE_EXT = PresentTimingInfoFlagBitsEXT::PRESENT_AT_NEAREST_REFRESH_CYCLE_EXT.0;
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PresentTimingInfoFlagBitsEXT(u32);
impl PresentTimingInfoFlagBitsEXT {
    pub const PRESENT_AT_RELATIVE_TIME_EXT: Self = Self(1 << 0);
    pub const PRESENT_AT_NEAREST_REFRESH_CYCLE_EXT: Self = Self(1 << 1);
}
pub type PFN_vkSetSwapchainPresentTimingQueueSizeEXT =
    unsafe extern "system" fn(device: Device, swapchain: SwapchainKHR, size: u32) -> Result;
pub type PFN_vkGetSwapchainTimingPropertiesEXT = unsafe extern "system" fn(
    device: Device,
    swapchain: SwapchainKHR,
    p_swapchain_timing_properties: *mut SwapchainTimingPropertiesEXT<'_>,
    p_swapchain_timing_properties_counter: *mut u64,
) -> Result;
pub type PFN_vkGetSwapchainTimeDomainPropertiesEXT = unsafe extern "system" fn(
    device: Device,
    swapchain: SwapchainKHR,
    p_swapchain_time_domain_properties: *mut SwapchainTimeDomainPropertiesEXT<'_>,
    p_time_domains_counter: *mut u64,
) -> Result;
pub type PFN_vkGetPastPresentationTimingEXT = unsafe extern "system" fn(
    device: Device,
    p_past_presentation_timing_info: *const PastPresentationTimingInfoEXT<'_>,
    p_past_presentation_timing_properties: *mut PastPresentationTimingPropertiesEXT<'_>,
) -> Result;
