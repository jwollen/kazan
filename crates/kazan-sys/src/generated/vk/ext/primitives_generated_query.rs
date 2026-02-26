#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{c_char, c_int, c_void};
use core::marker::PhantomData;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDevicePrimitivesGeneratedQueryFeaturesEXT<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub primitives_generated_query: Bool32,
    pub primitives_generated_query_with_rasterizer_discard: Bool32,
    pub primitives_generated_query_with_non_zero_streams: Bool32,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for PhysicalDevicePrimitivesGeneratedQueryFeaturesEXT<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::PHYSICAL_DEVICE_PRIMITIVES_GENERATED_QUERY_FEATURES_EXT,
            p_next: core::ptr::null_mut(),
            primitives_generated_query: Default::default(),
            primitives_generated_query_with_rasterizer_discard: Default::default(),
            primitives_generated_query_with_non_zero_streams: Default::default(),
            _marker: PhantomData,
        }
    }
}
