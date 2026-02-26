#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{c_char, c_int, c_void};
use core::marker::PhantomData;
#[repr(C)]
pub struct PhysicalDevicePresentBarrierFeaturesNV<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub present_barrier: Bool32,
    pub _marker: PhantomData<&'a ()>,
}
#[repr(C)]
pub struct SurfaceCapabilitiesPresentBarrierNV<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub present_barrier_supported: Bool32,
    pub _marker: PhantomData<&'a ()>,
}
#[repr(C)]
pub struct SwapchainPresentBarrierCreateInfoNV<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub present_barrier_enable: Bool32,
    pub _marker: PhantomData<&'a ()>,
}
