#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use std::ffi::{c_char, c_int, c_void};
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDevicePerStageDescriptorSetFeaturesNV {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub per_stage_descriptor_set: Bool32,
    pub dynamic_pipeline_layout: Bool32,
}
