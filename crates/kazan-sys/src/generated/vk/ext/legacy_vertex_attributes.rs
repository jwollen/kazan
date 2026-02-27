#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{CStr, c_char, c_int, c_void};
use core::marker::PhantomData;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceLegacyVertexAttributesFeaturesEXT<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub legacy_vertex_attributes: Bool32,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for PhysicalDeviceLegacyVertexAttributesFeaturesEXT<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::PHYSICAL_DEVICE_LEGACY_VERTEX_ATTRIBUTES_FEATURES_EXT,
            p_next: core::ptr::null_mut(),
            legacy_vertex_attributes: Default::default(),
            _marker: PhantomData,
        }
    }
}
impl<'a> PhysicalDeviceLegacyVertexAttributesFeaturesEXT<'a> {
    pub fn legacy_vertex_attributes(mut self, legacy_vertex_attributes: Bool32) -> Self {
        self.legacy_vertex_attributes = legacy_vertex_attributes;
        self
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceLegacyVertexAttributesPropertiesEXT<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub native_unaligned_performance: Bool32,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for PhysicalDeviceLegacyVertexAttributesPropertiesEXT<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::PHYSICAL_DEVICE_LEGACY_VERTEX_ATTRIBUTES_PROPERTIES_EXT,
            p_next: core::ptr::null_mut(),
            native_unaligned_performance: Default::default(),
            _marker: PhantomData,
        }
    }
}
impl<'a> PhysicalDeviceLegacyVertexAttributesPropertiesEXT<'a> {
    pub fn native_unaligned_performance(mut self, native_unaligned_performance: Bool32) -> Self {
        self.native_unaligned_performance = native_unaligned_performance;
        self
    }
}
