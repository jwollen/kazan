#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{CStr, c_char, c_int, c_void};
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
impl<'a> PhysicalDeviceAmigoProfilingFeaturesSEC<'a> {
    pub fn amigo_profiling(mut self, amigo_profiling: Bool32) -> Self {
        self.amigo_profiling = amigo_profiling;
        self
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
impl<'a> AmigoProfilingSubmitInfoSEC<'a> {
    pub fn first_draw_timestamp(mut self, first_draw_timestamp: u64) -> Self {
        self.first_draw_timestamp = first_draw_timestamp;
        self
    }
    pub fn swap_buffer_timestamp(mut self, swap_buffer_timestamp: u64) -> Self {
        self.swap_buffer_timestamp = swap_buffer_timestamp;
        self
    }
}
