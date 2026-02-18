#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use std::ffi::{c_char, c_int, c_void};
pub type PipelineInfoEXT = PipelineInfoKHR;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PipelinePropertiesIdentifierEXT {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub pipeline_identifier: u8,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDevicePipelinePropertiesFeaturesEXT {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub pipeline_properties_identifier: Bool32,
}
pub type PFN_vkGetPipelinePropertiesEXT = unsafe extern "system" fn(
    device: Device,
    p_pipeline_info: *const PipelineInfoEXT,
    p_pipeline_properties: *mut BaseOutStructure,
) -> Result;
