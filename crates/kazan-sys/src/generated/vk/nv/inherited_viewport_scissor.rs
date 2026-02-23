#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{c_char, c_int, c_void};
#[repr(C)]
pub struct PhysicalDeviceInheritedViewportScissorFeaturesNV {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub inherited_viewport_scissor2_d: Bool32,
}
#[repr(C)]
pub struct CommandBufferInheritanceViewportScissorInfoNV {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub viewport_scissor2_d: Bool32,
    pub viewport_depth_count: u32,
    pub p_viewport_depths: *const Viewport,
}
