#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{c_char, c_int, c_void};
use core::marker::PhantomData;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct TextureLODGatherFormatPropertiesAMD<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub supports_texture_gather_lod_bias_amd: Bool32,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for TextureLODGatherFormatPropertiesAMD<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::TEXTURE_LOD_GATHER_FORMAT_PROPERTIES_AMD,
            p_next: core::ptr::null_mut(),
            supports_texture_gather_lod_bias_amd: Default::default(),
            _marker: PhantomData,
        }
    }
}
