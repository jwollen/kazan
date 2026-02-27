#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{CStr, c_char, c_int, c_void};
use core::marker::PhantomData;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SharedPresentSurfaceCapabilitiesKHR<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub shared_present_supported_usage_flags: ImageUsageFlags,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for SharedPresentSurfaceCapabilitiesKHR<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::SHARED_PRESENT_SURFACE_CAPABILITIES_KHR,
            p_next: core::ptr::null_mut(),
            shared_present_supported_usage_flags: Default::default(),
            _marker: PhantomData,
        }
    }
}
impl<'a> SharedPresentSurfaceCapabilitiesKHR<'a> {
    pub fn shared_present_supported_usage_flags(
        mut self,
        shared_present_supported_usage_flags: ImageUsageFlags,
    ) -> Self {
        self.shared_present_supported_usage_flags = shared_present_supported_usage_flags;
        self
    }
}
pub type PFN_vkGetSwapchainStatusKHR =
    unsafe extern "system" fn(device: Device, swapchain: SwapchainKHR) -> Result;
