#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{CStr, c_char, c_int, c_void};
use core::marker::PhantomData;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PipelineRasterizationStateRasterizationOrderAMD<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub rasterization_order: RasterizationOrderAMD,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for PipelineRasterizationStateRasterizationOrderAMD<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::PIPELINE_RASTERIZATION_STATE_RASTERIZATION_ORDER_AMD,
            p_next: core::ptr::null(),
            rasterization_order: Default::default(),
            _marker: PhantomData,
        }
    }
}
impl<'a> PipelineRasterizationStateRasterizationOrderAMD<'a> {
    pub fn rasterization_order(mut self, rasterization_order: RasterizationOrderAMD) -> Self {
        self.rasterization_order = rasterization_order;
        self
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct RasterizationOrderAMD(i32);
impl RasterizationOrderAMD {
    pub const STRICT_AMD: Self = Self(0);
    pub const RELAXED_AMD: Self = Self(1);
}
