#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{c_char, c_int, c_void};
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ImageViewSampleWeightCreateInfoQCOM {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub filter_center: Offset2D,
    pub filter_size: Extent2D,
    pub num_phases: u32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceImageProcessingFeaturesQCOM {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub texture_sample_weighted: Bool32,
    pub texture_box_filter: Bool32,
    pub texture_block_match: Bool32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceImageProcessingPropertiesQCOM {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub max_weight_filter_phases: u32,
    pub max_weight_filter_dimension: Extent2D,
    pub max_block_match_region: Extent2D,
    pub max_box_filter_block_size: Extent2D,
}
