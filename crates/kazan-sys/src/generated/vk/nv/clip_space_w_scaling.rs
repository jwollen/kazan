#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{CStr, c_char, c_int, c_void};
use core::marker::PhantomData;
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct ViewportWScalingNV {
    pub xcoeff: f32,
    pub ycoeff: f32,
}
impl ViewportWScalingNV {
    pub fn xcoeff(mut self, xcoeff: f32) -> Self {
        self.xcoeff = xcoeff;
        self
    }
    pub fn ycoeff(mut self, ycoeff: f32) -> Self {
        self.ycoeff = ycoeff;
        self
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PipelineViewportWScalingStateCreateInfoNV<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub viewport_w_scaling_enable: Bool32,
    pub viewport_count: u32,
    pub p_viewport_w_scalings: *const ViewportWScalingNV,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for PipelineViewportWScalingStateCreateInfoNV<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::PIPELINE_VIEWPORT_W_SCALING_STATE_CREATE_INFO_NV,
            p_next: core::ptr::null(),
            viewport_w_scaling_enable: Default::default(),
            viewport_count: Default::default(),
            p_viewport_w_scalings: core::ptr::null(),
            _marker: PhantomData,
        }
    }
}
impl<'a> PipelineViewportWScalingStateCreateInfoNV<'a> {
    pub fn viewport_w_scaling_enable(mut self, viewport_w_scaling_enable: Bool32) -> Self {
        self.viewport_w_scaling_enable = viewport_w_scaling_enable;
        self
    }
    pub fn viewport_w_scalings(mut self, viewport_w_scalings: &'a [ViewportWScalingNV]) -> Self {
        self.viewport_count = viewport_w_scalings.len().try_into().unwrap();
        self.p_viewport_w_scalings = viewport_w_scalings.as_ptr();
        self
    }
}
pub type PFN_vkCmdSetViewportWScalingNV = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    first_viewport: u32,
    viewport_count: u32,
    p_viewport_w_scalings: *const ViewportWScalingNV,
);
