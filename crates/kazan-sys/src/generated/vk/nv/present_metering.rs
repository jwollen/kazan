#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{c_char, c_int, c_void};
use core::marker::PhantomData;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SetPresentConfigNV<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub num_frames_per_batch: u32,
    pub present_config_feedback: u32,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for SetPresentConfigNV<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::SET_PRESENT_CONFIG_NV,
            p_next: core::ptr::null(),
            num_frames_per_batch: Default::default(),
            present_config_feedback: Default::default(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDevicePresentMeteringFeaturesNV<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub present_metering: Bool32,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for PhysicalDevicePresentMeteringFeaturesNV<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::PHYSICAL_DEVICE_PRESENT_METERING_FEATURES_NV,
            p_next: core::ptr::null_mut(),
            present_metering: Default::default(),
            _marker: PhantomData,
        }
    }
}
