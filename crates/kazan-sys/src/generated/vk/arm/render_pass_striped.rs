#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{c_char, c_int, c_void};
use core::marker::PhantomData;
#[repr(C)]
pub struct PhysicalDeviceRenderPassStripedFeaturesARM<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub render_pass_striped: Bool32,
    pub _marker: PhantomData<&'a ()>,
}
#[repr(C)]
pub struct PhysicalDeviceRenderPassStripedPropertiesARM<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub render_pass_stripe_granularity: Extent2D,
    pub max_render_pass_stripes: u32,
    pub _marker: PhantomData<&'a ()>,
}
#[repr(C)]
pub struct RenderPassStripeInfoARM<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub stripe_area: Rect2D,
    pub _marker: PhantomData<&'a ()>,
}
#[repr(C)]
pub struct RenderPassStripeBeginInfoARM<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub stripe_info_count: u32,
    pub p_stripe_infos: *const RenderPassStripeInfoARM<'a>,
    pub _marker: PhantomData<&'a ()>,
}
#[repr(C)]
pub struct RenderPassStripeSubmitInfoARM<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub stripe_semaphore_info_count: u32,
    pub p_stripe_semaphore_infos: *const SemaphoreSubmitInfo<'a>,
    pub _marker: PhantomData<&'a ()>,
}
