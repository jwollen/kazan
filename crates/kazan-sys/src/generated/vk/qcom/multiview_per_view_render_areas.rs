#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{c_char, c_int, c_void};
use core::marker::PhantomData;
#[repr(C)]
pub struct PhysicalDeviceMultiviewPerViewRenderAreasFeaturesQCOM<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub multiview_per_view_render_areas: Bool32,
    pub _marker: PhantomData<&'a ()>,
}
#[repr(C)]
pub struct MultiviewPerViewRenderAreasRenderPassBeginInfoQCOM<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub per_view_render_area_count: u32,
    pub p_per_view_render_areas: *const Rect2D,
    pub _marker: PhantomData<&'a ()>,
}
