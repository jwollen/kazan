#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{CStr, c_char, c_int, c_void};
use core::marker::PhantomData;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceShader64BitIndexingFeaturesEXT<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub shader64_bit_indexing: Bool32,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for PhysicalDeviceShader64BitIndexingFeaturesEXT<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::PHYSICAL_DEVICE_SHADER_64_BIT_INDEXING_FEATURES_EXT,
            p_next: core::ptr::null_mut(),
            shader64_bit_indexing: Default::default(),
            _marker: PhantomData,
        }
    }
}
impl<'a> PhysicalDeviceShader64BitIndexingFeaturesEXT<'a> {
    pub fn shader64_bit_indexing(mut self, shader64_bit_indexing: Bool32) -> Self {
        self.shader64_bit_indexing = shader64_bit_indexing;
        self
    }
}
