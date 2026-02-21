#![allow(unused_imports)]
use crate::*;
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
use kazan_sys::{vk::*, *};
pub struct DeviceFn {
    set_swapchain_present_timing_queue_size_ext: PFN_vkSetSwapchainPresentTimingQueueSizeEXT,
    get_swapchain_timing_properties_ext: PFN_vkGetSwapchainTimingPropertiesEXT,
    get_swapchain_time_domain_properties_ext: PFN_vkGetSwapchainTimeDomainPropertiesEXT,
    get_past_presentation_timing_ext: PFN_vkGetPastPresentationTimingEXT,
}
impl DeviceFn {
    pub unsafe fn load(
        load: impl Fn(&CStr) -> Option<PFN_vkVoidFunction>,
    ) -> core::result::Result<Self, LoadingError> {
        unsafe {
            Ok(Self {
                set_swapchain_present_timing_queue_size_ext: transmute(
                    load(c"vkSetSwapchainPresentTimingQueueSizeEXT").ok_or(LoadingError)?,
                ),
                get_swapchain_timing_properties_ext: transmute(
                    load(c"vkGetSwapchainTimingPropertiesEXT").ok_or(LoadingError)?,
                ),
                get_swapchain_time_domain_properties_ext: transmute(
                    load(c"vkGetSwapchainTimeDomainPropertiesEXT").ok_or(LoadingError)?,
                ),
                get_past_presentation_timing_ext: transmute(
                    load(c"vkGetPastPresentationTimingEXT").ok_or(LoadingError)?,
                ),
            })
        }
    }
}
impl DeviceFn {
    pub unsafe fn set_swapchain_present_timing_queue_size_ext(
        &self,
        device: Device,
        swapchain: SwapchainKHR,
        size: u32,
    ) -> Result {
        unsafe { (self.set_swapchain_present_timing_queue_size_ext)(device, swapchain, size) }
    }
    pub unsafe fn get_swapchain_timing_properties_ext(
        &self,
        device: Device,
        swapchain: SwapchainKHR,
        swapchain_timing_properties: &mut SwapchainTimingPropertiesEXT,
        swapchain_timing_properties_counter: Option<&mut u64>,
    ) -> Result {
        unsafe {
            (self.get_swapchain_timing_properties_ext)(
                device,
                swapchain,
                swapchain_timing_properties,
                swapchain_timing_properties_counter.to_raw_mut_ptr(),
            )
        }
    }
    pub unsafe fn get_swapchain_time_domain_properties_ext(
        &self,
        device: Device,
        swapchain: SwapchainKHR,
        swapchain_time_domain_properties: &mut SwapchainTimeDomainPropertiesEXT,
        time_domains_counter: Option<&mut u64>,
    ) -> Result {
        unsafe {
            (self.get_swapchain_time_domain_properties_ext)(
                device,
                swapchain,
                swapchain_time_domain_properties,
                time_domains_counter.to_raw_mut_ptr(),
            )
        }
    }
    pub unsafe fn get_past_presentation_timing_ext(
        &self,
        device: Device,
        past_presentation_timing_info: &PastPresentationTimingInfoEXT,
        past_presentation_timing_properties: &mut PastPresentationTimingPropertiesEXT,
    ) -> Result {
        unsafe {
            (self.get_past_presentation_timing_ext)(
                device,
                past_presentation_timing_info,
                past_presentation_timing_properties,
            )
        }
    }
}
