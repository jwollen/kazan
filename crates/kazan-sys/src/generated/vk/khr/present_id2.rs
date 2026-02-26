#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{c_char, c_int, c_void};
use core::marker::PhantomData;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDevicePresentId2FeaturesKHR<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub present_id2: Bool32,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for PhysicalDevicePresentId2FeaturesKHR<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::PHYSICAL_DEVICE_PRESENT_ID_2_FEATURES_KHR,
            p_next: core::ptr::null_mut(),
            present_id2: Default::default(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PresentId2KHR<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub swapchain_count: u32,
    pub p_present_ids: *const u64,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for PresentId2KHR<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::PRESENT_ID_2_KHR,
            p_next: core::ptr::null(),
            swapchain_count: Default::default(),
            p_present_ids: core::ptr::null(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SurfaceCapabilitiesPresentId2KHR<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub present_id2_supported: Bool32,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for SurfaceCapabilitiesPresentId2KHR<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::SURFACE_CAPABILITIES_PRESENT_ID_2_KHR,
            p_next: core::ptr::null_mut(),
            present_id2_supported: Default::default(),
            _marker: PhantomData,
        }
    }
}
