#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use std::ffi::{c_char, c_int, c_void};
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDevicePrimitivesGeneratedQueryFeaturesEXT {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub primitives_generated_query: Bool32,
    pub primitives_generated_query_with_rasterizer_discard: Bool32,
    pub primitives_generated_query_with_non_zero_streams: Bool32,
}
