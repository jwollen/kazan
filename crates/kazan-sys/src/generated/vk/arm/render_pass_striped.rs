#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{c_char, c_int, c_void};
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceRenderPassStripedFeaturesARM {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub render_pass_striped: Bool32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceRenderPassStripedPropertiesARM {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub render_pass_stripe_granularity: Extent2D,
    pub max_render_pass_stripes: u32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct RenderPassStripeInfoARM {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub stripe_area: Rect2D,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct RenderPassStripeBeginInfoARM {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub stripe_info_count: u32,
    pub p_stripe_infos: *const RenderPassStripeInfoARM,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct RenderPassStripeSubmitInfoARM {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub stripe_semaphore_info_count: u32,
    pub p_stripe_semaphore_infos: *const SemaphoreSubmitInfo,
}
