#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{CStr, c_char, c_int, c_void};
use core::marker::PhantomData;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDevicePipelineOpacityMicromapFeaturesARM<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub pipeline_opacity_micromap: Bool32,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for PhysicalDevicePipelineOpacityMicromapFeaturesARM<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::PHYSICAL_DEVICE_PIPELINE_OPACITY_MICROMAP_FEATURES_ARM,
            p_next: core::ptr::null_mut(),
            pipeline_opacity_micromap: Default::default(),
            _marker: PhantomData,
        }
    }
}
impl<'a> PhysicalDevicePipelineOpacityMicromapFeaturesARM<'a> {
    pub fn pipeline_opacity_micromap(mut self, pipeline_opacity_micromap: Bool32) -> Self {
        self.pipeline_opacity_micromap = pipeline_opacity_micromap;
        self
    }
}
