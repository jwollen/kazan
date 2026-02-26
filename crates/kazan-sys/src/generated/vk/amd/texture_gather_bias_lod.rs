#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{c_char, c_int, c_void};
use core::marker::PhantomData;
#[repr(C)]
pub struct TextureLODGatherFormatPropertiesAMD<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub supports_texture_gather_lod_bias_amd: Bool32,
    pub _marker: PhantomData<&'a ()>,
}
