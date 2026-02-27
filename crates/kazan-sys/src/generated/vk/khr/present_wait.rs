#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{CStr, c_char, c_int, c_void};
use core::marker::PhantomData;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDevicePresentWaitFeaturesKHR<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub present_wait: Bool32,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for PhysicalDevicePresentWaitFeaturesKHR<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::PHYSICAL_DEVICE_PRESENT_WAIT_FEATURES_KHR,
            p_next: core::ptr::null_mut(),
            present_wait: Default::default(),
            _marker: PhantomData,
        }
    }
}
impl<'a> PhysicalDevicePresentWaitFeaturesKHR<'a> {
    pub fn present_wait(mut self, present_wait: Bool32) -> Self {
        self.present_wait = present_wait;
        self
    }
}
pub type PFN_vkWaitForPresentKHR = unsafe extern "system" fn(
    device: Device,
    swapchain: SwapchainKHR,
    present_id: u64,
    timeout: u64,
) -> Result;
