#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{c_char, c_int, c_void};
use core::marker::PhantomData;
pub type VertexInputBindingDivisorDescriptionEXT = VertexInputBindingDivisorDescription;
pub type PipelineVertexInputDivisorStateCreateInfoEXT<'a> =
    PipelineVertexInputDivisorStateCreateInfo<'a>;
pub type PhysicalDeviceVertexAttributeDivisorFeaturesEXT<'a> =
    PhysicalDeviceVertexAttributeDivisorFeatures<'a>;
#[repr(C)]
pub struct PhysicalDeviceVertexAttributeDivisorPropertiesEXT<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub max_vertex_attrib_divisor: u32,
    pub _marker: PhantomData<&'a ()>,
}
