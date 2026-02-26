#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{c_char, c_int, c_void};
use core::marker::PhantomData;
#[repr(C)]
pub struct PresentWait2InfoKHR<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub present_id: u64,
    pub timeout: u64,
    pub _marker: PhantomData<&'a ()>,
}
#[repr(C)]
pub struct PhysicalDevicePresentWait2FeaturesKHR<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub present_wait2: Bool32,
    pub _marker: PhantomData<&'a ()>,
}
#[repr(C)]
pub struct SurfaceCapabilitiesPresentWait2KHR<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub present_wait2_supported: Bool32,
    pub _marker: PhantomData<&'a ()>,
}
pub type PFN_vkWaitForPresent2KHR = unsafe extern "system" fn(
    device: Device,
    swapchain: SwapchainKHR,
    p_present_wait2_info: *const PresentWait2InfoKHR<'_>,
) -> Result;
