#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{CStr, c_char, c_int, c_void};
use core::marker::PhantomData;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceImageAlignmentControlFeaturesMESA<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub image_alignment_control: Bool32,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for PhysicalDeviceImageAlignmentControlFeaturesMESA<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::PHYSICAL_DEVICE_IMAGE_ALIGNMENT_CONTROL_FEATURES_MESA,
            p_next: core::ptr::null_mut(),
            image_alignment_control: Default::default(),
            _marker: PhantomData,
        }
    }
}
impl<'a> PhysicalDeviceImageAlignmentControlFeaturesMESA<'a> {
    pub fn image_alignment_control(mut self, image_alignment_control: Bool32) -> Self {
        self.image_alignment_control = image_alignment_control;
        self
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceImageAlignmentControlPropertiesMESA<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub supported_image_alignment_mask: u32,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for PhysicalDeviceImageAlignmentControlPropertiesMESA<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::PHYSICAL_DEVICE_IMAGE_ALIGNMENT_CONTROL_PROPERTIES_MESA,
            p_next: core::ptr::null_mut(),
            supported_image_alignment_mask: Default::default(),
            _marker: PhantomData,
        }
    }
}
impl<'a> PhysicalDeviceImageAlignmentControlPropertiesMESA<'a> {
    pub fn supported_image_alignment_mask(mut self, supported_image_alignment_mask: u32) -> Self {
        self.supported_image_alignment_mask = supported_image_alignment_mask;
        self
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ImageAlignmentControlCreateInfoMESA<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub maximum_requested_alignment: u32,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for ImageAlignmentControlCreateInfoMESA<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::IMAGE_ALIGNMENT_CONTROL_CREATE_INFO_MESA,
            p_next: core::ptr::null(),
            maximum_requested_alignment: Default::default(),
            _marker: PhantomData,
        }
    }
}
impl<'a> ImageAlignmentControlCreateInfoMESA<'a> {
    pub fn maximum_requested_alignment(mut self, maximum_requested_alignment: u32) -> Self {
        self.maximum_requested_alignment = maximum_requested_alignment;
        self
    }
}
