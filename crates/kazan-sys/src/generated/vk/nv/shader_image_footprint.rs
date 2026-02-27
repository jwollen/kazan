#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{CStr, c_char, c_int, c_void};
use core::marker::PhantomData;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceShaderImageFootprintFeaturesNV<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub image_footprint: Bool32,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for PhysicalDeviceShaderImageFootprintFeaturesNV<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::PHYSICAL_DEVICE_SHADER_IMAGE_FOOTPRINT_FEATURES_NV,
            p_next: core::ptr::null_mut(),
            image_footprint: Default::default(),
            _marker: PhantomData,
        }
    }
}
impl<'a> PhysicalDeviceShaderImageFootprintFeaturesNV<'a> {
    pub fn image_footprint(mut self, image_footprint: Bool32) -> Self {
        self.image_footprint = image_footprint;
        self
    }
}
