#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{c_char, c_int, c_void};
use core::marker::PhantomData;
#[repr(C)]
pub struct ImageViewASTCDecodeModeEXT<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub decode_mode: Format,
    pub _marker: PhantomData<&'a ()>,
}
#[repr(C)]
pub struct PhysicalDeviceASTCDecodeFeaturesEXT<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub decode_mode_shared_exponent: Bool32,
    pub _marker: PhantomData<&'a ()>,
}
