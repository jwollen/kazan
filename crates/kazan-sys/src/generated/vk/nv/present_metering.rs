#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{c_char, c_int, c_void};
use core::marker::PhantomData;
#[repr(C)]
pub struct SetPresentConfigNV<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub num_frames_per_batch: u32,
    pub present_config_feedback: u32,
    pub _marker: PhantomData<&'a ()>,
}
#[repr(C)]
pub struct PhysicalDevicePresentMeteringFeaturesNV<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub present_metering: Bool32,
    pub _marker: PhantomData<&'a ()>,
}
