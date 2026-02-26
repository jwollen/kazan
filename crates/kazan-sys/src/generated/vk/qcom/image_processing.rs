#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{c_char, c_int, c_void};
use core::marker::PhantomData;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ImageViewSampleWeightCreateInfoQCOM<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub filter_center: Offset2D,
    pub filter_size: Extent2D,
    pub num_phases: u32,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for ImageViewSampleWeightCreateInfoQCOM<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::IMAGE_VIEW_SAMPLE_WEIGHT_CREATE_INFO_QCOM,
            p_next: core::ptr::null(),
            filter_center: Default::default(),
            filter_size: Default::default(),
            num_phases: Default::default(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceImageProcessingFeaturesQCOM<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub texture_sample_weighted: Bool32,
    pub texture_box_filter: Bool32,
    pub texture_block_match: Bool32,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for PhysicalDeviceImageProcessingFeaturesQCOM<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::PHYSICAL_DEVICE_IMAGE_PROCESSING_FEATURES_QCOM,
            p_next: core::ptr::null_mut(),
            texture_sample_weighted: Default::default(),
            texture_box_filter: Default::default(),
            texture_block_match: Default::default(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceImageProcessingPropertiesQCOM<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub max_weight_filter_phases: u32,
    pub max_weight_filter_dimension: Extent2D,
    pub max_block_match_region: Extent2D,
    pub max_box_filter_block_size: Extent2D,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for PhysicalDeviceImageProcessingPropertiesQCOM<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::PHYSICAL_DEVICE_IMAGE_PROCESSING_PROPERTIES_QCOM,
            p_next: core::ptr::null_mut(),
            max_weight_filter_phases: Default::default(),
            max_weight_filter_dimension: Default::default(),
            max_block_match_region: Default::default(),
            max_box_filter_block_size: Default::default(),
            _marker: PhantomData,
        }
    }
}
