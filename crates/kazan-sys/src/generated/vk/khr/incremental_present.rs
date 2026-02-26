#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{c_char, c_int, c_void};
use core::marker::PhantomData;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PresentRegionsKHR<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub swapchain_count: u32,
    pub p_regions: *const PresentRegionKHR<'a>,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for PresentRegionsKHR<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::PRESENT_REGIONS_KHR,
            p_next: core::ptr::null(),
            swapchain_count: Default::default(),
            p_regions: core::ptr::null(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PresentRegionKHR<'a> {
    pub rectangle_count: u32,
    pub p_rectangles: *const RectLayerKHR,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for PresentRegionKHR<'_> {
    fn default() -> Self {
        Self {
            rectangle_count: Default::default(),
            p_rectangles: core::ptr::null(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct RectLayerKHR {
    pub offset: Offset2D,
    pub extent: Extent2D,
    pub layer: u32,
}
