#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{c_char, c_int, c_void};
use core::marker::PhantomData;
#[repr(C)]
pub struct PhysicalDeviceShaderIntegerFunctions2FeaturesINTEL<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub shader_integer_functions2: Bool32,
    pub _marker: PhantomData<&'a ()>,
}
