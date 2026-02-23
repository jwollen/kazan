#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{c_char, c_int, c_void};
#[repr(C)]
pub struct PipelineRasterizationStateRasterizationOrderAMD {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub rasterization_order: RasterizationOrderAMD,
}
#[repr(transparent)]
#[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct RasterizationOrderAMD(i32);
impl RasterizationOrderAMD {
    pub const STRICT_AMD: Self = Self(0);
    pub const RELAXED_AMD: Self = Self(1);
}
