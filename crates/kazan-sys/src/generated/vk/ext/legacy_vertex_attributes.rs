#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{c_char, c_int, c_void};
#[repr(C)]
pub struct PhysicalDeviceLegacyVertexAttributesFeaturesEXT {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub legacy_vertex_attributes: Bool32,
}
#[repr(C)]
pub struct PhysicalDeviceLegacyVertexAttributesPropertiesEXT {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub native_unaligned_performance: Bool32,
}
