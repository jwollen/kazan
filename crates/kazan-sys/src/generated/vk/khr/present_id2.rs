#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{c_char, c_int, c_void};
use core::marker::PhantomData;
#[repr(C)]
pub struct PhysicalDevicePresentId2FeaturesKHR<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub present_id2: Bool32,
    pub _marker: PhantomData<&'a ()>,
}
#[repr(C)]
pub struct PresentId2KHR<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub swapchain_count: u32,
    pub p_present_ids: *const u64,
    pub _marker: PhantomData<&'a ()>,
}
#[repr(C)]
pub struct SurfaceCapabilitiesPresentId2KHR<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub present_id2_supported: Bool32,
    pub _marker: PhantomData<&'a ()>,
}
