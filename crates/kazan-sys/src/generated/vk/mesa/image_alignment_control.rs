#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{c_char, c_int, c_void};
use core::marker::PhantomData;
#[repr(C)]
pub struct PhysicalDeviceImageAlignmentControlFeaturesMESA<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub image_alignment_control: Bool32,
    pub _marker: PhantomData<&'a ()>,
}
#[repr(C)]
pub struct PhysicalDeviceImageAlignmentControlPropertiesMESA<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub supported_image_alignment_mask: u32,
    pub _marker: PhantomData<&'a ()>,
}
#[repr(C)]
pub struct ImageAlignmentControlCreateInfoMESA<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub maximum_requested_alignment: u32,
    pub _marker: PhantomData<&'a ()>,
}
