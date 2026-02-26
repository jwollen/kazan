#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{c_char, c_int, c_void};
use core::marker::PhantomData;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDevicePresentIdFeaturesKHR<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub present_id: Bool32,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for PhysicalDevicePresentIdFeaturesKHR<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::PHYSICAL_DEVICE_PRESENT_ID_FEATURES_KHR,
            p_next: core::ptr::null_mut(),
            present_id: Default::default(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PresentIdKHR<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub swapchain_count: u32,
    pub p_present_ids: *const u64,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for PresentIdKHR<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::PRESENT_ID_KHR,
            p_next: core::ptr::null(),
            swapchain_count: Default::default(),
            p_present_ids: core::ptr::null(),
            _marker: PhantomData,
        }
    }
}
