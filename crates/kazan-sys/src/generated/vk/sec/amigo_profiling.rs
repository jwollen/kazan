#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{c_char, c_int, c_void};
use core::marker::PhantomData;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceAmigoProfilingFeaturesSEC<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub amigo_profiling: Bool32,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for PhysicalDeviceAmigoProfilingFeaturesSEC<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::PHYSICAL_DEVICE_AMIGO_PROFILING_FEATURES_SEC,
            p_next: core::ptr::null_mut(),
            amigo_profiling: Default::default(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct AmigoProfilingSubmitInfoSEC<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub first_draw_timestamp: u64,
    pub swap_buffer_timestamp: u64,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for AmigoProfilingSubmitInfoSEC<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::AMIGO_PROFILING_SUBMIT_INFO_SEC,
            p_next: core::ptr::null(),
            first_draw_timestamp: Default::default(),
            swap_buffer_timestamp: Default::default(),
            _marker: PhantomData,
        }
    }
}
