#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{c_char, c_int, c_void};
use core::marker::PhantomData;
pub type PipelineInfoEXT<'a> = PipelineInfoKHR<'a>;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PipelinePropertiesIdentifierEXT<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub pipeline_identifier: [u8; UUID_SIZE as usize],
    pub _marker: PhantomData<&'a ()>,
}
impl Default for PipelinePropertiesIdentifierEXT<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::PIPELINE_PROPERTIES_IDENTIFIER_EXT,
            p_next: core::ptr::null_mut(),
            pipeline_identifier: [Default::default(); _],
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDevicePipelinePropertiesFeaturesEXT<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub pipeline_properties_identifier: Bool32,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for PhysicalDevicePipelinePropertiesFeaturesEXT<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::PHYSICAL_DEVICE_PIPELINE_PROPERTIES_FEATURES_EXT,
            p_next: core::ptr::null_mut(),
            pipeline_properties_identifier: Default::default(),
            _marker: PhantomData,
        }
    }
}
pub type PFN_vkGetPipelinePropertiesEXT = unsafe extern "system" fn(
    device: Device,
    p_pipeline_info: *const PipelineInfoEXT<'_>,
    p_pipeline_properties: *mut BaseOutStructure,
) -> Result;
