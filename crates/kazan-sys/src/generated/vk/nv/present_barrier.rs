#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{CStr, c_char, c_int, c_void};
use core::marker::PhantomData;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDevicePresentBarrierFeaturesNV<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub present_barrier: Bool32,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for PhysicalDevicePresentBarrierFeaturesNV<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::PHYSICAL_DEVICE_PRESENT_BARRIER_FEATURES_NV,
            p_next: core::ptr::null_mut(),
            present_barrier: Default::default(),
            _marker: PhantomData,
        }
    }
}
impl<'a> PhysicalDevicePresentBarrierFeaturesNV<'a> {
    pub fn present_barrier(mut self, present_barrier: Bool32) -> Self {
        self.present_barrier = present_barrier;
        self
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SurfaceCapabilitiesPresentBarrierNV<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub present_barrier_supported: Bool32,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for SurfaceCapabilitiesPresentBarrierNV<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::SURFACE_CAPABILITIES_PRESENT_BARRIER_NV,
            p_next: core::ptr::null_mut(),
            present_barrier_supported: Default::default(),
            _marker: PhantomData,
        }
    }
}
impl<'a> SurfaceCapabilitiesPresentBarrierNV<'a> {
    pub fn present_barrier_supported(mut self, present_barrier_supported: Bool32) -> Self {
        self.present_barrier_supported = present_barrier_supported;
        self
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SwapchainPresentBarrierCreateInfoNV<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub present_barrier_enable: Bool32,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for SwapchainPresentBarrierCreateInfoNV<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::SWAPCHAIN_PRESENT_BARRIER_CREATE_INFO_NV,
            p_next: core::ptr::null_mut(),
            present_barrier_enable: Default::default(),
            _marker: PhantomData,
        }
    }
}
impl<'a> SwapchainPresentBarrierCreateInfoNV<'a> {
    pub fn present_barrier_enable(mut self, present_barrier_enable: Bool32) -> Self {
        self.present_barrier_enable = present_barrier_enable;
        self
    }
}
