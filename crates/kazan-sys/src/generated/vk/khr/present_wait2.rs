#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{c_char, c_int, c_void};
use core::marker::PhantomData;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PresentWait2InfoKHR<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub present_id: u64,
    pub timeout: u64,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for PresentWait2InfoKHR<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::PRESENT_WAIT_2_INFO_KHR,
            p_next: core::ptr::null(),
            present_id: Default::default(),
            timeout: Default::default(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDevicePresentWait2FeaturesKHR<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub present_wait2: Bool32,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for PhysicalDevicePresentWait2FeaturesKHR<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::PHYSICAL_DEVICE_PRESENT_WAIT_2_FEATURES_KHR,
            p_next: core::ptr::null_mut(),
            present_wait2: Default::default(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SurfaceCapabilitiesPresentWait2KHR<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub present_wait2_supported: Bool32,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for SurfaceCapabilitiesPresentWait2KHR<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::SURFACE_CAPABILITIES_PRESENT_WAIT_2_KHR,
            p_next: core::ptr::null_mut(),
            present_wait2_supported: Default::default(),
            _marker: PhantomData,
        }
    }
}
pub type PFN_vkWaitForPresent2KHR = unsafe extern "system" fn(
    device: Device,
    swapchain: SwapchainKHR,
    p_present_wait2_info: *const PresentWait2InfoKHR<'_>,
) -> Result;
