#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{c_char, c_int, c_void};
#[repr(C)]
pub struct ValidationFlagsEXT {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub disabled_validation_check_count: u32,
    pub p_disabled_validation_checks: *const ValidationCheckEXT,
}
#[repr(transparent)]
#[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ValidationCheckEXT(i32);
impl ValidationCheckEXT {
    pub const ALL_EXT: Self = Self(0);
    pub const SHADERS_EXT: Self = Self(1);
}
