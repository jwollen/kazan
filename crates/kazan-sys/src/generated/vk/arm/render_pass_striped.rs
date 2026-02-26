#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{c_char, c_int, c_void};
use core::marker::PhantomData;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceRenderPassStripedFeaturesARM<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub render_pass_striped: Bool32,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for PhysicalDeviceRenderPassStripedFeaturesARM<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::PHYSICAL_DEVICE_RENDER_PASS_STRIPED_FEATURES_ARM,
            p_next: core::ptr::null_mut(),
            render_pass_striped: Default::default(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceRenderPassStripedPropertiesARM<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub render_pass_stripe_granularity: Extent2D,
    pub max_render_pass_stripes: u32,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for PhysicalDeviceRenderPassStripedPropertiesARM<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::PHYSICAL_DEVICE_RENDER_PASS_STRIPED_PROPERTIES_ARM,
            p_next: core::ptr::null_mut(),
            render_pass_stripe_granularity: Default::default(),
            max_render_pass_stripes: Default::default(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct RenderPassStripeInfoARM<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub stripe_area: Rect2D,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for RenderPassStripeInfoARM<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::RENDER_PASS_STRIPE_INFO_ARM,
            p_next: core::ptr::null(),
            stripe_area: Default::default(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct RenderPassStripeBeginInfoARM<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub stripe_info_count: u32,
    pub p_stripe_infos: *const RenderPassStripeInfoARM<'a>,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for RenderPassStripeBeginInfoARM<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::RENDER_PASS_STRIPE_BEGIN_INFO_ARM,
            p_next: core::ptr::null(),
            stripe_info_count: Default::default(),
            p_stripe_infos: core::ptr::null(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct RenderPassStripeSubmitInfoARM<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub stripe_semaphore_info_count: u32,
    pub p_stripe_semaphore_infos: *const SemaphoreSubmitInfo<'a>,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for RenderPassStripeSubmitInfoARM<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::RENDER_PASS_STRIPE_SUBMIT_INFO_ARM,
            p_next: core::ptr::null(),
            stripe_semaphore_info_count: Default::default(),
            p_stripe_semaphore_infos: core::ptr::null(),
            _marker: PhantomData,
        }
    }
}
