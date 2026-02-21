#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{c_char, c_int, c_void};
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ViewportSwizzleNV {
    pub x: ViewportCoordinateSwizzleNV,
    pub y: ViewportCoordinateSwizzleNV,
    pub z: ViewportCoordinateSwizzleNV,
    pub w: ViewportCoordinateSwizzleNV,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PipelineViewportSwizzleStateCreateInfoNV {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: PipelineViewportSwizzleStateCreateFlagsNV,
    pub viewport_count: u32,
    pub p_viewport_swizzles: *const ViewportSwizzleNV,
}
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ViewportCoordinateSwizzleNV(i32);
impl ViewportCoordinateSwizzleNV {
    pub const POSITIVE_X_NV: Self = Self(0);
    pub const NEGATIVE_X_NV: Self = Self(1);
    pub const POSITIVE_Y_NV: Self = Self(2);
    pub const NEGATIVE_Y_NV: Self = Self(3);
    pub const POSITIVE_Z_NV: Self = Self(4);
    pub const NEGATIVE_Z_NV: Self = Self(5);
    pub const POSITIVE_W_NV: Self = Self(6);
    pub const NEGATIVE_W_NV: Self = Self(7);
}
bitflags! {
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Default)]
    pub struct PipelineViewportSwizzleStateCreateFlagsNV: Flags {
    }
}
