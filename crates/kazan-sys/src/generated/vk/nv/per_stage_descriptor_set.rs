#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{CStr, c_char, c_int, c_void};
use core::marker::PhantomData;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDevicePerStageDescriptorSetFeaturesNV<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub per_stage_descriptor_set: Bool32,
    pub dynamic_pipeline_layout: Bool32,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for PhysicalDevicePerStageDescriptorSetFeaturesNV<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::PHYSICAL_DEVICE_PER_STAGE_DESCRIPTOR_SET_FEATURES_NV,
            p_next: core::ptr::null_mut(),
            per_stage_descriptor_set: Default::default(),
            dynamic_pipeline_layout: Default::default(),
            _marker: PhantomData,
        }
    }
}
impl<'a> PhysicalDevicePerStageDescriptorSetFeaturesNV<'a> {
    pub fn per_stage_descriptor_set(mut self, per_stage_descriptor_set: Bool32) -> Self {
        self.per_stage_descriptor_set = per_stage_descriptor_set;
        self
    }
    pub fn dynamic_pipeline_layout(mut self, dynamic_pipeline_layout: Bool32) -> Self {
        self.dynamic_pipeline_layout = dynamic_pipeline_layout;
        self
    }
}
