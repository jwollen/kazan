#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{c_char, c_int, c_void};
use core::marker::PhantomData;
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct ViewportSwizzleNV {
    pub x: ViewportCoordinateSwizzleNV,
    pub y: ViewportCoordinateSwizzleNV,
    pub z: ViewportCoordinateSwizzleNV,
    pub w: ViewportCoordinateSwizzleNV,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PipelineViewportSwizzleStateCreateInfoNV<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: PipelineViewportSwizzleStateCreateFlagsNV,
    pub viewport_count: u32,
    pub p_viewport_swizzles: *const ViewportSwizzleNV,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for PipelineViewportSwizzleStateCreateInfoNV<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::PIPELINE_VIEWPORT_SWIZZLE_STATE_CREATE_INFO_NV,
            p_next: core::ptr::null(),
            flags: Default::default(),
            viewport_count: Default::default(),
            p_viewport_swizzles: core::ptr::null(),
            _marker: PhantomData,
        }
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
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
    #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct PipelineViewportSwizzleStateCreateFlagsNV: Flags {
    }
}
