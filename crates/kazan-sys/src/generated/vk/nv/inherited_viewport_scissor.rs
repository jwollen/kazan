#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{c_char, c_int, c_void};
use core::marker::PhantomData;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceInheritedViewportScissorFeaturesNV<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub inherited_viewport_scissor2_d: Bool32,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for PhysicalDeviceInheritedViewportScissorFeaturesNV<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::PHYSICAL_DEVICE_INHERITED_VIEWPORT_SCISSOR_FEATURES_NV,
            p_next: core::ptr::null_mut(),
            inherited_viewport_scissor2_d: Default::default(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct CommandBufferInheritanceViewportScissorInfoNV<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub viewport_scissor2_d: Bool32,
    pub viewport_depth_count: u32,
    pub p_viewport_depths: *const Viewport,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for CommandBufferInheritanceViewportScissorInfoNV<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::COMMAND_BUFFER_INHERITANCE_VIEWPORT_SCISSOR_INFO_NV,
            p_next: core::ptr::null(),
            viewport_scissor2_d: Default::default(),
            viewport_depth_count: Default::default(),
            p_viewport_depths: core::ptr::null(),
            _marker: PhantomData,
        }
    }
}
