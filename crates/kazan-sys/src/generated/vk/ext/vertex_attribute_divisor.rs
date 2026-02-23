#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{c_char, c_int, c_void};
pub type VertexInputBindingDivisorDescriptionEXT = VertexInputBindingDivisorDescription;
pub type PipelineVertexInputDivisorStateCreateInfoEXT = PipelineVertexInputDivisorStateCreateInfo;
pub type PhysicalDeviceVertexAttributeDivisorFeaturesEXT =
    PhysicalDeviceVertexAttributeDivisorFeatures;
#[repr(C)]
pub struct PhysicalDeviceVertexAttributeDivisorPropertiesEXT {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub max_vertex_attrib_divisor: u32,
}
