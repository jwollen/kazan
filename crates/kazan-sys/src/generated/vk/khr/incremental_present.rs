#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{c_char, c_int, c_void};
use core::marker::PhantomData;
#[repr(C)]
pub struct PresentRegionsKHR<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub swapchain_count: u32,
    pub p_regions: *const PresentRegionKHR<'a>,
    pub _marker: PhantomData<&'a ()>,
}
#[repr(C)]
pub struct PresentRegionKHR<'a> {
    pub rectangle_count: u32,
    pub p_rectangles: *const RectLayerKHR,
    pub _marker: PhantomData<&'a ()>,
}
#[repr(C)]
pub struct RectLayerKHR {
    pub offset: Offset2D,
    pub extent: Extent2D,
    pub layer: u32,
}
