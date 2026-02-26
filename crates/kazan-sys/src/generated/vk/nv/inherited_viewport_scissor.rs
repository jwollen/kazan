#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{c_char, c_int, c_void};
use core::marker::PhantomData;
#[repr(C)]
pub struct PhysicalDeviceInheritedViewportScissorFeaturesNV<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub inherited_viewport_scissor2_d: Bool32,
    pub _marker: PhantomData<&'a ()>,
}
#[repr(C)]
pub struct CommandBufferInheritanceViewportScissorInfoNV<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub viewport_scissor2_d: Bool32,
    pub viewport_depth_count: u32,
    pub p_viewport_depths: *const Viewport,
    pub _marker: PhantomData<&'a ()>,
}
