#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use std::ffi::{c_char, c_int, c_void};
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PresentRegionsKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub swapchain_count: u32,
    pub p_regions: *const PresentRegionKHR,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PresentRegionKHR {
    pub rectangle_count: u32,
    pub p_rectangles: *const RectLayerKHR,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct RectLayerKHR {
    pub offset: Offset2D,
    pub extent: Extent2D,
    pub layer: u32,
}
